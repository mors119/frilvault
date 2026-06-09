use crate::PathResolver;

use std::path::PathBuf;

#[test]
fn resolve_note_path_returns_vault_path() {
    let resolver = PathResolver::new("/workspace");

    let path = resolver.resolve_note_path("src/main.rs");

    let expected = PathBuf::from("/workspace/.vault/notes/src/main.rs.yml");

    assert_eq!(path, expected);
}
#[test]
fn workspace_relative_path() {
    let resolver = PathResolver::new("/workspace");

    let relative = resolver
        .to_workspace_relative("/workspace/src/main.rs")
        .unwrap();

    assert_eq!(relative.to_string_lossy(), "src/main.rs");
}

#[test]
fn source_file_from_note_path_returns_source_path() {
    let resolver = PathResolver::new("/workspace");

    let source_file = resolver
        .source_file_from_note_path("/workspace/.vault/notes/src/main.rs.yml")
        .unwrap();

    assert_eq!(source_file, PathBuf::from("src/main.rs"));
}
