#![cfg(feature = "analyzer")]

use grimoire_css_lib::analyzer::Analyzer;
use serde_json::json;
use std::fs;
use tempfile::tempdir;

fn write_repo_basic() -> tempfile::TempDir {
    let dir = tempdir().expect("tempdir");

    let html = dir.path().join("src/index.html");
    fs::create_dir_all(html.parent().unwrap()).unwrap();

    // - display=flex appears twice (spell_count)
    // - box=10px_20px triggers scroll refs + expanded spells
    // - --my-token=1 + c=var(--my-token) gives css var write+read
    // - bg=$primary gives grimoire variable refs
    // - display=flex + c=var(--my-token) repeated provides DRY candidates
    let content = r#"
<div class="display=flex c=var(--my-token) --my-token=1 box=10px_20px bg=$primary"></div>
<div class="display=flex c=var(--my-token)"></div>
<div class="display=flex c=var(--my-token)"></div>
"#;
    fs::write(&html, content).unwrap();

    fs::create_dir_all(dir.path().join("grimoire/config")).unwrap();

    let cfg = json!({
        "variables": {
            "primary": "#ff0000",
            "unusedVar": "123"
        },
        "scrolls": [
            {
                "name": "box",
                "spells": [
                    "height=var(--box-height)",
                    "width=var(--box-width)",
                    "c=$primary"
                ],
                "spellsByArgs": {
                    "2": [
                        "padding-top=$1",
                        "padding-left=$2"
                    ]
                }
            },
            {
                "name": "unusedScroll",
                "spells": ["display=block"]
            }
        ],
        "projects": [
            {
                "projectName": "main",
                "inputPaths": [html.to_string_lossy()]
            }
        ],
        "shared": [
            {
                "outputPath": "shared.css",
                "styles": ["display=flex"]
            }
        ]
    });

    fs::write(
        dir.path().join("grimoire/config/grimoire.config.json"),
        serde_json::to_string_pretty(&cfg).unwrap(),
    )
    .unwrap();

    dir
}

