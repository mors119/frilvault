use frilvault_core::{Note, NoteAnchor};

// pub enum OutputFormat {
//     Text,
//     Json,
// }

pub fn print_note(note: &Note) {
    println!("────────────────────────────────");

    println!("ID: {}", note.id);

    match &note.anchor {
        NoteAnchor::Line(anchor) => {
            println!("Location: {}:{}", anchor.line, anchor.column);
        }

        NoteAnchor::Symbol(anchor) => {
            println!("Symbol: {}", anchor.name);
        }
    }

    println!("File: {}", note.source_file.display());

    println!();

    println!("{}", note.content);

    println!();
}

pub fn print_note_count(count: usize) {
    println!("Found {} notes", count);

    println!();
}
