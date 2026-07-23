use anyhow::Result;
use frilvault_core::FrilVault;

use crate::{
    cli::list::ListCommand,
    output::{print_notes, resolve_format},
};

pub fn execute(command: ListCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.notes()?;

    let notes = service.list_notes(&command.file)?;
    let format = resolve_format(command.format);

    print_notes(&notes, format)?;

    Ok(())
}
