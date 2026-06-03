use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrilVaultError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yml::Error),

    #[error("source file path is outside workspace root")]
    SourcePathOutsideWorkspace,
}
