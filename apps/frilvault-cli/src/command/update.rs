use anyhow::Result;
use uuid::Uuid;

use crate::{cli::update::UpdateCommand, command::create_note_service};

pub fn execute(command: UpdateCommand) -> Result<()> {
    let service = create_note_service()?;

    service.update_note(
        &command.file,
        Uuid::parse_str(&command.id)?,
        command.content,
    )?;

    println!("Note updated");

    Ok(())
}
