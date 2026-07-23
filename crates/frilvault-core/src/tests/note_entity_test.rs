use crate::{
    AddNoteRequest,
    note::{LineAnchor, Note, NoteAnchor},
};

#[test]
fn create_note_from_input() {
    let input = AddNoteRequest {
        source_file: "src/main.rs".into(),
        anchor: NoteAnchor::Line(LineAnchor {
            line: 10,
            column: 5,
        }),
        content: "test note".to_string(),
        tags: None,
    };

    let note = Note::new(input);

    assert_eq!(note.content, "test note");

    match note.anchor {
        NoteAnchor::Line(anchor) => {
            assert_eq!(anchor.line, 10);
            assert_eq!(anchor.column, 5);
        }
        _ => panic!("expected line anchor"),
    }
}

#[test]
fn create_note_generates_uuid() {
    let input = AddNoteRequest {
        source_file: "src/main.rs".into(),
        anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
        content: "uuid test".to_string(),
        tags: None,
    };

    let note = Note::new(input);

    assert_ne!(note.id.to_string(), "");
}

#[test]
fn create_note_stores_tags() {
    let input = AddNoteRequest {
        source_file: "src/main.rs".into(),
        anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
        content: "tagged note".to_string(),
        tags: Some(vec!["bug".to_string(), "architecture".to_string()]),
    };

    let note = Note::new(input);

    assert_eq!(
        note.tags,
        vec!["bug".to_string(), "architecture".to_string()]
    );
}
