#[derive(Debug, Default)]
pub struct WorkspaceHealth {
    pub missing_source_files: Vec<String>,
}
