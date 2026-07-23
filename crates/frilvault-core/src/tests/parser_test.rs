use crate::{LineAnchor, Note, NoteAnchor, note::NoteFile, parser::JsonParser};

use chrono::Utc;
use uuid::Uuid;

#[test]
fn serialize_note_file() {
    let parser = JsonParser;

    let note = Note {
        id: Uuid::new_v4(),
        anchor: NoteAnchor::Line(LineAnchor {
            line: 10,
            column: 5,
        }),
        content: "hello".to_string(),
        tags: vec!["bug".to_string(), "architecture".to_string()],
        attachments: vec![],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let note_file = NoteFile { notes: vec![note] };

    let json = parser.serialize(&note_file).unwrap();

    assert!(json.contains("hello"));
    assert!(json.contains("bug"));
    assert!(json.contains("architecture"));
}

#[test]
fn deserialize_note_file() {
    let parser = JsonParser;

    let json = r#"
{
  "notes": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "anchor": {
        "type": "Line",
        "line": 10,
        "column": 5
      },
      "content": "hello",
      "tags": ["bug", "architecture"],
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-01-01T00:00:00Z"
    }
  ]
}
"#;

    let note_file = parser.deserialize(json).unwrap();

    assert_eq!(note_file.notes.len(), 1);

    assert_eq!(note_file.notes[0].content, "hello");
    assert_eq!(
        note_file.notes[0].tags,
        vec!["bug".to_string(), "architecture".to_string()]
    );
}

#[test]
fn deserialize_note_file_without_tags_defaults_to_empty() {
    let parser = JsonParser;

    let json = r#"
{
  "notes": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "anchor": {
        "type": "Line",
        "line": 10,
        "column": 5
      },
      "content": "legacy note",
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-01-01T00:00:00Z"
    }
  ]
}
"#;

    let note_file = parser.deserialize(json).unwrap();

    assert!(note_file.notes[0].tags.is_empty());
}

#[test]
fn deserialize_note_file_without_attachments_defaults_to_empty() {
    let parser = JsonParser;

    let json = r#"
{
  "notes": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "anchor": {
        "type": "Line",
        "line": 10,
        "column": 5
      },
      "content": "legacy note",
      "tags": [],
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-01-01T00:00:00Z"
    }
  ]
}
"#;

    let note_file = parser.deserialize(json).unwrap();

    assert!(note_file.notes[0].attachments.is_empty());
}
