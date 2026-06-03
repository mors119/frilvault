use anyhow::Result;

use crate::{cli::list::ListCommand, command::create_note_service, output};

pub fn execute(command: ListCommand) -> Result<()> {
    let service = create_note_service()?;

    let notes = service.list_notes(&command.file)?;

    output::print_note_count(notes.len());

    for note in notes {
        output::print_note(&note);
    }

    Ok(())
}
