use std::fs;
use std::path::Path;

use crate::{FrilVaultResult, constants::VAULT_DIR_NAME};

const GITIGNORE_ENTRY: &str = ".vault/";

pub fn is_vault_gitignored(workspace_root: &Path) -> FrilVaultResult<bool> {
    let gitignore_path = workspace_root.join(".gitignore");

    if !gitignore_path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(gitignore_path)?;

    Ok(content.lines().any(is_vault_ignore_pattern))
}

pub fn append_vault_to_gitignore(workspace_root: &Path) -> FrilVaultResult<()> {
    if is_vault_gitignored(workspace_root)? {
        return Ok(());
    }

    let gitignore_path = workspace_root.join(".gitignore");

    if gitignore_path.exists() {
        let mut content = fs::read_to_string(&gitignore_path)?;

        if !content.ends_with('\n') {
            content.push('\n');
        }

        content.push_str(GITIGNORE_ENTRY);
        content.push('\n');
        fs::write(gitignore_path, content)?;
    } else {
        fs::write(gitignore_path, format!("{GITIGNORE_ENTRY}\n"))?;
    }

    Ok(())
}

fn is_vault_ignore_pattern(line: &str) -> bool {
    let line = line.split('#').next().unwrap_or("").trim();

    if line.is_empty() {
        return false;
    }

    line == VAULT_DIR_NAME || line == GITIGNORE_ENTRY || line == "**/.vault" || line == "**/.vault/"
}
