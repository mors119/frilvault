use anyhow::Result;

use crate::app::create_index_repository;

pub fn execute() -> Result<()> {
    let repository = create_index_repository()?;

    let health = repository.health_check()?;

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
