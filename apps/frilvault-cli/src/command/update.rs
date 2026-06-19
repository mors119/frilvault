use anyhow::Result;
use frilvault_core::create_note_service;
use uuid::Uuid;

use crate::cli::update::UpdateCommand;

pub fn execute(command: UpdateCommand) -> Result<()> {
    let mut service = create_note_service()?;

    service.update_note(
        &command.file,
        Uuid::parse_str(&command.id)?,
        command.content,
    )?;

    println!("Note updated");

    Ok(())
}
