use serde::{Deserialize, Serialize};

use crate::workspace::WorkspaceIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSnapshot {
    pub index: WorkspaceIndex,
}
