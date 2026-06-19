use anyhow::Result;
use frilvault_core::create_note_service;

use crate::{
    cli::search::{SearchCommand, SearchFormatArg},
    output::{OutputFormat, print_notes},
};

pub fn execute(command: SearchCommand) -> Result<()> {
    let mut service = create_note_service()?;

    let results = match (command.keyword.as_deref(), command.file.as_deref()) {
        (Some(keyword), Some(file)) => service
            .search_notes(keyword)?
            .into_iter()
            .filter(|note| note.source_file.to_string_lossy() == file)
            .collect(),
        (Some(keyword), None) => service.search_notes(keyword)?,
        (None, Some(file)) => service.list_notes(file)?,
        (None, None) => anyhow::bail!("search requires either a keyword or --file"),
    };

    let format = match (command.format, command.json) {
        (Some(SearchFormatArg::Json), _) | (None, true) => OutputFormat::Json,
        _ => OutputFormat::Text,
    };

    print_notes(&results, format)?;

    Ok(())
}
