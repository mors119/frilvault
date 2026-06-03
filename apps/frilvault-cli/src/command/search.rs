use anyhow::Result;

use crate::{cli::search::SearchCommand, command::create_note_service, output};

pub fn execute(command: SearchCommand) -> Result<()> {
    let service = create_note_service()?;

    let notes = service.search_notes(&command.keyword)?;

    output::print_note_count(notes.len());

    for note in notes {
        output::print_note(&note);
    }

    Ok(())
}
