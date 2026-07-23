use anyhow::{Result, bail};
use frilvault_core::{FrilVault, NoteQuery};

use crate::{
    cli::search::SearchCommand,
    output::{print_notes, resolve_format},
};

pub fn execute(command: SearchCommand) -> Result<()> {
    if command.keyword.is_none() && command.file.is_none() && command.tag.is_none() {
        bail!("search requires a keyword, --file, or --tag");
    }

    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.notes()?;

    let query = NoteQuery {
        source_file: command.file.map(Into::into),
        keyword: command.keyword,
        tag: command.tag,
    };

    let results = service.query_notes(&query)?;

    print_notes(&results, resolve_format(command.format))?;

    Ok(())
}
