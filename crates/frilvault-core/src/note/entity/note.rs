use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AddNoteRequest, note::NoteAnchor};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Note {
    /// id
    pub id: Uuid,

    /// anchor: the position where the note is anchored. it can be either line-based or symbol-based.
    pub anchor: NoteAnchor,

    /// content: the content of the note.
    pub content: String,

    /// created_at: the time when the note was created.
    pub created_at: DateTime<Utc>,

    /// updated_at: the time when the note was last updated. initially same as created_at.
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(input: AddNoteRequest) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            anchor: input.anchor,
            content: input.content,
            created_at: now,
            updated_at: now,
        }
    }
}
