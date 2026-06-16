use anyhow::Result;
use uuid::Uuid;

use crate::{app::create_note_service, cli::delete::DeleteCommand};

pub fn execute(command: DeleteCommand) -> Result<()> {
    let mut service = create_note_service()?;

    service.delete_note(&command.file, Uuid::parse_str(&command.id)?)?;

    println!("Note deleted");

    Ok(())
}
