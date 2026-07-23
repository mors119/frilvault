use anyhow::{Context, Result};
use frilvault_core::FrilVault;

use crate::{
    cli::resolve_uri::ResolveUriCommand,
    output::{OutputFormat, print_json, resolve_format},
};

pub fn execute(command: ResolveUriCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.notes()?;

    let note_view = service
        .resolve_note_uri(&command.uri)
        .context("failed to resolve note uri")?;

    if matches!(resolve_format(command.format), OutputFormat::Json) {
        print_json(&note_view)?;
        return Ok(());
    }

    println!("{}:{}", note_view.source_file.display(), note_view.note.id);

    Ok(())
}
