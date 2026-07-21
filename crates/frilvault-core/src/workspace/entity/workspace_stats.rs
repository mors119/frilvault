use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct WorkspaceStats {
    pub file_count: usize,

    pub total_notes: usize,

    pub existing_files: usize,

    pub missing_files: usize,

    pub line_notes: usize,

    pub symbol_notes: usize,
}
