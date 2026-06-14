use anyhow::Result;

use crate::{
    app::create_note_service,
    cli::list::{ListCommand, ListFormatArg},
    output::{OutputFormat, print_notes},
};

pub fn execute(command: ListCommand) -> Result<()> {
    let service = create_note_service()?;

    let notes = service.list_notes(&command.file)?;

    let format = match (command.format, command.json) {
        (Some(ListFormatArg::Json), _) | (None, true) => OutputFormat::Json,
        _ => OutputFormat::Text,
    };

    print_notes(&notes, format)?;

    Ok(())
}
