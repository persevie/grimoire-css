#![cfg(feature = "mcp")]

use std::fs;

#[test]
fn mcp_remains_a_thin_adapter_over_existing_public_api() {
    let source = fs::read_to_string("src/mcp/mod.rs").unwrap();
    assert!(source.contains("analyzer::Analyzer"));
    for forbidden in [
        "ProjectContext",
        "WorkspaceContext",
        "crate::core",
        "Spell::new",
        "start_in_memory",
        "std::fs",
        "libc::",
        "openat",
        "OwnedFd",
        "BorrowedFd",
        "mutation",
    ] {
        assert!(
            !source.contains(forbidden),
            "MCP adapter must not own engine/filesystem behavior: {forbidden}"
        );
    }
}

#[test]
fn transmutator_has_one_domain_implementation_and_no_dependency_cycle() {
    let manifest = fs::read_to_string("Cargo.toml").unwrap();
    assert!(manifest.contains("cssparser ="));
    assert!(!manifest.contains("grimoire_css_transmutator"));
    assert!(manifest.contains("analyzer = []"));
    let mcp_feature = manifest
        .lines()
        .find(|line| line.starts_with("mcp = ["))
        .expect("mcp feature declaration");
    for dependency in ["analyzer", "dep:jsonschema", "dep:tempfile"] {
        assert!(
            mcp_feature.contains(&format!("\"{dependency}\"")),
            "mcp feature must include {dependency}"
        );
    }

    let transmutator = fs::read_to_string("src/transmutator.rs").unwrap();
    let cli = fs::read_to_string("src/commands/transmute.rs").unwrap();
    let analyzer = fs::read_to_string("src/analyzer/mod.rs").unwrap();
    let mcp = fs::read_to_string("src/mcp/mod.rs").unwrap();
    assert!(transmutator.contains("use cssparser::"));
    assert!(transmutator.contains("IndexMap"));
    assert!(transmutator.contains("IndexSet"));
    assert!(!transmutator.contains("type TransmutedMap = Vec"));
    assert!(transmutator.contains("fn process_css_into_raw_spells"));
    assert!(!transmutator.contains("current_dir"));
    assert!(cli.contains("transmute_css"));
    assert!(cli.contains("transmute_paths"));
    assert!(analyzer.contains("transmute_css"));
    assert!(analyzer.contains("transmute_paths"));
    assert!(mcp.contains("Analyzer::transmute_and_validate"));
    assert!(mcp.contains("Analyzer::import_css"));
    for adapter in [cli, analyzer, mcp] {
        assert!(!adapter.contains("cssparser::"));
        assert!(!adapter.contains("process_css_into_raw_spells"));
    }
}

#[test]
fn rejected_mcp_architecture_is_absent_from_the_repository() {
    for path in [
        "src/mcp/project_context.rs",
        "src/mcp/project_context/authority.rs",
        "src/mcp/project_context/macos.rs",
        "src/mcp/project_context/native.rs",
    ] {
        assert!(
            !std::path::Path::new(path).exists(),
            "obsolete MCP path returned: {path}"
        );
    }
}

#[test]
fn cli_and_mcp_share_refs_and_stats_implementations() {
    let fi = fs::read_to_string("src/commands/fi.rs").unwrap();
    let mcp = fs::read_to_string("src/mcp/mod.rs").unwrap();
    for shared_call in ["Analyzer::refs", "Analyzer::stats"] {
        assert!(fi.contains(shared_call), "fi must call {shared_call}");
        assert!(mcp.contains(shared_call), "MCP must call {shared_call}");
    }
}
