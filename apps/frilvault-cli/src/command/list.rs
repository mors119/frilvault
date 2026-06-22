use anyhow::Result;
use frilvault_core::FrilVault;

use crate::{
    cli::list::{ListCommand, ListFormatArg},
    output::{OutputFormat, print_notes},
};

pub fn execute(command: ListCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.notes()?;

    let notes = service.list_notes(&command.file)?;

    let format = match (command.format, command.json) {
        (Some(ListFormatArg::Json), _) | (None, true) => OutputFormat::Json,
        _ => OutputFormat::Text,
    };

    print_notes(&notes, format)?;

    Ok(())
}
