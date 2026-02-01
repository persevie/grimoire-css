#![cfg(feature = "analyzer")]

use grimoire_css_lib::analyzer::Analyzer;
use serde_json::json;
use std::fs;
use tempfile::tempdir;

fn write_fixture_repo() -> tempfile::TempDir {
    let dir = tempdir().expect("tempdir");

    fs::create_dir_all(dir.path().join("src")).expect("create src/");
    let input_file = dir.path().join("src/index.html");
    fs::write(&input_file, r#"<div class="display=flex"></div>"#).expect("write index.html");

    fs::create_dir_all(dir.path().join("grimoire/config")).expect("create grimoire/config/");
    let cfg = json!({
        "variables": { "unusedVar": "123" },
        "scrolls": [
            { "name": "unusedScroll", "spells": ["display=block"] }
        ],
        "projects": [
            {
                "projectName": "main",
                "inputPaths": [input_file.to_string_lossy()]
            }
        ],
        "shared": [
            {
                "outputPath": "shared.css",
                "styles": ["g!color=red;"]
            }
        ]
    });
    fs::write(
        dir.path().join("grimoire/config/grimoire.config.json"),
        serde_json::to_string_pretty(&cfg).expect("serialize config"),
    )
    .expect("write config");

    dir
}

#[test]
fn analyzer_refs_spell_finds_templated_spell() {
    let dir = write_fixture_repo();

    let refs = Analyzer::refs_spell(dir.path(), "display=flex").expect("refs_spell ok");
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].spell, "display=flex");
    assert_eq!(refs[0].occurrence.token, "display=flex");
    assert_eq!(refs[0].occurrence.file, "src/index.html");
}

#[test]
fn analyzer_lint_warns_on_unused_shared_only() {
    let dir = write_fixture_repo();

    let lint = Analyzer::lint(dir.path()).expect("lint ok");
    assert!(lint.errors.is_empty());

    let codes: Vec<&str> = lint.warnings.iter().map(|w| w.code.as_str()).collect();
    assert_eq!(codes, vec!["unused_shared_style"]);
}
