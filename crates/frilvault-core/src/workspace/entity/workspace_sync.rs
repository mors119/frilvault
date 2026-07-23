use serde::Serialize;

/// Result of synchronizing external workspace changes.
#[derive(Debug, Serialize)]
pub struct SyncResult {
    pub notes_synced: bool,
    pub repairs_applied: usize,
}
