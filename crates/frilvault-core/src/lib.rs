mod app;
mod constants;
mod error;
mod note;
mod parser;
mod runtime;
mod symbol;
mod workspace;

pub use app::FrilVault;

pub use error::{FrilVaultError, FrilVaultResult};
pub use note::{
    AddNoteRequest, LineAnchor, Note, NoteAnchor, NoteQuery, NoteView, SymbolAnchor, SymbolKind,
};
pub use symbol::{ResolvedSymbol, SymbolResolver, symbol_marker};
pub use workspace::{
    ExplorerGroup, ExplorerNode, FileMove, RepairSuggestion, SyncResult, WorkspaceExplorer,
    WorkspaceHealth, WorkspaceService, WorkspaceStats,
};

#[cfg(test)]
mod tests;
