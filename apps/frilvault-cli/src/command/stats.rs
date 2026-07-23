use anyhow::Result;
use frilvault_core::FrilVault;

use crate::{
    cli::stats::StatsCommand,
    output::{OutputFormat, print_json, resolve_format},
};

pub fn execute(command: StatsCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.workspace()?;

    let stats = service.stats()?;

    if matches!(resolve_format(command.format), OutputFormat::Json) {
        print_json(&stats)?;
        return Ok(());
    }

    println!("Workspace Statistics\n");

    println!("Files: {}", stats.file_count,);

    println!("Total Notes: {}", stats.total_notes,);

    println!("Existing Files: {}", stats.existing_files,);

    println!("Missing Files: {}", stats.missing_files,);

    println!();

    println!("Line Notes: {}", stats.line_notes,);

    println!("Symbol Notes: {}", stats.symbol_notes,);

    Ok(())
}
