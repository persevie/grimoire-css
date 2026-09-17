#![cfg(feature = "mcp")]

use serde_json::{Value, json};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::tempdir;

#[test]
fn stdio_binary_completes_a_real_mcp_lifecycle_and_tool_call() {
    let root = tempdir().unwrap();
    let html = root.path().join("src/index.html");
    fs::create_dir_all(html.parent().unwrap()).unwrap();
    fs::write(&html, r#"<div class="display=flex"></div>"#).unwrap();
    fs::create_dir_all(root.path().join("grimoire/config")).unwrap();
    fs::write(
        root.path().join("grimoire/config/grimoire.config.json"),
        serde_json::to_vec_pretty(&json!({
            "version":env!("CARGO_PKG_VERSION"),
            "projects":[{
                "projectName":"main",
                "inputPaths":[html],
                "outputDirPath":"dist",
                "singleOutputFileName":"main.css"
            }]
        }))
        .unwrap(),
    )
    .unwrap();

    let mut child = Command::new(env!("CARGO_BIN_EXE_grimoire_css_mcp"))
        .args(["--root", root.path().to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let stdin = child.stdin.as_mut().unwrap();
        for request in [
            json!({
                "jsonrpc":"2.0","id":1,"method":"initialize",
                "params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"integration-test","version":"1"}}
            }),
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
            json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
            json!({
                "jsonrpc":"2.0","id":3,"method":"resources/read",
                "params":{"uri":"grimoire://components"}
            }),
            json!({
                "jsonrpc":"2.0","id":4,"method":"tools/call",
                "params":{"name":"grimoire_explain","arguments":{"token":"display=flex"}}
            }),
            json!({
                "jsonrpc":"2.0","id":5,"method":"tools/call",
                "params":{"name":"grimoire_build","arguments":{}}
            }),
            json!({
                "jsonrpc":"2.0","id":6,"method":"tools/call",
                "params":{"name":"grimoire_validate_config","arguments":{}}
            }),
            json!({
                "jsonrpc":"2.0","id":7,"method":"tools/call",
                "params":{
                    "name":"grimoire_validate_spells",
                    "arguments":{"tokens":["display=flex","invented=never"]}
                }
            }),
            json!({
                "jsonrpc":"2.0","id":8,"method":"tools/call",
                "params":{"name":"grimoire_check_project","arguments":{}}
            }),
            json!({
                "jsonrpc":"2.0","id":9,"method":"tools/call",
                "params":{
                    "name":"grimoire_transmute_css",
                    "arguments":{"content":".migrated { color: red; }"}
                }
            }),
            json!({
                "jsonrpc":"2.0","id":10,"method":"tools/call",
                "params":{
                    "name":"grimoire_import_css",
                    "arguments":{
                        "content":".migrated { color: red; }",
                        "import_name":"migration"
                    }
                }
            }),
        ] {
            writeln!(stdin, "{request}").unwrap();
        }
    }
    drop(child.stdin.take());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let responses = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        responses.len(),
        10,
        "notification must not produce a response"
    );
    assert_eq!(responses[0]["result"]["protocolVersion"], "2025-11-25");
    assert_eq!(
        responses[1]["result"]["tools"].as_array().unwrap().len(),
        19
    );
    assert_eq!(
        responses[2]["result"]["contents"][0]["mimeType"],
        "application/json"
    );
    assert_eq!(responses[3]["result"]["isError"], false);
    assert_eq!(
        responses[3]["result"]["structuredContent"]["data"]["class_token"],
        "display=flex"
    );
    assert!(
        responses[3]["result"]["structuredContent"]["data"]["css"]
            .as_str()
            .unwrap()
            .contains("display: flex")
    );
    assert_eq!(responses[4]["result"]["isError"], false);
    assert!(
        fs::read_to_string(root.path().join("dist/main.css"))
            .unwrap()
            .contains("display:flex")
    );
    assert_eq!(
        responses[5]["result"]["structuredContent"]["data"]["valid"],
        true
    );
    assert_eq!(
        responses[6]["result"]["structuredContent"]["data"]["valid"],
        false
    );
    assert_eq!(
        responses[7]["result"]["structuredContent"]["data"]["valid"],
        true
    );
    assert_eq!(
        responses[8]["result"]["structuredContent"]["data"]["valid"],
        true
    );
    assert_eq!(
        responses[9]["result"]["structuredContent"]["data"]["valid"],
        true
    );
    assert!(
        root.path()
            .join("grimoire/config/grimoire.migration.scrolls.json")
            .is_file()
    );
}

#[test]
fn cyclic_scroll_inheritance_is_a_tool_error_and_the_server_stays_alive() {
    let root = tempdir().unwrap();
    fs::create_dir_all(root.path().join("grimoire/config")).unwrap();
    fs::write(
        root.path().join("grimoire/config/grimoire.config.json"),
        serde_json::to_vec(&json!({
            "projects": [],
            "scrolls": [
                {"name":"a", "spells":["color=red"], "extends":["b"]},
                {"name":"b", "spells":[], "extends":["a"]}
            ]
        }))
        .unwrap(),
    )
    .unwrap();
    let config_path = root.path().join("grimoire/config/grimoire.config.json");
    let original_config = fs::read(&config_path).unwrap();
    let expected_error = grimoire_css_lib::config::ConfigFs::load(root.path())
        .unwrap_err()
        .to_string();
    let mut child = Command::new(env!("CARGO_BIN_EXE_grimoire_css_mcp"))
        .args(["--root", root.path().to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    for request in [
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"grimoire_validate_config","arguments":{}}}),
        json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"grimoire_build","arguments":{}}}),
        json!({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"grimoire_init","arguments":{}}}),
        json!({"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"grimoire_shorten","arguments":{}}}),
        json!({"jsonrpc":"2.0","id":6,"method":"ping"}),
    ] {
        writeln!(child.stdin.as_mut().unwrap(), "{request}").unwrap();
    }
    drop(child.stdin.take());
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let replies: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(replies.len(), 6);
    let data = &replies[1]["result"]["structuredContent"]["data"];
    assert_eq!(data["valid"], false);
    assert_eq!(data["engine_load_valid"], false);
    assert!(
        data["issues"]
            .to_string()
            .contains("Cyclic scroll inheritance")
    );
    for reply in &replies[2..5] {
        assert_eq!(reply["result"]["isError"], true, "{reply}");
        assert_eq!(
            reply["result"]["structuredContent"]["error"],
            expected_error
        );
    }
    assert_eq!(replies[5]["result"], json!({}));
    assert_eq!(fs::read(config_path).unwrap(), original_config);
}

