use anyhow::Result;

use frilvault_core::FrilVault;
use uuid::Uuid;

use crate::cli::update::UpdateCommand;

pub fn execute(command: UpdateCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.notes()?;

    service.update_note(
        &command.file,
        Uuid::parse_str(&command.id)?,
        command.content,
    )?;

    println!("Note updated");

    Ok(())
}
