use anyhow::Result;

use frilvault_core::FrilVault;
use uuid::Uuid;

use crate::cli::delete::DeleteCommand;

pub fn execute(command: DeleteCommand) -> Result<()> {
    let workspace = FrilVault::open(std::env::current_dir()?)?;
    let mut service = FrilVault::create_note_service(&workspace)?;

    service.delete_note(&command.file, Uuid::parse_str(&command.id)?)?;

    println!("Note deleted");

    Ok(())
}
