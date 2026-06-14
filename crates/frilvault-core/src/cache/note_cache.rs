use std::collections::HashMap;
use std::path::PathBuf;

use crate::NoteFile;

#[derive(Debug, Default)]
pub struct NoteCache {
    files: HashMap<PathBuf, NoteFile>,
}

impl NoteCache {
    pub fn get(&self, path: &PathBuf) -> Option<&NoteFile> {
        self.files.get(path)
    }

    pub fn insert(&mut self, path: PathBuf, note_file: NoteFile) {
        self.files.insert(path, note_file);
    }

    pub fn invalidate(&mut self, path: &PathBuf) {
        self.files.remove(path);
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }
}
