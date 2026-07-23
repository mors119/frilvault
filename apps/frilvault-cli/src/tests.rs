use clap::Parser;

use crate::cli::{
    Cli, Commands, add::SymbolKindArg, format::FormatArg, gitignore::GitignoreAction,
};

#[test]
fn parses_list_format_json() {
    let cli = Cli::parse_from(["flvt", "list", "--file", "src/main.rs", "--format", "json"]);

    match cli.command {
        Commands::List(command) => {
            assert_eq!(command.file, "src/main.rs");
            assert!(matches!(command.format, Some(FormatArg::Json)));
        }
        _ => panic!("expected list command"),
    }
}

#[test]
fn parses_list_format_text() {
    let cli = Cli::parse_from(["flvt", "list", "--file", "src/main.rs", "--format", "text"]);

    match cli.command {
        Commands::List(command) => {
            assert!(matches!(command.format, Some(FormatArg::Text)));
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
            assert!(matches!(command.format, Some(FormatArg::Json)));
        }
        _ => panic!("expected search command"),
    }
}

#[test]
fn parses_health_command_alias() {
    let cli = Cli::parse_from(["flvt", "health"]);

    match cli.command {
        Commands::Health(command) => {
            assert!(command.format.is_none());
        }
        _ => panic!("expected health command"),
    }
}

#[test]
fn parses_stats_json_format() {
    let cli = Cli::parse_from(["flvt", "stats", "--format", "json"]);

    match cli.command {
        Commands::Stats(command) => {
            assert!(matches!(command.format, Some(FormatArg::Json)));
        }
        _ => panic!("expected stats command"),
    }
}

#[test]
fn rejects_legacy_json_flag() {
    match Cli::try_parse_from(["flvt", "doctor", "--json"]) {
        Err(error) => assert!(error.to_string().contains("--json")),
        Ok(_) => panic!("expected legacy --json flag to be rejected"),
    }
}

#[test]
fn parses_repair_json_format() {
    let cli = Cli::parse_from(["flvt", "repair", "--format", "json"]);

    match cli.command {
        Commands::Repair(command) => {
            assert!(!command.apply);
            assert!(matches!(command.format, Some(FormatArg::Json)));
        }
        _ => panic!("expected repair command"),
    }
}

#[test]
fn parses_repair_interactive_flag() {
    let cli = Cli::parse_from(["flvt", "repair", "--interactive"]);

    match cli.command {
        Commands::Repair(command) => {
            assert!(command.interactive);
            assert!(!command.apply);
        }
        _ => panic!("expected repair command"),
    }
}

#[test]
fn parses_health_json_format() {
    let cli = Cli::parse_from(["flvt", "health", "--format", "json"]);

    match cli.command {
        Commands::Health(command) => {
            assert!(matches!(command.format, Some(FormatArg::Json)));
        }
        _ => panic!("expected health command"),
    }
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

#[test]
fn parses_gitignore_check_json_format() {
    let cli = Cli::parse_from(["flvt", "gitignore", "check", "--format", "json"]);

    match cli.command {
        Commands::Gitignore(command) => match command.action {
            GitignoreAction::Check(check) => {
                assert!(matches!(check.format, Some(FormatArg::Json)));
            }
            _ => panic!("expected gitignore check command"),
        },
        _ => panic!("expected gitignore command"),
    }
}

#[test]
fn resolve_format_defaults_to_text() {
    use crate::output::{OutputFormat, resolve_format};

    assert!(matches!(resolve_format(None), OutputFormat::Text));
    assert!(matches!(
        resolve_format(Some(FormatArg::Text)),
        OutputFormat::Text
    ));
    assert!(matches!(
        resolve_format(Some(FormatArg::Json)),
        OutputFormat::Json
    ));
}
