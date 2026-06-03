use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum FrilVaultError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yml::Error),

    #[error("source path is outside workspace")]
    SourcePathOutsideWorkspace,

    #[error("note not found: {0}")]
    NoteNotFound(Uuid),
}
