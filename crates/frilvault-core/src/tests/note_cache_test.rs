use std::path::Path;

use crate::{AddNoteRequest, LineAnchor, Note, NoteAnchor, note::NoteFile, runtime::NoteCache};

fn sample_note_file() -> NoteFile {
    NoteFile {
        notes: vec![Note::new(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "cached note".to_string(),
        })],
    }
}

#[test]
fn get_returns_none_for_missing_path() {
    let cache = NoteCache::default();

    assert!(cache.get(Path::new("src/missing.rs")).is_none());
}

#[test]
fn insert_and_get_return_cached_note_file() {
    let mut cache = NoteCache::default();
    let note_file = sample_note_file();

    cache.insert("src/main.rs".into(), note_file.clone());

    let cached = cache.get(Path::new("src/main.rs")).unwrap();
    assert_eq!(cached, &note_file);
}

#[test]
fn invalidate_removes_cached_entry() {
    let mut cache = NoteCache::default();
    cache.insert("src/main.rs".into(), sample_note_file());

    cache.invalidate(Path::new("src/main.rs"));

    assert!(cache.get(Path::new("src/main.rs")).is_none());
    assert!(!cache.contains(Path::new("src/main.rs")));
}

#[test]
fn clear_removes_all_cached_entries() {
    let mut cache = NoteCache::default();
    cache.insert("src/a.rs".into(), sample_note_file());
    cache.insert("src/b.rs".into(), sample_note_file());

    cache.clear();

    assert!(!cache.contains(Path::new("src/a.rs")));
    assert!(!cache.contains(Path::new("src/b.rs")));
}
