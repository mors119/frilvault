mod app;
mod constants;
mod error;
mod note;
mod parser;
mod runtime;
mod storage;
mod workspace;

pub use app::FrilVault;

pub use error::{FrilVaultError, FrilVaultResult};
pub use note::{AddNoteRequest, LineAnchor, Note, NoteAnchor, NoteView, SymbolAnchor, SymbolKind};
pub use workspace::{RepairSuggestion, WorkspaceHealth, WorkspaceStats};

#[cfg(test)]
mod tests;