#[test]
fn explicit_root_resolves_relative_config_paths_independently_of_launcher_directory() {
    for project_name in ["project", "project[1]"] {
        for relative_root in [false, true] {
            let workspace = tempdir().unwrap();
            let host = workspace.path().join("host");
            let root = workspace.path().join(project_name);
            for dir in [&host, &root] {
                fs::create_dir_all(dir.join("src")).unwrap();
                fs::write(
                    dir.join("src/index.html"),
                    if dir == &root {
                        "<html><head></head><body class=\"display=flex\"></body></html>"
                    } else {
                        "<html><head></head><body class=\"display=grid\"></body></html>"
                    },
                )
                .unwrap();
                fs::create_dir_all(dir.join("styles")).unwrap();
                fs::write(
                    dir.join("styles/shared.css"),
                    if dir == &root {
                        ".shared { color: red; }"
                    } else {
                        ".wrong { color: blue; }"
                    },
                )
                .unwrap();
            }
            fs::create_dir_all(root.join("grimoire/config")).unwrap();
            fs::write(
                root.join("grimoire/grimoire.lock.json"),
                serde_json::to_vec(&json!({"paths":[
                    root.join("dist/main.css"), "dist/shared.css", "dist/stale.css"
                ]}))
                .unwrap(),
            )
            .unwrap();
            fs::create_dir_all(root.join("dist")).unwrap();
            fs::write(root.join("dist/stale.css"), "obsolete").unwrap();
            let config = json!({
                "version": env!("CARGO_PKG_VERSION"),
                "projects": [{"projectName":"main", "inputPaths":["src/**/*.html", "missing/**/*.html"],
                    "outputDirPath":"dist", "singleOutputFileName":"main.css"}],
                "shared": [{"outputPath":"dist/shared.css", "styles":["styles/shared.css"]}],
                "critical": [{"fileToInlinePaths":["src/**/*.html"], "styles":["styles/shared.css"]}],
                "lock": true
            });
            fs::write(
                root.join("grimoire/config/grimoire.config.json"),
                serde_json::to_vec(&config).unwrap(),
            )
            .unwrap();
            let root_argument = if relative_root {
                std::path::PathBuf::from("..").join(project_name)
            } else {
                root.clone()
            };
            let mut child = Command::new(env!("CARGO_BIN_EXE_grimoire_css_mcp"))
                .current_dir(&host)
                .arg("--root")
                .arg(root_argument)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            for (id, name) in [(1, "grimoire_index"), (2, "grimoire_check_project")] {
                writeln!(child.stdin.as_mut().unwrap(), "{}", json!({"jsonrpc":"2.0", "id":id, "method":"tools/call", "params":{"name":name,"arguments":{}}})).unwrap();
            }
            drop(child.stdin.take());
            let output = child.wait_with_output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let replies: Vec<Value> = String::from_utf8(output.stdout)
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();
            assert_eq!(
                replies[0]["result"]["structuredContent"]["data"]["files_scanned"], 1,
                "{replies:?}"
            );
            assert_eq!(
                replies[1]["result"]["structuredContent"]["data"]["valid"], true,
                "{replies:?}"
            );
            let css = fs::read_to_string(root.join("dist/main.css")).unwrap();
            assert!(css.contains("display:flex"), "{css}");
            assert!(!css.contains("display:grid"), "{css}");
            let shared = fs::read_to_string(root.join("dist/shared.css")).unwrap();
            assert!(
                shared.contains(".shared{") && shared.contains("color:red"),
                "{shared}"
            );
            assert!(
                fs::read_to_string(root.join("src/index.html"))
                    .unwrap()
                    .contains(&format!(
                        "<style data-grimoire-critical-css>{shared}</style>"
                    ))
            );
            assert!(!host.join("dist").exists());
            assert!(!root.join("dist/stale.css").exists());
            assert!(!host.join("grimoire").exists());
            assert!(!host.join(".browserslistrc").exists());
            assert!(
                !fs::read_to_string(host.join("src/index.html"))
                    .unwrap()
                    .contains("data-grimoire")
            );
            assert_eq!(
                serde_json::from_slice::<Value>(
                    &fs::read(root.join("grimoire/config/grimoire.config.json")).unwrap()
                )
                .unwrap(),
                config
            );
            let cli = Command::new(env!("CARGO_BIN_EXE_grimoire_css"))
                .current_dir(&root)
                .arg("build")
                .output()
                .unwrap();
            assert!(
                cli.status.success(),
                "{}",
                String::from_utf8_lossy(&cli.stderr)
            );
            assert_eq!(fs::read_to_string(root.join("dist/main.css")).unwrap(), css);
            assert_eq!(
                fs::read_to_string(root.join("dist/shared.css")).unwrap(),
                shared
            );
        }
    }
}
