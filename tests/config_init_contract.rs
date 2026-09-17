use grimoire_css_lib::{build, init, shorten};
use serde_json::json;
use std::{fs, process::Command};
use tempfile::tempdir;

#[test]
fn commands_reject_cyclic_scrolls_without_overwriting_configuration() {
    for mode in ["init", "build", "shorten"] {
        for cli in [false, true] {
            let root = tempdir().unwrap();
            let config_path = root.path().join("grimoire/config/grimoire.config.json");
            fs::create_dir_all(config_path.parent().unwrap()).unwrap();
            let config = serde_json::to_vec_pretty(&json!({
                "version": env!("CARGO_PKG_VERSION"),
                "variables": {"brand": "red"},
                "projects": [{"projectName": "app", "inputPaths": ["src/**/*.html"]}],
                "scrolls": [
                    {"name": "a", "spells": ["color=red"], "extends": ["b"]},
                    {"name": "b", "spells": [], "extends": ["a"]}
                ]
            }))
            .unwrap();
            fs::write(&config_path, &config).unwrap();

            if cli {
                let output = Command::new(env!("CARGO_BIN_EXE_grimoire_css"))
                    .current_dir(root.path())
                    .arg(mode)
                    .output()
                    .unwrap();
                assert!(
                    !output.status.success(),
                    "{mode} must reject cyclic Scrolls"
                );
                let diagnostics = format!(
                    "{}{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                assert!(
                    diagnostics.contains("Cyclic scroll inheritance"),
                    "{diagnostics}"
                );
            } else {
                let result = match mode {
                    "init" => init(root.path()),
                    "build" => build(root.path()),
                    "shorten" => shorten(root.path()),
                    _ => unreachable!(),
                };
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("Cyclic scroll inheritance")
                );
            }
            assert_eq!(fs::read(&config_path).unwrap(), config, "{mode}, cli={cli}");
        }
    }
}

#[test]
fn init_creates_missing_configuration_and_preserves_existing_project_settings() {
    let root = tempdir().unwrap();
    init(root.path()).unwrap();
    let config_path = root.path().join("grimoire/config/grimoire.config.json");
    let mut config: serde_json::Value =
        serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
    assert_eq!(config["version"], env!("CARGO_PKG_VERSION"));
    config["version"] = json!("1.8.0");
    config["variables"] = json!({"brand": "red"});
    config["projects"][0]["projectName"] = json!("custom");
    fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_grimoire_css"))
        .current_dir(root.path())
        .arg("init")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    config["version"] = json!(env!("CARGO_PKG_VERSION"));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&fs::read(config_path).unwrap()).unwrap(),
        config
    );
}
