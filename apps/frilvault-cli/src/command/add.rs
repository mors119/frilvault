use anyhow::{Result, bail};
use frilvault_core::{
    AddNoteRequest, LineAnchor, NoteAnchor, SymbolAnchor, SymbolKind, create_note_service,
};

use crate::cli::add::{AddCommand, SymbolKindArg};

pub fn execute(command: AddCommand) -> Result<()> {
    let mut service = create_note_service()?;

    let anchor = create_anchor(&command)?;

    service.add_note(AddNoteRequest {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_anchor_builds_line_anchor() {
        let command = AddCommand {
            file: "src/main.rs".to_string(),
            line: Some(12),
            column: Some(3),
            symbol: None,
            kind: SymbolKindArg::Unknown,
            signature: None,
            line_hint: None,
            content: "note".to_string(),
        };

        let anchor = create_anchor(&command).unwrap();

        match anchor {
            NoteAnchor::Line(line) => {
                assert_eq!(line.line, 12);
                assert_eq!(line.column, 3);
            }
            _ => panic!("expected line anchor"),
        }
    }

    #[test]
    fn create_anchor_builds_symbol_anchor() {
        let command = AddCommand {
            file: "src/main.rs".to_string(),
            line: None,
            column: None,
            symbol: Some("main".to_string()),
            kind: SymbolKindArg::Function,
            signature: Some("fn main()".to_string()),
            line_hint: Some(1),
            content: "note".to_string(),
        };

        let anchor = create_anchor(&command).unwrap();

        match anchor {
            NoteAnchor::Symbol(symbol) => {
                assert_eq!(symbol.name, "main");
                assert_eq!(symbol.kind, SymbolKind::Function);
                assert_eq!(symbol.signature.as_deref(), Some("fn main()"));
                assert_eq!(symbol.line_hint, Some(1));
            }
            _ => panic!("expected symbol anchor"),
        }
    }
}
