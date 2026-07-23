use anyhow::Result;
use frilvault_core::FrilVault;

use crate::{
    cli::health::HealthCommand,
    output::{OutputFormat, print_json, resolve_format},
};

pub fn execute(command: HealthCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.workspace()?;

    let health = service.health_check()?;

    if matches!(resolve_format(command.format), OutputFormat::Json) {
        print_json(&health)?;
        return Ok(());
    }

    println!("Workspace Health Check\n");

    if health.missing_source_files.is_empty() {
        println!("No missing source files.");

        return Ok(());
    }

    println!("Missing Source Files\n");

    for file in health.missing_source_files {
        println!("- {}", file);
    }

    Ok(())
}
