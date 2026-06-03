use anyhow::Result;
use uuid::Uuid;

use crate::{cli::delete::DeleteCommand, command::create_note_service};

pub fn execute(command: DeleteCommand) -> Result<()> {
    let service = create_note_service()?;

    service.delete_note(&command.file, Uuid::parse_str(&command.id)?)?;

    println!("Note deleted");

    Ok(())
}
