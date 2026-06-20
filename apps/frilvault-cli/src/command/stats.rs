use anyhow::Result;
use frilvault_core::FrilVault;

pub fn execute() -> Result<()> {
    let workspace = FrilVault::open(std::env::current_dir()?)?;
    let mut service = FrilVault::create_workspace_service(&workspace)?;

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
