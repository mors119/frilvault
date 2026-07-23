use anyhow::{Result, bail};
use frilvault_core::{FrilVault, NoteAnchor};

use crate::{
    cli::search::SearchCommand,
    output::{print_notes, resolve_format},
};

pub fn execute(command: SearchCommand) -> Result<()> {
    if command.keyword.is_none() && command.file.is_none() && command.tag.is_none() {
        bail!("search requires a keyword, --file, or --tag");
    }

    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.notes()?;

    let mut results = if let Some(tag) = command.tag.as_deref() {
        service.search_by_tag(tag)?
    } else {
        match (command.keyword.as_deref(), command.file.as_deref()) {
            (Some(keyword), Some(file)) => {
                let keyword = keyword.to_lowercase();

                service
                    .search_notes_by_file(file)?
                    .into_iter()
                    .filter(|note| note_matches_keyword(note, &keyword))
                    .collect()
            }
            (Some(keyword), None) => service.search_notes(keyword)?,
            (None, Some(file)) => service.search_notes_by_file(file)?,
            (None, None) => Vec::new(),
        }
    };

    if command.tag.is_some() {
        if let Some(file) = command.file.as_deref() {
            results.retain(|note| note.source_file.to_string_lossy() == file);
        }

        if let Some(keyword) = command.keyword.as_deref() {
            let keyword = keyword.to_lowercase();
            results.retain(|note| note_matches_keyword(note, &keyword));
        }
    }

    print_notes(&results, resolve_format(command.format))?;

    Ok(())
}

fn note_matches_keyword(note: &frilvault_core::NoteView, keyword: &str) -> bool {
    let content_match = note.note.content.to_lowercase().contains(keyword);

    let symbol_match = matches!(
        &note.note.anchor,
        NoteAnchor::Symbol(anchor) if anchor.name.to_lowercase().contains(keyword)
    );

    content_match || symbol_match
}
