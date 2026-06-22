//! In-memory cache for note files.
//!
//! Used by long-running runtimes such as
//! VSCode extensions to reduce filesystem access.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::note::NoteFile;

#[derive(Debug, Default, Clone)]
pub struct NoteCache {
    files: HashMap<PathBuf, NoteFile>,
}

impl NoteCache {
    pub fn get(&self, path: &Path) -> Option<&NoteFile> {
        self.files.get(path)
    }

    pub fn insert(&mut self, path: PathBuf, note_file: NoteFile) {
        self.files.insert(path, note_file);
    }

    pub fn invalidate(&mut self, path: &Path) {
        self.files.remove(path);
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }

    pub fn contains(&self, source_file: &Path) -> bool {
        self.files.contains_key(source_file)
    }
}
