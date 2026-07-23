//! In-memory cache for note files.
//!
//! Used by long-running runtimes such as VS Code extensions to reduce filesystem access.
//!
//! VS Code extension 같은 장기 실행 runtime에서 filesystem 접근을 줄이기 위한
//! in-memory note file cache입니다.

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
