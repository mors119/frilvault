use clap::Parser;

use crate::cli::{Cli, Commands, add::SymbolKindArg, list::ListFormatArg, search::SearchFormatArg};

#[test]
fn parses_list_format_json() {
    let cli = Cli::parse_from(["flvt", "list", "--file", "src/main.rs", "--format", "json"]);

    match cli.command {
        Commands::List(command) => {
            assert_eq!(command.file, "src/main.rs");
            assert!(matches!(command.format, Some(ListFormatArg::Json)));
            assert!(!command.json);
        }
        _ => panic!("expected list command"),
    }
}

#[test]
fn parses_search_with_file_and_json_format() {
    let cli = Cli::parse_from([
        "flvt",
        "search",
        "--file",
        "src/main.rs",
        "--format",
        "json",
    ]);

    match cli.command {
        Commands::Search(command) => {
            assert_eq!(command.keyword, None);
            assert_eq!(command.file.as_deref(), Some("src/main.rs"));
            assert!(matches!(command.format, Some(SearchFormatArg::Json)));
        }
        _ => panic!("expected search command"),
    }
}

#[test]
fn parses_health_command_alias() {
    let cli = Cli::parse_from(["flvt", "health"]);

    assert!(matches!(cli.command, Commands::Health));
}

#[test]
fn parses_symbol_add_command() {
    let cli = Cli::parse_from([
        "flvt",
        "add",
        "--file",
        "src/main.rs",
        "--symbol",
        "main",
        "--kind",
        "function",
        "--content",
        "note",
    ]);

    match cli.command {
        Commands::Add(command) => {
            assert_eq!(command.symbol.as_deref(), Some("main"));
            assert!(matches!(command.kind, SymbolKindArg::Function));
            assert_eq!(command.content, "note");
        }
        _ => panic!("expected add command"),
    }
}
