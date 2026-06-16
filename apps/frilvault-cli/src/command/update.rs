use anyhow::Result;
use uuid::Uuid;

use crate::{app::create_note_service, cli::update::UpdateCommand};

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
