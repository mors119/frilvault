use anyhow::Result;
use frilvault_core::FrilVault;

use crate::{
    cli::gitignore::{GitignoreAction, GitignoreCheckCommand, GitignoreCommand},
    output::{OutputFormat, print_json, resolve_format},
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

    if matches!(resolve_format(command.format), OutputFormat::Json) {
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
