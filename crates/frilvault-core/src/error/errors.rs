use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum FrilVaultError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    JSON(#[from] serde_json::Error),

    #[error("source path is outside workspace")]
    SourcePathOutsideWorkspace,

    #[error("note not found: {0}")]
    NoteNotFound(Uuid),

    #[error("invalid note file path")]
    InvalidNoteFilePath,

    #[error("attachment not found: {0}")]
    AttachmentNotFound(Uuid),

    #[error("invalid image type: {0}")]
    InvalidImageType(String),

    #[error("image exceeds maximum size of {max_bytes} bytes")]
    ImageTooLarge { max_bytes: usize },
}

pub type FrilVaultResult<T> = Result<T, FrilVaultError>;