fn write_repo_with_unused_shared() -> tempfile::TempDir {
    let dir = tempdir().expect("tempdir");

    let html = dir.path().join("src/index.html");
    fs::create_dir_all(html.parent().unwrap()).unwrap();
    fs::write(&html, r#"<div class=\"display=flex\"></div>"#).unwrap();

    fs::create_dir_all(dir.path().join("grimoire/config")).unwrap();

    let cfg = json!({
        "variables": {
            "unusedVar": "123"
        },
        "scrolls": [
            { "name": "unusedScroll", "spells": ["display=block"] }
        ],
        "projects": [
            {
                "projectName": "main",
                "inputPaths": [html.to_string_lossy()]
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
        serde_json::to_string_pretty(&cfg).unwrap(),
    )
    .unwrap();

    dir
}

#[test]
fn load_config_ok_and_missing() {
    let dir = write_repo_basic();
    let cfg = Analyzer::load_config(dir.path()).expect("load_config ok");
    assert!(cfg.scrolls.as_ref().is_some_and(|m| m.contains_key("box")));

    let empty = tempdir().unwrap();
    assert!(Analyzer::load_config(empty.path()).is_err());
}

#[test]
fn config_summary_lists_expected_entities() {
    let dir = write_repo_basic();
    let s = Analyzer::config_summary(dir.path()).expect("config_summary ok");

    assert_eq!(s.config_path, "grimoire/config/grimoire.config.json");
    assert!(s.scrolls.contains(&"box".to_string()));
    assert!(s.variables.iter().any(|v| v.name == "primary"));
}

#[test]
fn index_includes_scroll_refs_and_css_var_usage() {
    let dir = write_repo_basic();
    let idx = Analyzer::index(dir.path(), 50).expect("index ok");

    assert_eq!(idx.files_scanned, 1);
    assert!(idx.token_occurrences >= 1);

    assert!(
        idx.scroll_references
            .iter()
            .any(|r| r.scroll == "box" && r.arity == 2)
    );

    assert!(idx.css_variables_read.contains(&"--my-token".to_string()));
    assert!(
        idx.css_variables_written
            .contains(&"--my-token".to_string())
    );
}

#[test]
fn dry_candidates_finds_and_respects_thresholds() {
    let dir = write_repo_basic();

    let res = Analyzer::dry_candidates(dir.path(), 2, 2).expect("dry_candidates ok");
    assert!(!res.candidates.is_empty());

    let best = &res.candidates[0];
    assert!(best.tokens.contains(&"display=flex".to_string()));
    assert!(best.tokens.contains(&"c=var(--my-token)".to_string()));

    let empty = Analyzer::dry_candidates(dir.path(), 100, 2).expect("dry_candidates ok");
    assert!(empty.candidates.is_empty());
}

#[test]
fn refs_scroll_known_and_unknown() {
    let dir = write_repo_basic();

    let refs = Analyzer::refs_scroll(dir.path(), "box").expect("refs_scroll ok");
    assert!(!refs.is_empty());
    assert!(refs.iter().all(|r| r.scroll == "box"));

    let none = Analyzer::refs_scroll(dir.path(), "nope").expect("refs_scroll ok");
    assert!(none.is_empty());
}

#[test]
fn refs_spell_direct_and_from_scroll_expansion() {
    let dir = write_repo_basic();

    let direct = Analyzer::refs_spell(dir.path(), "display=flex").expect("refs_spell ok");
    assert_eq!(direct.len(), 3);

    let from_scroll = Analyzer::refs_spell(dir.path(), "padding-top=10px").expect("refs_spell ok");
    assert_eq!(from_scroll.len(), 1);
}

#[test]
fn spell_count_known_and_unknown() {
    let dir = write_repo_basic();

    let n = Analyzer::spell_count(dir.path(), "display=flex").expect("spell_count ok");
    assert_eq!(n, 3);

    let z = Analyzer::spell_count(dir.path(), "nope=1").expect("spell_count ok");
    assert_eq!(z, 0);
}

#[test]
fn stats_spells_contains_hot_spells() {
    let dir = write_repo_basic();

    let top = Analyzer::stats_spells(dir.path(), 10).expect("stats_spells ok");
    assert!(!top.is_empty());
    assert!(
        top.iter()
            .any(|s| s.spell == "display=flex" && s.count == 3)
    );
}

#[test]
fn refs_variable_reads_and_writes() {
    let dir = write_repo_basic();

    let refs = Analyzer::refs_variable(dir.path(), "--my-token").expect("refs_variable ok");
    assert!(refs.iter().any(|r| r.kind == "read"));
    assert!(refs.iter().any(|r| r.kind == "write"));

    let none = Analyzer::refs_variable(dir.path(), "--nope").expect("refs_variable ok");
    assert!(none.is_empty());
}

#[test]
fn list_grimoire_variables_is_sorted() {
    let dir = write_repo_basic();

    let vars = Analyzer::list_grimoire_variables(dir.path()).expect("list_grimoire_variables ok");
    assert!(vars.iter().any(|v| v.name == "primary"));
    assert_eq!(vars[0].name, "primary");
}

#[test]
fn refs_grimoire_variable_finds_in_code_and_in_config_json() {
    let dir = write_repo_basic();

    let refs = Analyzer::refs_grimoire_variable(dir.path(), "primary").expect("refs_gvar ok");
    assert!(!refs.is_empty());
    assert!(refs.iter().any(|r| r.spell.contains("$primary")));
    assert!(refs.iter().any(|r| {
        r.occurrence
            .file
            .contains("grimoire/config/grimoire.config.json")
    }));
}

#[test]
fn explain_class_token_scroll_ok_and_invalid_component_errors() {
    let dir = write_repo_basic();

    let res = Analyzer::explain_class_token(dir.path(), "box=10px_20px").expect("explain ok");
    assert_eq!(res.class_token, "box=10px_20px");
    assert!(!res.expanded_spells.is_empty());
    assert!(!res.css.is_empty());

    assert!(Analyzer::explain_class_token(dir.path(), "nope=1").is_err());
}

#[test]
fn lint_warns_on_unused_shared_but_ignores_unused_config_scrolls_vars() {
    let dir = write_repo_with_unused_shared();

    let lint = Analyzer::lint(dir.path()).expect("lint ok");
    assert!(lint.errors.is_empty());

    let codes: Vec<&str> = lint.warnings.iter().map(|w| w.code.as_str()).collect();
    assert_eq!(codes, vec!["unused_shared_style"]);
}

#[test]
fn lint_clean_when_shared_is_used() {
    let dir = write_repo_basic();

    let lint = Analyzer::lint(dir.path()).expect("lint ok");
    assert!(lint.errors.is_empty());
    assert!(lint.warnings.is_empty());
}
