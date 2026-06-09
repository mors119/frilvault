use anyhow::Result;

use crate::{
    app::create_note_service,
    cli::search::SearchCommand,
    output::{OutputFormat, print_notes},
};

pub fn execute(command: SearchCommand) -> Result<()> {
    let service = create_note_service()?;

    let results = service.search_notes(&command.keyword)?;

    let format = if command.json {
        OutputFormat::Json
    } else {
        OutputFormat::Text
    };

    print_notes(&results, format)?;

    Ok(())
}
