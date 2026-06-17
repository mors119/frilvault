use serde::{Deserialize, Serialize};

use crate::WorkspaceIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSnapshot {
    pub index: WorkspaceIndex,
}
