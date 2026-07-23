use std::path::PathBuf;

/// Unified query for listing and searching notes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NoteQuery {
    /// Restrict results to a single source file.
    pub source_file: Option<PathBuf>,

    /// Case-insensitive substring match on note content and symbol names.
    pub keyword: Option<String>,

    /// Case-insensitive exact tag match.
    pub tag: Option<String>,
}
