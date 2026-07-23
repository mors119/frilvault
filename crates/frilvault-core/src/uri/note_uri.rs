use std::path::{Component, Path, PathBuf};

use uuid::Uuid;

use crate::{FrilVaultError, FrilVaultResult};

pub const NOTE_URI_SCHEME: &str = "frilvault";
pub const NOTE_URI_VERSION: &str = "v1";
const NOTE_URI_PREFIX: &str = "frilvault://note/";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedNoteUri {
    pub note_id: Uuid,
    pub workspace: PathBuf,
}

impl ParsedNoteUri {
    pub fn parse(uri: &str) -> FrilVaultResult<Self> {
        let uri = uri.trim();

        if !uri.starts_with(NOTE_URI_PREFIX) {
            return Err(FrilVaultError::MalformedNoteUri(
                "uri must start with frilvault://note/".to_string(),
            ));
        }

        let remainder = &uri[NOTE_URI_PREFIX.len()..];
        let (path_part, query) = remainder.split_once('?').ok_or_else(|| {
            FrilVaultError::MalformedNoteUri("uri must include workspace query".to_string())
        })?;

        let workspace = parse_workspace_query(query)?;
        let note_id = parse_note_id_from_path(path_part)?;

        Ok(Self { note_id, workspace })
    }

    pub fn serialize(note_id: Uuid, workspace: &Path) -> FrilVaultResult<String> {
        let workspace = validate_workspace_path(workspace)?;

        Ok(format!(
            "{NOTE_URI_PREFIX}{NOTE_URI_VERSION}/{note_id}?workspace={}",
            percent_encode(&workspace.to_string_lossy())
        ))
    }
}

pub fn parse_workspace_query(query: &str) -> FrilVaultResult<PathBuf> {
    let mut workspace: Option<String> = None;

    for pair in query.split('&') {
        let Some((key, value)) = pair.split_once('=') else {
            return Err(FrilVaultError::MalformedNoteUri(format!(
                "invalid query segment: {pair}"
            )));
        };

        if key != "workspace" {
            return Err(FrilVaultError::MalformedNoteUri(format!(
                "unsupported query parameter: {key}"
            )));
        }

        if workspace.is_some() {
            return Err(FrilVaultError::MalformedNoteUri(
                "duplicate workspace parameter".to_string(),
            ));
        }

        workspace = Some(percent_decode(value)?);
    }

    let workspace = workspace.ok_or_else(|| {
        FrilVaultError::MalformedNoteUri("missing workspace parameter".to_string())
    })?;

    validate_workspace_path(Path::new(&workspace))
}

fn parse_note_id_from_path(path_part: &str) -> FrilVaultResult<Uuid> {
    let note_segment = path_part
        .strip_prefix(&format!("{NOTE_URI_VERSION}/"))
        .unwrap_or(path_part);

    if note_segment.is_empty() || note_segment.contains('/') {
        return Err(FrilVaultError::MalformedNoteUri(
            "note id path is invalid".to_string(),
        ));
    }

    Uuid::parse_str(note_segment)
        .map_err(|error| FrilVaultError::MalformedNoteUri(format!("invalid note id: {error}")))
}

pub fn validate_workspace_path(workspace: &Path) -> FrilVaultResult<PathBuf> {
    if !workspace.is_absolute() {
        return Err(FrilVaultError::MalformedNoteUri(
            "workspace path must be absolute".to_string(),
        ));
    }

    for component in workspace.components() {
        if matches!(component, Component::ParentDir) {
            return Err(FrilVaultError::MalformedNoteUri(
                "workspace path must not contain parent directory segments".to_string(),
            ));
        }
    }

    Ok(workspace.to_path_buf())
}

fn percent_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());

    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                encoded.push(byte as char);
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }

    encoded
}

fn percent_decode(input: &str) -> FrilVaultResult<String> {
    let mut decoded = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return Err(FrilVaultError::MalformedNoteUri(
                    "invalid percent encoding".to_string(),
                ));
            }

            let hex = &input[index + 1..index + 3];
            let value = u8::from_str_radix(hex, 16).map_err(|_| {
                FrilVaultError::MalformedNoteUri("invalid percent encoding".to_string())
            })?;
            decoded.push(value);
            index += 3;
            continue;
        }

        decoded.push(bytes[index]);
        index += 1;
    }

    String::from_utf8(decoded).map_err(|_| {
        FrilVaultError::MalformedNoteUri("workspace path is not valid utf-8".to_string())
    })
}
