use anyhow::Result;

use crate::{
    app::create_note_service,
    cli::list::ListCommand,
    output::{OutputFormat, print_notes},
};

pub fn execute(command: ListCommand) -> Result<()> {
    let service = create_note_service()?;

    let notes = service.list_notes(&command.file)?;

    let format = if command.json {
        OutputFormat::Json
    } else {
        OutputFormat::Text
    };

    print_notes(&notes, format)?;

    Ok(())
}
