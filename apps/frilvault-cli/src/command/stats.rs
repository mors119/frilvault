use anyhow::Result;

use crate::app::create_workspace_service;

pub fn execute() -> Result<()> {
    let service = create_workspace_service()?;

    let stats = service.stats()?;

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
