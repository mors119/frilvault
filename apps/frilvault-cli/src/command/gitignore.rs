use anyhow::Result;
use frilvault_core::FrilVault;

use crate::{
    cli::gitignore::{
        GitignoreAction, GitignoreCheckCommand, GitignoreCommand, GitignoreFormatArg,
    },
    output::{OutputFormat, print_json},
};

#[derive(serde::Serialize)]
struct GitignoreStatus {
    ignored: bool,
}

pub fn execute(command: GitignoreCommand) -> Result<()> {
    match command.action {
        GitignoreAction::Check(check) => execute_check(check),
        GitignoreAction::Add => execute_add(),
    }
}

fn execute_check(command: GitignoreCheckCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let service = vault.workspace()?;
    let ignored = service.is_vault_gitignored()?;

    let format = match (command.format, command.json) {
        (Some(GitignoreFormatArg::Json), _) | (None, true) => OutputFormat::Json,
        _ => OutputFormat::Text,
    };

    if matches!(format, OutputFormat::Json) {
        print_json(&GitignoreStatus { ignored })?;
        return Ok(());
    }

    if ignored {
        println!(".vault/ is ignored by Git.");
    } else {
        println!(".vault/ is not ignored by Git.");
    }

    Ok(())
}

fn execute_add() -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let service = vault.workspace()?;
    service.append_vault_to_gitignore()?;
    println!("Added .vault/ to .gitignore.");

    Ok(())
}
