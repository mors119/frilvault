use anyhow::Result;
use frilvault_core::FrilVault;

pub fn execute() -> Result<()> {
    let workspace = FrilVault::open(std::env::current_dir()?)?;
    let mut service = FrilVault::create_workspace_service(&workspace)?;

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
