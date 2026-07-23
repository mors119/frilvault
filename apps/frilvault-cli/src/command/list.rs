use anyhow::Result;
use frilvault_core::{FrilVault, NoteQuery};

use crate::{
    cli::list::ListCommand,
    output::{print_notes, resolve_format},
};

pub fn execute(command: ListCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.notes()?;

    let notes = service.query_notes(&NoteQuery {
        source_file: Some(command.file.into()),
        keyword: None,
        tag: None,
    })?;
    let format = resolve_format(command.format);

    print_notes(&notes, format)?;

    Ok(())
}
