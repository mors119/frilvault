use anyhow::Result;
use frilvault_core::{FrilVault, NoteAnchor};

use crate::{
    cli::search::SearchCommand,
    output::{print_notes, resolve_format},
};

pub fn execute(command: SearchCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.notes()?;

    let results = match (command.keyword.as_deref(), command.file.as_deref()) {
        (Some(keyword), Some(file)) => {
            let keyword = keyword.to_lowercase();

            service
                .search_notes_by_file(file)?
                .into_iter()
                .filter(|note| {
                    let content_match = note.note.content.to_lowercase().contains(&keyword);

                    let symbol_match = matches!(
                        &note.note.anchor,
                        NoteAnchor::Symbol(anchor)
                            if anchor.name.to_lowercase().contains(&keyword)
                    );

                    content_match || symbol_match
                })
                .collect()
        }
        (Some(keyword), None) => service.search_notes(keyword)?,
        (None, Some(file)) => service.search_notes_by_file(file)?,
        (None, None) => anyhow::bail!("search requires either a keyword or --file"),
    };

    print_notes(&results, resolve_format(command.format))?;

    Ok(())
}
