use serde::{Deserialize, Serialize};

use crate::workspace::IndexedFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceIndex {
    pub version: u32,

    pub files: Vec<IndexedFile>,
}

impl Default for WorkspaceIndex {
    fn default() -> Self {
        Self {
            version: 1,
            files: Vec::new(),
        }
    }
}

impl WorkspaceIndex {
    pub fn upsert_file(&mut self, entry: IndexedFile) {
        if let Some(existing) = self
            .files
            .iter_mut()
            .find(|file| file.source_file == entry.source_file)
        {
            *existing = entry;
            return;
        }

        self.files.push(entry);
    }

    pub fn remove_file(&mut self, source_file: &str) -> bool {
        let before = self.files.len();
        self.files.retain(|file| file.source_file != source_file);
        self.files.len() < before
    }

    pub fn move_file(&mut self, from: &str, to: &str) -> bool {
        let Some(file) = self.files.iter_mut().find(|file| file.source_file == from) else {
            return false;
        };

        file.source_file = to.to_string();

        true
    }
}
