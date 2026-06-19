use anyhow::Result;
use frilvault_core::create_note_service;
use uuid::Uuid;

use crate::cli::delete::DeleteCommand;

pub fn execute(command: DeleteCommand) -> Result<()> {
    let mut service = create_note_service()?;

    service.delete_note(&command.file, Uuid::parse_str(&command.id)?)?;

    println!("Note deleted");

    Ok(())
}
