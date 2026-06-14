use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use frilvault_core::{
    AddNoteInput, FrilVaultError, LineAnchor, Note, NoteAnchor, NoteService, NoteView,
    PathResolver, RepairSuggestion, SymbolAnchor, SymbolKind, WorkspaceHealth,
    WorkspaceIndexRepository, WorkspaceRepository, WorkspaceService, WorkspaceStats,
    YamlNoteRepository,
};
use napi::bindgen_prelude::Error;
use napi_derive::napi;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct SerializableNote {
    id: String,
    anchor: SerializableAnchor,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum SerializableAnchor {
    Line {
        line: u32,
        column: u32,
    },
    Symbol {
        name: String,
        kind: String,
        signature: Option<String>,
        line_hint: Option<u32>,
    },
}

#[derive(Serialize)]
struct SerializableNoteView {
    source_file: String,
    note: SerializableNote,
}

#[derive(Serialize)]
struct SerializableWorkspaceStats {
    file_count: usize,
    total_notes: usize,
    existing_files: usize,
    missing_files: usize,
    line_notes: usize,
    symbol_notes: usize,
}

#[derive(Serialize)]
struct SerializableWorkspaceHealth {
    missing_source_files: Vec<String>,
}

#[derive(Serialize)]
struct SerializableRepairSuggestion {
    missing_file: String,
    candidates: Vec<String>,
}

#[derive(Serialize)]
struct SerializableMutationResult {
    note: Option<SerializableNote>,
}

fn napi_error(error: FrilVaultError) -> Error {
    Error::from_reason(error.to_string())
}

fn parse_uuid(note_id: &str) -> napi::Result<Uuid> {
    Uuid::parse_str(note_id).map_err(|error| Error::from_reason(error.to_string()))
}

fn note_service(workspace_root: &str) -> napi::Result<NoteService<YamlNoteRepository>> {
    let resolver = PathResolver::new(workspace_root);
    let workspace_repository = WorkspaceRepository::new(resolver.clone());
    workspace_repository
        .create_if_missing()
        .map_err(napi_error)?;

    let index_repository = WorkspaceIndexRepository::new(resolver.clone());
    index_repository.create_if_missing().map_err(napi_error)?;

    Ok(NoteService::new(YamlNoteRepository::new(resolver)))
}

fn workspace_service(workspace_root: &str) -> napi::Result<WorkspaceService> {
    let resolver = PathResolver::new(workspace_root);
    let workspace_repository = WorkspaceRepository::new(resolver.clone());
    workspace_repository
        .create_if_missing()
        .map_err(napi_error)?;

    let index_repository = WorkspaceIndexRepository::new(resolver.clone());
    index_repository.create_if_missing().map_err(napi_error)?;

    let note_repository = YamlNoteRepository::new(resolver);

    Ok(WorkspaceService::new(note_repository, index_repository))
}

fn normalize_source_file(workspace_root: &str, source_file: &str) -> napi::Result<PathBuf> {
    let resolver = PathResolver::new(workspace_root);
    let path = PathBuf::from(source_file);

    if path.is_absolute() {
        return resolver.to_workspace_relative(path).map_err(napi_error);
    }

    if Path::new(source_file).starts_with(resolver.workspace_root()) {
        return resolver
            .to_workspace_relative(source_file)
            .map_err(napi_error);
    }

    Ok(path)
}

fn serialize_json<T: Serialize>(value: &T) -> napi::Result<String> {
    serde_json::to_string(value).map_err(|error| Error::from_reason(error.to_string()))
}

fn serialize_note(note: Note) -> SerializableNote {
    SerializableNote {
        id: note.id.to_string(),
        anchor: serialize_anchor(note.anchor),
        content: note.content,
        created_at: note.created_at,
        updated_at: note.updated_at,
    }
}

fn serialize_anchor(anchor: NoteAnchor) -> SerializableAnchor {
    match anchor {
        NoteAnchor::Line(LineAnchor { line, column }) => SerializableAnchor::Line { line, column },
        NoteAnchor::Symbol(SymbolAnchor {
            name,
            kind,
            signature,
            line_hint,
        }) => SerializableAnchor::Symbol {
            name,
            kind: serialize_symbol_kind(kind),
            signature,
            line_hint,
        },
    }
}

fn serialize_note_view(view: NoteView) -> SerializableNoteView {
    SerializableNoteView {
        source_file: view.source_file.to_string_lossy().to_string(),
        note: serialize_note(view.note),
    }
}

fn serialize_symbol_kind(kind: SymbolKind) -> String {
    match kind {
        SymbolKind::Function => "Function",
        SymbolKind::Struct => "Struct",
        SymbolKind::Enum => "Enum",
        SymbolKind::Trait => "Trait",
        SymbolKind::Impl => "Impl",
        SymbolKind::Method => "Method",
        SymbolKind::Unknown => "Unknown",
    }
    .to_string()
}

fn serialize_stats(stats: WorkspaceStats) -> SerializableWorkspaceStats {
    SerializableWorkspaceStats {
        file_count: stats.file_count,
        total_notes: stats.total_notes,
        existing_files: stats.existing_files,
        missing_files: stats.missing_files,
        line_notes: stats.line_notes,
        symbol_notes: stats.symbol_notes,
    }
}

fn serialize_health(health: WorkspaceHealth) -> SerializableWorkspaceHealth {
    SerializableWorkspaceHealth {
        missing_source_files: health.missing_source_files,
    }
}

fn serialize_repair_suggestion(suggestion: RepairSuggestion) -> SerializableRepairSuggestion {
    SerializableRepairSuggestion {
        missing_file: suggestion.missing_file,
        candidates: suggestion.candidates,
    }
}

#[napi]
pub fn add_line_note(
    workspace_root: String,
    source_file: String,
    line: u32,
    column: u32,
    content: String,
) -> napi::Result<String> {
    let service = note_service(&workspace_root)?;
    let source_file = normalize_source_file(&workspace_root, &source_file)?;

    let note = service
        .add_note(AddNoteInput {
            source_file,
            anchor: NoteAnchor::Line(LineAnchor { line, column }),
            content,
        })
        .map_err(napi_error)?;

    serialize_json(&SerializableMutationResult {
        note: Some(serialize_note(note)),
    })
}

#[napi]
pub fn list_notes(workspace_root: String, source_file: String) -> napi::Result<String> {
    let service = note_service(&workspace_root)?;
    let source_file = normalize_source_file(&workspace_root, &source_file)?;

    let views = service.list_notes(source_file).map_err(napi_error)?;
    let serialized: Vec<_> = views.into_iter().map(serialize_note_view).collect();

    serialize_json(&serialized)
}

#[napi]
pub fn update_note(
    workspace_root: String,
    source_file: String,
    note_id: String,
    content: String,
) -> napi::Result<String> {
    let service = note_service(&workspace_root)?;
    let source_file = normalize_source_file(&workspace_root, &source_file)?;
    let note_id = parse_uuid(&note_id)?;

    service
        .update_note(&source_file, note_id, content)
        .map_err(napi_error)?;

    let note = service
        .list_notes(source_file)
        .map_err(napi_error)?
        .into_iter()
        .find(|view| view.note.id == note_id)
        .map(|view| serialize_note(view.note));

    serialize_json(&SerializableMutationResult { note })
}

#[napi]
pub fn delete_note(
    workspace_root: String,
    source_file: String,
    note_id: String,
) -> napi::Result<()> {
    let service = note_service(&workspace_root)?;
    let source_file = normalize_source_file(&workspace_root, &source_file)?;
    let note_id = parse_uuid(&note_id)?;

    service
        .delete_note(source_file, note_id)
        .map_err(napi_error)
}

#[napi]
pub fn search_notes(workspace_root: String, keyword: String) -> napi::Result<String> {
    let service = note_service(&workspace_root)?;
    let views = service.search_notes(&keyword).map_err(napi_error)?;
    let serialized: Vec<_> = views.into_iter().map(serialize_note_view).collect();

    serialize_json(&serialized)
}

#[napi]
pub fn workspace_stats(workspace_root: String) -> napi::Result<String> {
    let service = workspace_service(&workspace_root)?;
    let stats = service.stats().map_err(napi_error)?;

    serialize_json(&serialize_stats(stats))
}

#[napi]
pub fn workspace_health(workspace_root: String) -> napi::Result<String> {
    let service = workspace_service(&workspace_root)?;
    let health = service.health_check().map_err(napi_error)?;

    serialize_json(&serialize_health(health))
}

#[napi]
pub fn repair_suggestions(workspace_root: String) -> napi::Result<String> {
    let service = workspace_service(&workspace_root)?;
    let suggestions = service.repair_suggestions().map_err(napi_error)?;
    let serialized: Vec<_> = suggestions
        .into_iter()
        .map(serialize_repair_suggestion)
        .collect();

    serialize_json(&serialized)
}

#[napi]
pub fn apply_repairs(workspace_root: String) -> napi::Result<u32> {
    let service = workspace_service(&workspace_root)?;
    let repaired = service.apply_repairs().map_err(napi_error)?;

    Ok(repaired as u32)
}
