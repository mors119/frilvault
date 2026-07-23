use anyhow::{Context, Result};
use frilvault_core::FrilVault;
use uuid::Uuid;

use crate::{
    cli::attach::AttachCommand,
    output::{OutputFormat, print_json, resolve_format},
};

pub fn execute(command: AttachCommand) -> Result<()> {
    let note_id = Uuid::parse_str(&command.id).context("invalid note id")?;

    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.notes()?;

    let attachment = service.attach_image(&command.file, note_id, &command.image)?;

    if matches!(resolve_format(command.format), OutputFormat::Json) {
        print_json(&attachment)?;
        return Ok(());
    }

    println!(
        "Attached {} ({})",
        attachment.filename, attachment.mime_type
    );

    Ok(())
}
