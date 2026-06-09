use anyhow::Result;

use frilvault_core::{AddNoteInput, LineAnchor, NoteAnchor};

use crate::{app::create_note_service, cli::add::AddCommand};

pub fn execute(command: AddCommand) -> Result<()> {
    let service = create_note_service()?;

    service.add_note(AddNoteInput {
        source_file: command.file.into(),

        anchor: NoteAnchor::Line(LineAnchor {
            line: command.line,
            column: command.column,
        }),

        content: command.content,
    })?;

    println!("Note added successfully");

    Ok(())
}
