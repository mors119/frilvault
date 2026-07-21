use anyhow::Result;
use frilvault_core::FrilVault;

use crate::{
    cli::stats::{StatsCommand, StatsFormatArg},
    output::{OutputFormat, print_json},
};

pub fn execute(command: StatsCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.workspace()?;

    let stats = service.stats()?;

    let format = match (command.format, command.json) {
        (Some(StatsFormatArg::Json), _) | (None, true) => OutputFormat::Json,
        _ => OutputFormat::Text,
    };

    if matches!(format, OutputFormat::Json) {
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
