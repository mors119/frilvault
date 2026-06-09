use anyhow::Result;

use crate::{app::create_workspace_service, cli::repair::RepairCommand};

pub fn execute(command: RepairCommand) -> Result<()> {
    let service = create_workspace_service()?;

    if command.apply {
        let repaired = service.apply_repairs()?;

        println!("Applied {} repair(s)", repaired,);

        return Ok(());
    }

    let suggestions = service.repair_suggestions()?;

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
