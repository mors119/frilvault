use crate::{
    FrilVaultError, FrilVaultResult,
    constants::{NOTE_FILE_EXTENSION, VAULT_DIR_NAME},
};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PathResolver {
    workspace_root: PathBuf,
}

impl PathResolver {
    pub fn new(workspace_root: impl Into<PathBuf>) -> Self {
        Self {
            workspace_root: workspace_root.into(),
        }
    }

    pub fn note_file_name(source_file: impl AsRef<Path>) -> String {
        format!("{}.{}", source_file.as_ref().display(), NOTE_FILE_EXTENSION)
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    // Convert the relative path of the source file to the path '.vault/{source_file}.yml'
    pub fn resolve_note_path(&self, source_file: impl AsRef<Path>) -> PathBuf {
        let note_file_name = Self::note_file_name(source_file);

        self.workspace_root
            .join(VAULT_DIR_NAME)
            .join(note_file_name)
    }

    // Converting an Absolute Path to a Relative Path
    pub fn to_workspace_relative(&self, source_file: impl AsRef<Path>) -> FrilVaultResult<PathBuf> {
        let source_file = source_file.as_ref();

        let relative = source_file
            .strip_prefix(&self.workspace_root)
            .map_err(|_| FrilVaultError::SourcePathOutsideWorkspace)?;

        Ok(relative.to_path_buf())
    }
}
