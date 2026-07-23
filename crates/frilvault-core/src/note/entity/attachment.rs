use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Image file attached to a note, stored under `.vault/images/{note_id}/`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteAttachment {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub extension: String,
}
