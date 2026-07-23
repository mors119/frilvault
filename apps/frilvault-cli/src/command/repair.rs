use anyhow::Result;
use frilvault_core::FrilVault;

use crate::{
    cli::repair::RepairCommand,
    output::{OutputFormat, print_json, resolve_format},
};

pub fn execute(command: RepairCommand) -> Result<()> {
    let vault = FrilVault::open(std::env::current_dir()?)?;
    let mut service = vault.workspace()?;
    service.warm_up()?;
    let format = resolve_format(command.format);

    if command.apply {
        let repaired = service.apply_repairs()?;

        if matches!(format, OutputFormat::Json) {
            print_json(&repaired)?;
            return Ok(());
        }

        println!("Applied {} repair(s)", repaired,);

        return Ok(());
    }

    let suggestions = service.repair_suggestions()?;

    if matches!(format, OutputFormat::Json) {
        print_json(&suggestions)?;
        return Ok(());
    }

    if suggestions.is_empty() {
        println!("No repair suggestions.");

        return Ok(());
    }

    println!("Repair Suggestions\n");

    for suggestion in suggestions {
        println!("Missing: {}\n", suggestion.missing_file,);

        if suggestion.candidates.is_empty() {
            println!("No candidates found.\n");

            continue;
        }

        println!("Possible Matches:");

        for candidate in suggestion.candidates {
            println!("- {}", candidate,);
        }

        println!();
    }

    Ok(())
}
