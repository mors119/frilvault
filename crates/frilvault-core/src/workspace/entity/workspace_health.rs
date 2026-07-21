use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct WorkspaceHealth {
    pub missing_source_files: Vec<String>,
}
