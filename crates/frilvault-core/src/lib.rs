mod app;
mod constants;
mod error;
mod note;
mod parser;
mod runtime;
mod workspace;

pub use app::FrilVault;

pub use error::{FrilVaultError, FrilVaultResult};
pub use note::{AddNoteRequest, LineAnchor, Note, NoteAnchor, NoteView, SymbolAnchor, SymbolKind};
pub use workspace::{
    FileMove, RepairSuggestion, WorkspaceHealth, WorkspaceService, WorkspaceStats,
};

#[cfg(test)]
mod tests;
