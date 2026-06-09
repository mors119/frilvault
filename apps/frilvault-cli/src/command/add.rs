use anyhow::{Result, bail};

use frilvault_core::{AddNoteInput, LineAnchor, NoteAnchor, SymbolAnchor, SymbolKind};

use crate::{
    app::create_note_service,
    cli::add::{AddCommand, SymbolKindArg},
};

pub fn execute(command: AddCommand) -> Result<()> {
    let service = create_note_service()?;

    let anchor = create_anchor(&command)?;

    service.add_note(AddNoteInput {
        source_file: command.file.into(),
        anchor,
        content: command.content,
    })?;

    println!("Note added successfully");

    Ok(())
}

fn create_anchor(command: &AddCommand) -> Result<NoteAnchor> {
    if let Some(symbol) = &command.symbol {
        return Ok(NoteAnchor::Symbol(SymbolAnchor {
            name: symbol.clone(),
            kind: command.kind.clone().into(),
            signature: command.signature.clone(),
            line_hint: command.line_hint,
        }));
    }

    let Some(line) = command.line else {
        bail!("either --line or --symbol must be provided");
    };

    let column = command.column.unwrap_or(1);

    Ok(NoteAnchor::Line(LineAnchor { line, column }))
}

impl From<SymbolKindArg> for SymbolKind {
    fn from(value: SymbolKindArg) -> Self {
        match value {
            SymbolKindArg::Function => SymbolKind::Function,
            SymbolKindArg::Struct => SymbolKind::Struct,
            SymbolKindArg::Enum => SymbolKind::Enum,
            SymbolKindArg::Trait => SymbolKind::Trait,
            SymbolKindArg::Impl => SymbolKind::Impl,
            SymbolKindArg::Method => SymbolKind::Method,
            SymbolKindArg::Unknown => SymbolKind::Unknown,
        }
    }
}
