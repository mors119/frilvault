use std::fs;
use std::path::Path;

use uuid::Uuid;

use crate::{
    FrilVaultError, FrilVaultResult,
    constants::{ALLOWED_IMAGE_EXTENSIONS, MAX_IMAGE_BYTES},
    note::NoteAttachment,
    workspace::PathResolver,
};

#[derive(Debug, Clone)]
pub struct AttachmentRepository {
    path_resolver: PathResolver,
}

impl AttachmentRepository {
    pub fn new(path_resolver: PathResolver) -> Self {
        Self { path_resolver }
    }

    pub fn store(&self, note_id: Uuid, source_path: &Path) -> FrilVaultResult<NoteAttachment> {
        let extension = image_extension(source_path)?;
        let metadata = fs::metadata(source_path)?;

        if metadata.len() as usize > MAX_IMAGE_BYTES {
            return Err(FrilVaultError::ImageTooLarge {
                max_bytes: MAX_IMAGE_BYTES,
            });
        }

        let attachment_id = Uuid::new_v4();
        let dest_dir = self.path_resolver.note_images_dir(note_id);
        fs::create_dir_all(&dest_dir)?;

        let dest = self
            .path_resolver
            .resolve_attachment_path(note_id, attachment_id, extension);
        fs::copy(source_path, dest)?;

        Ok(NoteAttachment {
            id: attachment_id,
            filename: source_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("image")
                .to_string(),
            mime_type: mime_type_for_extension(extension),
            extension: extension.to_string(),
        })
    }

    pub fn remove(&self, note_id: Uuid, attachment: &NoteAttachment) -> FrilVaultResult<()> {
        let path = self.path_resolver.resolve_attachment_path(
            note_id,
            attachment.id,
            &attachment.extension,
        );

        if path.exists() {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    pub fn remove_all_for_note(&self, note_id: Uuid) -> FrilVaultResult<()> {
        let directory = self.path_resolver.note_images_dir(note_id);

        if directory.exists() {
            fs::remove_dir_all(directory)?;
        }

        Ok(())
    }
}

fn image_extension(path: &Path) -> FrilVaultResult<&'static str> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| FrilVaultError::InvalidImageType("missing file extension".to_string()))?;

    ALLOWED_IMAGE_EXTENSIONS
        .iter()
        .find(|allowed| **allowed == extension)
        .copied()
        .ok_or(FrilVaultError::InvalidImageType(extension))
}

fn mime_type_for_extension(extension: &str) -> String {
    match extension {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    }
    .to_string()
}
