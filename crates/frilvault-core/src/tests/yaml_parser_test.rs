use crate::{
    note::{LineAnchor, Note, NoteAnchor, NoteFile},
    parser::{NoteParser, YamlParser},
};

use chrono::Utc;
use uuid::Uuid;

#[test]
fn serialize_note_file() {
    let parser = YamlParser;

    let note = Note {
        id: Uuid::new_v4(),
        source_file: "src/main.rs".into(),
        anchor: NoteAnchor::Line(LineAnchor {
            line: 10,
            column: 5,
        }),
        content: "hello".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let note_file = NoteFile { notes: vec![note] };

    let yaml = parser.serialize(&note_file).unwrap();

    assert!(yaml.contains("hello"));
}

#[test]
fn deserialize_note_file() {
    let parser = YamlParser;

    let yaml = r#"
notes:
  - id: "550e8400-e29b-41d4-a716-446655440000"
    source_file: src/main.rs
    anchor:
      type: Line
      line: 10
      column: 5
    content: hello
    created_at: 2025-01-01T00:00:00Z
    updated_at: 2025-01-01T00:00:00Z
"#;

    let note_file = parser.deserialize(yaml).unwrap();

    assert_eq!(note_file.notes.len(), 1);

    assert_eq!(note_file.notes[0].content, "hello");
}
