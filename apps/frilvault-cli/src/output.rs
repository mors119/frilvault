use anyhow::Result;
use frilvault_core::{NoteAnchor, NoteView};
use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Text,
    Json,
}

pub fn print_note_count(count: usize) {
    println!("Found {} note{}", count, if count == 1 { "" } else { "s" });

    println!();
}

pub fn print_notes(notes: &[NoteView], format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Text => {
            print_note_count(notes.len());

            for note in notes {
                print_note_view(note);
            }
        }

        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(notes,)?);
        }
    }

    Ok(())
}

pub fn print_json<T>(value: &T) -> Result<()>
where
    T: Serialize,
{
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

pub fn print_note_view(view: &NoteView) {
    println!("────────────────────────────────");

    println!("ID: {}", view.note.id,);

    println!("File: {}", view.source_file.display(),);

    match &view.note.anchor {
        NoteAnchor::Line(anchor) => {
            println!("Location: {}:{}", anchor.line, anchor.column,);
        }

        NoteAnchor::Symbol(anchor) => {
            println!("Symbol: {}", anchor.name,);

            println!("Kind: {:?}", anchor.kind,);

            if let Some(signature) = &anchor.signature {
                println!("Signature: {}", signature);
            }

            if let Some(line_hint) = anchor.line_hint {
                println!("Line hint: {}", line_hint);
            }
        }
    }

    println!();

    println!("{}", view.note.content,);

    println!();
}
