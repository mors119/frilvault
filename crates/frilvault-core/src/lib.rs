pub mod constants;
pub mod error;
pub mod note;
pub mod parser;
pub mod storage;
pub mod workspace;

pub use error::FrilVaultError;
pub use note::{AddNoteInput, Note, NoteFile, NoteService};
pub use storage::YamlNoteRepository;
pub use workspace::{PathResolver, Workspace};

pub type FrilVaultResult<T> = Result<T, FrilVaultError>;

#[cfg(test)]
mod tests;
