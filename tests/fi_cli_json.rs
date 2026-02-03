#![cfg(feature = "analyzer")]

use serde_json::Value;
use serde_json::json;
use std::{fs, process::Command};
use tempfile::tempdir;

fn write_fixture_repo() -> tempfile::TempDir {
    let dir = tempdir().expect("tempdir");

    fs::create_dir_all(dir.path().join("src")).expect("create src/");
    let input_file = dir.path().join("src/index.html");
    fs::write(&input_file, r#"<div class="display=flex"></div>"#).expect("write index.html");

    fs::create_dir_all(dir.path().join("grimoire/config")).expect("create grimoire/config/");
    let cfg = json!({
        "projects": [
            {
                "projectName": "main",
                "inputPaths": [input_file.to_string_lossy()]
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

fn run_fi(root: &std::path::Path, args: &[&str]) -> Value {
    let exe = env!("CARGO_BIN_EXE_grimoire_css");
    let mut cmd = Command::new(exe);

    cmd.arg("fi")
        .arg("--root")
        .arg(root)
        .arg("--json")
        .args(args);

    let out = cmd.output().expect("run fi");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    serde_json::from_slice(&out.stdout).expect("stdout json")
}

#[test]
fn fi_refs_finds_spells() {
    let dir = write_fixture_repo();

    let v = run_fi(dir.path(), &["refs", "display=flex"]);
    assert_eq!(v["query"], "display=flex");

    let results = v["results"].as_array().expect("results array");
    assert!(!results.is_empty());

    assert_eq!(results[0]["kind"], "spell");
    assert_eq!(results[0]["name"], "display=flex");

    let refs = results[0]["refs"].as_array().expect("refs array");
    assert_eq!(refs.len(), 1);
}

#[test]
fn fi_stats_token_supports_spells() {
    let dir = write_fixture_repo();

    let v = run_fi(dir.path(), &["stats", "--token", "display=flex"]);
    assert_eq!(v["token"]["kind"], "spell");
    assert_eq!(v["token"]["name"], "display=flex");
    assert_eq!(v["token"]["count"], 1);
}
