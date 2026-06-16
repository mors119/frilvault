use anyhow::Result;

use crate::app::create_workspace_service;

pub fn execute() -> Result<()> {
    let mut service = create_workspace_service()?;

    let health = service.health_check()?;

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
