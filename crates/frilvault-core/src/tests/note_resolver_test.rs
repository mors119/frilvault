use crate::PathResolver;

#[test]
fn resolve_note_path_returns_vault_path() {
    let resolver = PathResolver::new("/workspace");

    let path = resolver.resolve_note_path("src/main.rs");

    assert!(path.ends_with(".vault/src/main.rs.yml"));
}

#[test]
fn workspace_relative_path() {
    let resolver = PathResolver::new("/workspace");

    let relative = resolver
        .to_workspace_relative("/workspace/src/main.rs")
        .unwrap();

    assert_eq!(relative.to_string_lossy(), "src/main.rs");
}
