use crate::workspace::{WorkspaceIndex, WorkspaceSnapshot};

#[derive(Default)]
pub struct SnapshotManager {
    previous: Option<WorkspaceSnapshot>,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self { previous: None }
    }

    pub fn update(&mut self, index: WorkspaceIndex) {
        let snapshot = WorkspaceSnapshot { index };

        self.previous = Some(snapshot);
    }

    pub fn previous(&self) -> Option<&WorkspaceIndex> {
        self.previous.as_ref().map(|s| &s.index)
    }
}
