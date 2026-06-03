use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

// TODO: Regex parser
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum NoteAnchor {
    Line(LineAnchor),
    Symbol(SymbolAnchor),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LineAnchor {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolAnchor {
    pub name: String,
    pub kind: SymbolKind,
    pub signature: Option<String>,
    pub line_hint: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Method,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Note {
    /// id
    pub id: Uuid,

    /// source_file
    pub source_file: PathBuf,

    /// anchor: the position where the note is anchored. it can be either line-based or symbol-based.
    pub anchor: NoteAnchor,

    /// content: the content of the note.
    pub content: String,

    /// created_at: the time when the note was created.
    pub created_at: DateTime<Utc>,

    /// updated_at: the time when the note was last updated. initially same as created_at.
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AddNoteInput {
    /// source_file: the path to the source file where the note is located.
    pub source_file: PathBuf,

    /// anchor: the position where the note is anchored. it can be either line-based or symbol-based.
    pub anchor: NoteAnchor,

    /// content: the content of the note to be saved.
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteFile {
    pub notes: Vec<Note>,
}

impl Note {
    pub fn new(input: AddNoteInput) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            source_file: input.source_file,
            anchor: input.anchor,
            content: input.content,
            created_at: now,
            updated_at: now,
        }
    }
}
