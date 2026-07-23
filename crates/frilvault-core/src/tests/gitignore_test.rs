use std::fs;

use crate::tests::helper::create_test_workspace;
use crate::workspace::gitignore::{append_vault_to_gitignore, is_vault_gitignored};

#[test]
fn is_vault_gitignored_returns_false_when_gitignore_is_missing() {
    let workspace = create_test_workspace();

    assert!(!is_vault_gitignored(workspace.root()).unwrap());
}

#[test]
fn is_vault_gitignored_detects_vault_entry() {
    let workspace = create_test_workspace();
    fs::write(workspace.root().join(".gitignore"), ".vault/\n").unwrap();

    assert!(is_vault_gitignored(workspace.root()).unwrap());
}

#[test]
fn is_vault_gitignored_ignores_comments_and_blank_lines() {
    let workspace = create_test_workspace();
    fs::write(
        workspace.root().join(".gitignore"),
        "# local notes\n\nnode_modules/\n",
    )
    .unwrap();

    assert!(!is_vault_gitignored(workspace.root()).unwrap());
}

#[test]
fn append_vault_to_gitignore_creates_gitignore_file() {
    let workspace = create_test_workspace();

    append_vault_to_gitignore(workspace.root()).unwrap();

    let content = fs::read_to_string(workspace.root().join(".gitignore")).unwrap();
    assert!(content.contains(".vault/"));
    assert!(is_vault_gitignored(workspace.root()).unwrap());
}

#[test]
fn append_vault_to_gitignore_appends_to_existing_gitignore() {
    let workspace = create_test_workspace();
    fs::write(workspace.root().join(".gitignore"), "target/\n").unwrap();

    append_vault_to_gitignore(workspace.root()).unwrap();

    let content = fs::read_to_string(workspace.root().join(".gitignore")).unwrap();
    assert!(content.contains("target/"));
    assert!(content.contains(".vault/"));
}

#[test]
fn append_vault_to_gitignore_is_idempotent() {
    let workspace = create_test_workspace();
    fs::write(workspace.root().join(".gitignore"), ".vault/\n").unwrap();

    append_vault_to_gitignore(workspace.root()).unwrap();

    let content = fs::read_to_string(workspace.root().join(".gitignore")).unwrap();
    assert_eq!(content.matches(".vault/").count(), 1);
}
