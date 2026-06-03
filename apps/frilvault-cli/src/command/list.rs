use anyhow::Result;

use crate::{cli::list::ListCommand, command::create_note_service};

pub fn execute(command: ListCommand) -> Result<()> {
    let service = create_note_service()?;

    let notes = service.list_notes(&command.file)?;

    println!("Found {} notes", notes.len());

    for note in notes {
        println!("\nID: {}", note.id);

        println!("\nFilename: {:?}", note.source_file);

        println!("Content: {}", note.content);

        println!("Anchor: {:?}", note.anchor);
    }

    Ok(())
}
