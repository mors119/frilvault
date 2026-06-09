use crate::{
    FrilVaultError, FrilVaultResult,
    constants::{
        INDEX_DIR_NAME, NOTE_FILE_EXTENSION, NOTES_DIR_NAME, VAULT_DIR_NAME, WORKSPACE_FILE_NAME,
    },
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

    pub fn vault_root(&self) -> PathBuf {
        self.workspace_root.join(VAULT_DIR_NAME)
    }

    pub fn notes_root(&self) -> PathBuf {
        self.vault_root().join(NOTES_DIR_NAME)
    }

    pub fn workspace_metadata_path(&self) -> PathBuf {
        self.vault_root().join(WORKSPACE_FILE_NAME)
    }

    pub fn workspace_index_path(&self) -> PathBuf {
        self.vault_root().join(INDEX_DIR_NAME).join("workspace.yml")
    }

    // Convert the relative path of the source file to the path '.vault/notes/{source_file}.yml'
    pub fn resolve_note_path(&self, source_file: impl AsRef<Path>) -> PathBuf {
        self.notes_root().join(Self::note_file_name(source_file))
    }

    pub fn source_file_from_note_path(
        &self,
        note_path: impl AsRef<Path>,
    ) -> FrilVaultResult<PathBuf> {
        let note_path = note_path.as_ref();

        let relative = note_path
            .strip_prefix(self.notes_root())
            .map_err(|_| FrilVaultError::SourcePathOutsideWorkspace)?;

        let file_name = relative.to_string_lossy();

        let suffix = format!(".{}", NOTE_FILE_EXTENSION);

        let source_file = file_name
            .strip_suffix(&suffix)
            .ok_or(FrilVaultError::InvalidNoteFilePath)?
            .to_string();

        Ok(PathBuf::from(source_file))
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
