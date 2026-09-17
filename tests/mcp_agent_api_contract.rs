#![cfg(feature = "mcp")]

use grimoire_css_lib::{analyzer::Analyzer, build_with_options, init, mcp::McpServer, shorten};
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;

fn fixture() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    let html = dir.path().join("src/index.html");
    fs::create_dir_all(html.parent().unwrap()).unwrap();
    fs::write(
        &html,
        r#"<div class="display=flex box=10px_20px border-radius=0.375rem"></div>"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("grimoire/config")).unwrap();
    fs::write(
        dir.path().join("grimoire/config/grimoire.config.json"),
        serde_json::to_vec_pretty(&json!({
            "version": env!("CARGO_PKG_VERSION"),
            "variables": { "primary": "#ff0000" },
            "scrolls": [{
                "name": "box",
                "spells": ["display=block"],
                "spellsByArgs": { "2": ["padding-top=$1", "padding-left=$2"] }
            }],
            "projects": [{
                "projectName": "main",
                "inputPaths": ["src/**/*.html"],
                "outputDirPath": "dist",
                "singleOutputFileName": "main.css"
            }]
        }))
        .unwrap(),
    )
    .unwrap();
    dir
}

fn call(server: &McpServer, id: u64, name: &str, arguments: Value) -> Value {
    server
        .dispatch(json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/call",
            "params": { "name": name, "arguments": arguments }
        }))
        .unwrap()
}

#[test]
fn discovery_is_small_and_request_schemas_cannot_select_a_root() {
    let dir = fixture();
    let response = McpServer::new(dir.path().to_path_buf())
        .dispatch(json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}))
        .unwrap();
    let tools = response["result"]["tools"].as_array().unwrap();
    let names = tools
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            "grimoire_explain",
            "grimoire_config_summary",
            "grimoire_index",
            "grimoire_lint",
            "grimoire_dry",
            "grimoire_list_variables",
            "grimoire_list_scrolls",
            "grimoire_refs",
            "grimoire_stats_spells",
            "grimoire_stats",
            "grimoire_refs_auto",
            "grimoire_validate_config",
            "grimoire_validate_spells",
            "grimoire_check_project",
            "grimoire_transmute_css",
            "grimoire_import_css",
            "grimoire_init",
            "grimoire_build",
            "grimoire_shorten",
        ]
    );
    for tool in tools {
        let schema = &tool["inputSchema"];
        assert!(schema.get("root").is_none());
        assert!(schema["properties"].get("root").is_none());
        assert!(schema["properties"].get("path").is_none());
        assert!(schema["properties"].get("cwd").is_none());
        let mutating = matches!(
            tool["name"].as_str(),
            Some(
                "grimoire_check_project"
                    | "grimoire_import_css"
                    | "grimoire_init"
                    | "grimoire_build"
                    | "grimoire_shorten"
            )
        );
        assert_eq!(tool["annotations"]["readOnlyHint"], !mutating);
        assert_eq!(tool["annotations"]["destructiveHint"], mutating);
        assert_eq!(tool["annotations"]["idempotentHint"], !mutating);
    }
}

#[test]
fn transmute_preview_is_read_only_and_engine_validated() {
    let dir = fixture();
    let response = call(
        &McpServer::new(dir.path().to_path_buf()),
        31,
        "grimoire_transmute_css",
        json!({
            "content":".button { color: red; display: flex; }",
            "with_oneliner":true
        }),
    );
    let data = &response["result"]["structuredContent"]["data"];
    assert_eq!(response["result"]["isError"], false);
    assert_eq!(data["valid"], true, "{response}");
    assert_eq!(data["transmutation"]["scrolls"][0]["name"], "button");
    assert_eq!(
        data["transmutation"]["scrolls"][0]["spells"],
        json!(["color=red", "display=flex"])
    );
    assert_eq!(data["validation"]["valid"], true);
    assert!(
        !dir.path()
            .join("grimoire/config/grimoire.button.scrolls.json")
            .exists()
    );

    let invalid = call(
        &McpServer::new(dir.path().to_path_buf()),
        32,
        "grimoire_transmute_css",
        json!({"content":".button { invented-property: never; }"}),
    );
    assert_eq!(
        invalid["result"]["structuredContent"]["data"]["valid"],
        false
    );
}

#[test]
fn import_writes_external_scroll_and_requires_explicit_replace() {
    let dir = fixture();
    fs::write(
        dir.path().join("src/index.html"),
        r#"<div class="display=flex box=10px_20px imported"></div>"#,
    )
    .unwrap();
    let server = McpServer::new(dir.path().to_path_buf());
    let response = call(
        &server,
        33,
        "grimoire_import_css",
        json!({
            "content":".imported { color: red; }",
            "import_name":"migration"
        }),
    );
    let data = &response["result"]["structuredContent"]["data"];
    assert_eq!(data["valid"], true, "{response}");
    assert_eq!(data["rolled_back"], false);
    assert_eq!(data["project_check"]["valid"], true);
    assert_eq!(data["build_outputs_transactional"], false);
    let import_path = dir
        .path()
        .join("grimoire/config/grimoire.migration.scrolls.json");
    let imported: Value = serde_json::from_slice(&fs::read(&import_path).unwrap()).unwrap();
    assert_eq!(
        imported,
        json!({"scrolls":[{"name":"imported","spells":["color=red"]}]})
    );

    let refused = call(
        &server,
        34,
        "grimoire_import_css",
        json!({
            "content":".imported { color: blue; }",
            "import_name":"migration"
        }),
    );
    assert_eq!(refused["result"]["isError"], true);
    assert!(
        refused["result"]["structuredContent"]["error"]
            .as_str()
            .unwrap()
            .contains("replace=true")
    );

    let replaced = call(
        &server,
        35,
        "grimoire_import_css",
        json!({
            "content":".imported { color: blue; }",
            "import_name":"migration",
            "replace":true
        }),
    );
    assert_eq!(
        replaced["result"]["structuredContent"]["data"]["valid"],
        true
    );
    assert!(
        fs::read_to_string(import_path)
            .unwrap()
            .contains("color=blue")
    );
}

#[test]
fn freshly_initialized_project_validates_and_accepts_an_import() {
    let dir = tempdir().unwrap();
    let server = McpServer::new(dir.path().to_path_buf());

    let initialized = call(&server, 351, "grimoire_init", json!({}));
    assert_eq!(initialized["result"]["isError"], false);

    let validation = call(&server, 352, "grimoire_validate_config", json!({}));
    assert_eq!(
        validation["result"]["structuredContent"]["data"]["valid"], true,
        "the config emitted by grimoire_init must satisfy the published schema"
    );

    let imported = call(
        &server,
        353,
        "grimoire_import_css",
        json!({
            "content":".fresh { display: grid; }",
            "import_name":"fresh"
        }),
    );
    assert_eq!(
        imported["result"]["structuredContent"]["data"]["valid"],
        true
    );
}

#[test]
fn import_is_loaded_when_project_root_contains_glob_metacharacters() {
    let parent = tempdir().unwrap();
    let root = parent.path().join(if cfg!(windows) {
        "project[x]-literal"
    } else {
        "project[x]-with-?-literal"
    });
    fs::create_dir_all(root.join("grimoire/config")).unwrap();
    fs::write(
        root.join("grimoire/config/grimoire.config.json"),
        serde_json::to_vec_pretty(&json!({
            "version":env!("CARGO_PKG_VERSION"),
            "projects":[{
                "projectName":"main",
                "inputPaths":[]
            }]
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("grimoire/config/grimoire.existing.scrolls.json"),
        serde_json::to_vec_pretty(&json!({
            "scrolls":[{"name":"occupied","spells":["display=block"]}]
        }))
        .unwrap(),
    )
    .unwrap();
    let server = McpServer::new(root);

    let conflict = call(
        &server,
        354,
        "grimoire_import_css",
        json!({
            "content":".occupied { display: grid; }",
            "import_name":"conflict"
        }),
    );
    assert_eq!(conflict["result"]["isError"], true);
    assert!(
        conflict["result"]["structuredContent"]["error"]
            .as_str()
            .unwrap()
            .contains("scroll name conflict")
    );

    let imported = call(
        &server,
        355,
        "grimoire_import_css",
        json!({
            "content":".literal-root { display: grid; }",
            "import_name":"literal-root"
        }),
    );
    assert_eq!(
        imported["result"]["structuredContent"]["data"]["valid"],
        true
    );

    let listed = call(&server, 356, "grimoire_list_scrolls", json!({}));
    assert_eq!(
        listed["result"]["structuredContent"]["data"],
        json!(["literal-root", "occupied"]),
        "a successful import must be visible to the real config loader"
    );

    let summary = call(&server, 357, "grimoire_config_summary", json!({}));
    assert_eq!(
        summary["result"]["structuredContent"]["data"]["external_scroll_files"],
        json!([
            "grimoire/config/grimoire.existing.scrolls.json",
            "grimoire/config/grimoire.literal-root.scrolls.json"
        ]),
        "the summary must enumerate the same literal-root external files that the loader consumed"
    );
}

#[test]
fn import_rejects_missing_projects_escapes_and_scroll_conflicts() {
    let missing = tempdir().unwrap();
    let missing_response = call(
        &McpServer::new(missing.path().to_path_buf()),
        36,
        "grimoire_import_css",
        json!({"content":".a { color: red; }","import_name":"migration"}),
    );
    assert_eq!(missing_response["result"]["isError"], true);
    assert!(
        missing_response["result"]["structuredContent"]["error"]
            .as_str()
            .unwrap()
            .contains("grimoire_init")
    );

    let conflict = fixture();
    let conflict_response = call(
        &McpServer::new(conflict.path().to_path_buf()),
        37,
        "grimoire_import_css",
        json!({"content":".box { color: red; }","import_name":"migration"}),
    );
    assert_eq!(conflict_response["result"]["isError"], true);
    assert!(
        conflict_response["result"]["structuredContent"]["error"]
            .as_str()
            .unwrap()
            .contains("scroll name conflict")
    );

    fs::write(
        conflict
            .path()
            .join("grimoire/config/grimoire.existing.scrolls.json"),
        serde_json::to_vec_pretty(&json!({
            "scrolls":[{"name":"external-name","spells":["display=grid"]}]
        }))
        .unwrap(),
    )
    .unwrap();
    let external_conflict = call(
        &McpServer::new(conflict.path().to_path_buf()),
        371,
        "grimoire_import_css",
        json!({
            "content":".external-name { color: red; }",
            "import_name":"migration"
        }),
    );
    assert_eq!(external_conflict["result"]["isError"], true);

    let escaped = call(
        &McpServer::new(conflict.path().to_path_buf()),
        38,
        "grimoire_import_css",
        json!({"paths":["../outside.css"],"import_name":"migration"}),
    );
    assert_eq!(escaped["result"]["isError"], true);
}

#[test]
fn path_import_is_root_relative_and_omits_preview_only_oneliner() {
    let dir = fixture();
    fs::write(
        dir.path().join("legacy.css"),
        ".legacy { color: red; display: flex; }",
    )
    .unwrap();
    let response = call(
        &McpServer::new(dir.path().to_path_buf()),
        381,
        "grimoire_import_css",
        json!({
            "paths":["legacy.css"],
            "import_name":"legacy",
            "with_oneliner":true
        }),
    );
    let data = &response["result"]["structuredContent"]["data"];
    assert_eq!(data["valid"], true, "{response}");
    assert_eq!(
        data["transmutation"]["scrolls"][0]["oneliner"],
        "color=red display=flex"
    );
    let written = fs::read_to_string(
        dir.path()
            .join("grimoire/config/grimoire.legacy.scrolls.json"),
    )
    .unwrap();
    assert!(!written.contains("oneliner"));
}

#[test]
fn failed_project_check_restores_replaced_import_file() {
    let dir = fixture();
    fs::write(
        dir.path().join("src/index.html"),
        r#"<div class="display=flex box=10px_20px invented=never"></div>"#,
    )
    .unwrap();
    let target = dir
        .path()
        .join("grimoire/config/grimoire.migration.scrolls.json");
    let original = serde_json::to_vec_pretty(&json!({
        "scrolls":[{"name":"imported","spells":["color=red"]}]
    }))
    .unwrap();
    fs::write(&target, &original).unwrap();

    let response = call(
        &McpServer::new(dir.path().to_path_buf()),
        39,
        "grimoire_import_css",
        json!({
            "content":".unused { color: blue; }",
            "import_name":"migration",
            "replace":true
        }),
    );
    let data = &response["result"]["structuredContent"]["data"];
    assert_eq!(data["valid"], false);
    assert_eq!(data["rolled_back"], true);
    assert_eq!(fs::read(&target).unwrap(), original);
}

#[test]
fn failed_project_check_removes_a_new_import_file() {
    let dir = fixture();
    fs::write(
        dir.path().join("src/index.html"),
        r#"<div class="invented=never"></div>"#,
    )
    .unwrap();
    let target = dir
        .path()
        .join("grimoire/config/grimoire.new-import.scrolls.json");
    let response = call(
        &McpServer::new(dir.path().to_path_buf()),
        391,
        "grimoire_import_css",
        json!({
            "content":".new-scroll { color: blue; }",
            "import_name":"new-import"
        }),
    );
    let data = &response["result"]["structuredContent"]["data"];
    assert_eq!(data["valid"], false);
    assert_eq!(data["rolled_back"], true);
    assert!(!target.exists());
}

#[test]
fn config_validation_uses_schema_and_real_engine_loader() {
    let valid = fixture();
    let server = McpServer::new(valid.path().to_path_buf());
    let expected = Analyzer::validate_config(valid.path()).unwrap();
    assert!(expected.valid);
    let response = call(&server, 28, "grimoire_validate_config", json!({}));
    assert_eq!(
        response["result"]["structuredContent"]["data"],
        serde_json::to_value(expected).unwrap()
    );

    let invalid = fixture();
    let path = invalid.path().join("grimoire/config/grimoire.config.json");
    let mut config: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    config["inventedByAgent"] = json!(true);
    fs::write(&path, serde_json::to_vec_pretty(&config).unwrap()).unwrap();
    let report = Analyzer::validate_config(invalid.path()).unwrap();
    assert!(!report.valid);
    assert!(!report.schema_valid);
    assert!(report.engine_load_valid);
    assert!(report.issues.iter().any(|issue| issue.stage == "schema"));
}

#[test]
fn malformed_and_engine_invalid_configs_have_structured_issues() {
    let malformed = fixture();
    fs::write(
        malformed
            .path()
            .join("grimoire/config/grimoire.config.json"),
        b"{",
    )
    .unwrap();
    let malformed_report = Analyzer::validate_config(malformed.path()).unwrap();
    assert!(!malformed_report.valid);
    assert_eq!(malformed_report.issues[0].stage, "json");

    let external = fixture();
    fs::write(
        external.path().join("grimoire/animations"),
        b"not-a-directory",
    )
    .unwrap();
    let external_report = Analyzer::validate_config(external.path()).unwrap();
    assert!(external_report.schema_valid);
    assert!(!external_report.engine_load_valid);
    assert!(
        external_report
            .issues
            .iter()
            .any(|issue| issue.stage == "engine_load")
    );
}

#[test]
fn malformed_external_configs_make_validation_and_project_check_fail() {
    for (file_name, content) in [
        ("grimoire.broken.scrolls.json", b"{".as_slice()),
        (
            "grimoire.broken.variables.json",
            br#"{"variables":{"primary":42}}"#.as_slice(),
        ),
    ] {
        let dir = fixture();
        let path = dir.path().join("grimoire/config").join(file_name);
        fs::write(&path, content).unwrap();

        let config = Analyzer::validate_config(dir.path()).unwrap();
        assert!(!config.valid, "{file_name} must invalidate the config");
        assert!(!config.schema_valid);
        assert!(
            config
                .issues
                .iter()
                .any(|issue| issue.path.ends_with(file_name)),
            "the report must identify {file_name}"
        );

        let project = Analyzer::check_project(dir.path()).unwrap();
        assert!(!project.valid, "{file_name} must invalidate the project");
        assert!(!project.build.attempted);
    }
}

#[test]
fn batch_spell_validation_checks_every_token_with_the_real_engine() {
    let dir = fixture();
    let tokens = vec![
        "display=flex".to_string(),
        "box=10px_20px".to_string(),
        "invented-spell=never".to_string(),
    ];
    let expected = Analyzer::validate_spells(dir.path(), &tokens).unwrap();
    assert!(!expected.valid);
    assert_eq!(expected.checked, 3);
    assert!(expected.items[0].valid);
    assert!(!expected.items[2].valid);
    assert!(expected.items[2].error.is_some());

    let response = call(
        &McpServer::new(dir.path().to_path_buf()),
        29,
        "grimoire_validate_spells",
        json!({"tokens":tokens}),
    );
    assert_eq!(
        response["result"]["structuredContent"]["data"],
        serde_json::to_value(expected).unwrap()
    );

    let valid = Analyzer::validate_spells(
        dir.path(),
        &["display=flex".to_string(), "box=10px_20px".to_string()],
    )
    .unwrap();
    assert!(valid.valid);
}

#[test]
fn project_check_requires_config_spells_lint_and_real_build() {
    let valid = fixture();
    let expected = Analyzer::check_project(valid.path()).unwrap();
    assert!(expected.valid);
    assert!(expected.config.valid);
    assert!(expected.spells_valid);
    assert!(expected.lint_clean);
    assert!(expected.build.attempted);
    assert!(expected.build.successful);
    assert!(valid.path().join("dist/main.css").is_file());

    let through_mcp = fixture();
    let response = call(
        &McpServer::new(through_mcp.path().to_path_buf()),
        30,
        "grimoire_check_project",
        json!({}),
    );
    assert_eq!(
        response["result"]["structuredContent"]["data"]["valid"],
        true
    );
    assert!(through_mcp.path().join("dist/main.css").is_file());

    let invalid = fixture();
    let config_path = invalid.path().join("grimoire/config/grimoire.config.json");
    let mut config: Value = serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
    config["unknown"] = json!(1);
    fs::write(config_path, serde_json::to_vec_pretty(&config).unwrap()).unwrap();
    let invalid_report = Analyzer::check_project(invalid.path()).unwrap();
    assert!(!invalid_report.valid);
    assert!(!invalid_report.build.attempted);
    assert!(!invalid.path().join("dist/main.css").exists());

    let invalid_spell = fixture();
    fs::write(
        invalid_spell.path().join("src/index.html"),
        r#"<div class="invented-spell=never"></div>"#,
    )
    .unwrap();
    let invalid_spell_report = Analyzer::check_project(invalid_spell.path()).unwrap();
    assert!(!invalid_spell_report.valid);
    assert!(!invalid_spell_report.spells_valid);
    assert!(!invalid_spell_report.spell_errors.is_empty());
    assert!(invalid_spell_report.build.attempted);
    assert!(!invalid_spell_report.build.successful);

    let lint_warning = fixture();
    let config_path = lint_warning
        .path()
        .join("grimoire/config/grimoire.config.json");
    let shared_output = lint_warning.path().join("dist/shared.css");
    let mut config: Value = serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
    config["shared"] = json!([{
        "outputPath": shared_output,
        "styles":["position=absolute"]
    }]);
    fs::write(config_path, serde_json::to_vec_pretty(&config).unwrap()).unwrap();
    let lint_report = Analyzer::check_project(lint_warning.path()).unwrap();
    assert!(!lint_report.valid);
    assert!(lint_report.spells_valid);
    assert!(!lint_report.lint_clean);
    assert!(!lint_report.lint.warnings.is_empty());
    assert!(lint_report.build.successful);
}

#[test]
fn init_is_the_existing_public_init_call_with_the_fixed_root() {
    let direct = tempdir().unwrap();
    let through_mcp = tempdir().unwrap();

    init(direct.path()).unwrap();
    let response = call(
        &McpServer::new(through_mcp.path().to_path_buf()),
        23,
        "grimoire_init",
        json!({}),
    );

    assert_eq!(response["result"]["structuredContent"]["data"], Value::Null);
    assert_eq!(response["result"]["isError"], false);
    assert_eq!(
        fs::read(direct.path().join("grimoire/config/grimoire.config.json")).unwrap(),
        fs::read(
            through_mcp
                .path()
                .join("grimoire/config/grimoire.config.json")
        )
        .unwrap()
    );
}

#[test]
fn build_is_the_existing_public_build_call_with_the_fixed_root() {
    let direct = fixture();
    let through_mcp = fixture();

    build_with_options(direct.path(), false).unwrap();
    let response = call(
        &McpServer::new(through_mcp.path().to_path_buf()),
        21,
        "grimoire_build",
        json!({}),
    );

    assert_eq!(response["result"]["structuredContent"]["data"], Value::Null);
    assert_eq!(response["result"]["isError"], false);
    assert_eq!(
        fs::read(direct.path().join("dist/main.css")).unwrap(),
        fs::read(through_mcp.path().join("dist/main.css")).unwrap()
    );
}

#[test]
fn build_force_version_update_matches_the_existing_cli_option() {
    let direct = fixture();
    let through_mcp = fixture();
    for root in [direct.path(), through_mcp.path()] {
        let config = root.join("grimoire/config/grimoire.config.json");
        let mut value: Value = serde_json::from_slice(&fs::read(&config).unwrap()).unwrap();
        value["version"] = json!("outdated");
        fs::write(config, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    }

    build_with_options(direct.path(), true).unwrap();
    let response = call(
        &McpServer::new(through_mcp.path().to_path_buf()),
        27,
        "grimoire_build",
        json!({"force_version_update":true}),
    );

    assert_eq!(response["result"]["isError"], false);
    assert_eq!(
        fs::read(direct.path().join("dist/main.css")).unwrap(),
        fs::read(through_mcp.path().join("dist/main.css")).unwrap()
    );
    let mcp_config: Value = serde_json::from_slice(
        &fs::read(
            through_mcp
                .path()
                .join("grimoire/config/grimoire.config.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(mcp_config["version"], env!("CARGO_PKG_VERSION"));
}

#[test]
fn shorten_is_the_existing_public_shorten_call_with_the_fixed_root() {
    let direct = fixture();
    let through_mcp = fixture();

    shorten(direct.path()).unwrap();
    let response = call(
        &McpServer::new(through_mcp.path().to_path_buf()),
        22,
        "grimoire_shorten",
        json!({}),
    );

    assert_eq!(response["result"]["structuredContent"]["data"], Value::Null);
    assert_eq!(response["result"]["isError"], false);
    let direct_source = fs::read(direct.path().join("src/index.html")).unwrap();
    let mcp_source = fs::read(through_mcp.path().join("src/index.html")).unwrap();
    assert_eq!(direct_source, mcp_source);
    let direct_source = String::from_utf8(direct_source).unwrap();
    assert!(
        direct_source.contains("bd-rad=0.375rem"),
        "shortened source: {direct_source}"
    );
}

#[test]
fn explain_success_is_exactly_the_existing_analyzer_result() {
    let dir = fixture();
    let expected =
        serde_json::to_value(Analyzer::explain_class_token(dir.path(), "box=10px_20px").unwrap())
            .unwrap();
    let response = call(
        &McpServer::new(dir.path().to_path_buf()),
        2,
        "grimoire_explain",
        json!({"token":"box=10px_20px"}),
    );
    assert_eq!(response["result"]["structuredContent"]["data"], expected);
    assert_eq!(response["result"]["isError"], false);
}

#[test]
fn explain_error_preserves_the_existing_domain_error() {
    let dir = fixture();
    let expected = Analyzer::explain_class_token(dir.path(), "not a spell")
        .unwrap_err()
        .to_string();
    let response = call(
        &McpServer::new(dir.path().to_path_buf()),
        3,
        "grimoire_explain",
        json!({"token":"not a spell"}),
    );
    assert_eq!(response["result"]["isError"], true);
    assert_eq!(response["result"]["structuredContent"]["error"], expected);
}

#[test]
fn project_knowledge_tools_match_existing_analyzer_results() {
    let dir = fixture();
    let server = McpServer::new(dir.path().to_path_buf());

    let summary = call(&server, 4, "grimoire_config_summary", json!({}));
    assert_eq!(
        summary["result"]["structuredContent"]["data"],
        serde_json::to_value(Analyzer::config_summary(dir.path()).unwrap()).unwrap()
    );

    let variables = call(&server, 5, "grimoire_list_variables", json!({}));
    assert_eq!(
        variables["result"]["structuredContent"]["data"],
        serde_json::to_value(Analyzer::list_grimoire_variables(dir.path()).unwrap()).unwrap()
    );

    let scrolls = call(&server, 6, "grimoire_list_scrolls", json!({}));
    assert_eq!(
        scrolls["result"]["structuredContent"]["data"],
        json!(["box"])
    );
}

#[test]
fn analysis_tools_match_existing_analyzer_results() {
    let dir = fixture();
    let server = McpServer::new(dir.path().to_path_buf());

    let index = call(&server, 8, "grimoire_index", json!({"top": 12}));
    assert_eq!(
        index["result"]["structuredContent"]["data"],
        serde_json::to_value(Analyzer::index(dir.path(), 12).unwrap()).unwrap()
    );

    let lint = call(&server, 9, "grimoire_lint", json!({}));
    assert_eq!(
        lint["result"]["structuredContent"]["data"],
        serde_json::to_value(Analyzer::lint(dir.path()).unwrap()).unwrap()
    );

    let dry = call(
        &server,
        10,
        "grimoire_dry",
        json!({"min_support": 2, "min_items": 2}),
    );
    assert_eq!(
        dry["result"]["structuredContent"]["data"],
        serde_json::to_value(Analyzer::dry_candidates(dir.path(), 2, 2).unwrap()).unwrap()
    );
}

#[test]
fn refs_and_stats_are_direct_existing_analyzer_calls() {
    let dir = fixture();
    let server = McpServer::new(dir.path().to_path_buf());

    let spell_refs = call(
        &server,
        16,
        "grimoire_refs",
        json!({"kind":"spell","query":"display=flex"}),
    );
    assert_eq!(
        spell_refs["result"]["structuredContent"]["data"],
        serde_json::to_value(Analyzer::refs_spell(dir.path(), "display=flex").unwrap()).unwrap()
    );

    let scroll_refs = call(
        &server,
        17,
        "grimoire_refs",
        json!({"kind":"scroll","query":"box"}),
    );
    assert_eq!(
        scroll_refs["result"]["structuredContent"]["data"],
        serde_json::to_value(Analyzer::refs_scroll(dir.path(), "box").unwrap()).unwrap()
    );

    let variable_refs = call(
        &server,
        18,
        "grimoire_refs",
        json!({"kind":"variable","query":"$primary"}),
    );
    assert_eq!(
        variable_refs["result"]["structuredContent"]["data"],
        serde_json::to_value(Analyzer::refs_grimoire_variable(dir.path(), "primary").unwrap())
            .unwrap()
    );

    let stats = call(&server, 19, "grimoire_stats_spells", json!({"top":10}));
    assert_eq!(
        stats["result"]["structuredContent"]["data"],
        serde_json::to_value(Analyzer::stats_spells(dir.path(), 10).unwrap()).unwrap()
    );

    let auto_refs = call(
        &server,
        24,
        "grimoire_refs_auto",
        json!({"query":"$primary"}),
    );
    assert_eq!(
        auto_refs["result"]["structuredContent"]["data"],
        Analyzer::refs(dir.path(), "$primary").unwrap()
    );

    let all_stats = call(
        &server,
        25,
        "grimoire_stats",
        json!({"group":"all","top":10}),
    );
    assert_eq!(
        all_stats["result"]["structuredContent"]["data"],
        Analyzer::stats(dir.path(), Some("all"), None, 10).unwrap()
    );

    let token_stats = call(
        &server,
        26,
        "grimoire_stats",
        json!({"token":"box","top":10}),
    );
    assert_eq!(
        token_stats["result"]["structuredContent"]["data"],
        Analyzer::stats(dir.path(), None, Some("box"), 10).unwrap()
    );
}

#[test]
fn initialize_resources_and_notifications_follow_the_mcp_adapter_boundary() {
    let dir = fixture();
    let server = McpServer::new(dir.path().to_path_buf());
    let initialized = server
        .dispatch(json!({
            "jsonrpc":"2.0",
            "id":11,
            "method":"initialize",
            "params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"test","version":"1"}}
        }))
        .unwrap();
    assert_eq!(initialized["result"]["serverInfo"]["name"], "grimoire-css");
    assert_eq!(initialized["result"]["protocolVersion"], "2025-11-25");
    let instructions = initialized["result"]["instructions"].as_str().unwrap();
    for required in [
        "grimoire_validate_spells",
        "grimoire_validate_config",
        "grimoire_check_project",
        "grimoire_transmute_css",
        "grimoire_import_css",
        "valid=true",
    ] {
        assert!(instructions.contains(required));
    }
    let ping = server
        .dispatch(json!({"jsonrpc":"2.0","id":20,"method":"ping"}))
        .unwrap();
    assert_eq!(ping["result"], json!({}));

    let resources = server
        .dispatch(json!({"jsonrpc":"2.0","id":12,"method":"resources/list"}))
        .unwrap();
    assert_eq!(
        resources["result"]["resources"].as_array().unwrap().len(),
        4
    );

    let primer = server
        .dispatch(json!({
            "jsonrpc":"2.0","id":13,"method":"resources/read",
            "params":{"uri":"grimoire://primer"}
        }))
        .unwrap();
    assert!(
        primer["result"]["contents"][0]["text"]
            .as_str()
            .unwrap()
            .contains("grimoire_explain")
    );
    assert!(
        primer["result"]["contents"][0]["text"]
            .as_str()
            .unwrap()
            .contains("MUST NOT tell the user")
    );
    assert!(
        server
            .dispatch(json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
            .is_none()
    );
}

#[test]
fn malformed_json_rpc_envelopes_receive_invalid_request_errors() {
    let dir = fixture();
    let server = McpServer::new(dir.path().to_path_buf());

    for (request, expected_id) in [
        (json!({"jsonrpc":"2.0","id":91}), json!(91)),
        (
            json!({"jsonrpc":"1.0","id":"bad-version","method":"ping"}),
            json!("bad-version"),
        ),
        (json!({"jsonrpc":"2.0","id":92,"method":false}), json!(92)),
        (
            json!({"jsonrpc":"2.0","id":{"invalid":true},"method":"ping"}),
            Value::Null,
        ),
        (json!(["not", "a", "request"]), Value::Null),
        (json!({"jsonrpc":"2.0"}), Value::Null),
    ] {
        let response = server
            .dispatch(request)
            .expect("invalid requests must receive an error response");
        assert_eq!(response["id"], expected_id);
        assert_eq!(response["error"]["code"], -32600);
        assert_eq!(response["error"]["message"], "Invalid Request");
    }
}

#[test]
fn invalid_adapter_arguments_do_not_reach_a_different_root_or_engine_rule() {
    let dir = fixture();
    let server = McpServer::new(dir.path().to_path_buf());
    for arguments in [
        json!({"token":"display=flex","root":"/tmp/other"}),
        json!({"token":"display=flex","path":"../other"}),
    ] {
        let response = call(&server, 14, "grimoire_explain", arguments);
        assert_eq!(response["error"]["code"], -32602);
        assert_eq!(server.root(), dir.path());
    }
    let response = call(&server, 15, "grimoire_index", json!({"top":0}));
    assert_eq!(response["error"]["code"], -32602);
}

#[test]
fn component_resource_is_backed_by_the_existing_component_catalog() {
    let dir = fixture();
    let response = McpServer::new(dir.path().to_path_buf())
        .dispatch(json!({
            "jsonrpc":"2.0",
            "id":7,
            "method":"resources/read",
            "params":{"uri":"grimoire://components"}
        }))
        .unwrap();
    let actual: Value =
        serde_json::from_str(response["result"]["contents"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(
        actual,
        serde_json::to_value(grimoire_css_lib::component::get_all_components_map()).unwrap()
    );
}

#[test]
fn read_only_tools_do_not_create_or_prune_project_directories() {
    let missing = tempdir().unwrap();
    let response = call(
        &McpServer::new(missing.path().to_path_buf()),
        40,
        "grimoire_explain",
        json!({"token":"display=flex"}),
    );
    assert_eq!(response["result"]["isError"], true);
    assert!(!missing.path().join("grimoire").exists());

    let configured = fixture();
    let animations = configured.path().join("grimoire/animations");
    fs::create_dir_all(&animations).unwrap();
    let response = call(
        &McpServer::new(configured.path().to_path_buf()),
        41,
        "grimoire_config_summary",
        json!({}),
    );
    assert_eq!(response["result"]["isError"], false);
    assert!(animations.is_dir());
}

#[test]
fn tool_calls_discard_cli_messages_on_success_and_error() {
    let dir = fixture();
    let server = McpServer::new(dir.path().to_path_buf());

    for id in 50..70 {
        let response = call(&server, id, "grimoire_init", json!({}));
        assert_eq!(response["result"]["isError"], false);
        assert!(grimoire_css_lib::get_logged_messages().is_empty());
    }

    let response = call(
        &server,
        70,
        "grimoire_explain",
        json!({"token":"invented=never"}),
    );
    assert_eq!(response["result"]["isError"], true);
    assert!(grimoire_css_lib::get_logged_messages().is_empty());
}

#[test]
fn validation_compiles_the_complete_token_including_quoted_values() {
    let root = fixture();
    let server = McpServer::new(root.path().to_path_buf());
    let result = call(
        &server,
        100,
        "grimoire_validate_spells",
        json!({
            "tokens": ["content=\"hello\"", "content='hello'", "font-family=\"Open_Sans\"", "color=red\"junk"]
        }),
    );
    let data = &result["result"]["structuredContent"]["data"];
    assert_eq!(data["valid"], false);
    for i in [0, 1] {
        assert_eq!(data["items"][i]["valid"], true);
        assert!(
            data["items"][i]["css"]
                .as_str()
                .unwrap()
                .contains("content: \"hello\"")
        );
    }
    assert!(
        data["items"][2]["css"]
            .as_str()
            .unwrap()
            .contains("Open Sans")
    );
    assert_eq!(data["items"][3]["valid"], false);
}

#[test]
fn import_rolls_back_new_and_replaced_files_when_browserslist_cannot_be_read() {
    for replace in [false, true] {
        let root = fixture();
        let path = root
            .path()
            .join("grimoire/config/grimoire.rollback.scrolls.json");
        let original = br#"{"scrolls":[{"name":"old","spells":["color=blue"]}]}"#;
        if replace {
            fs::write(&path, original).unwrap();
        }
        fs::create_dir(root.path().join(".browserslistrc")).unwrap();
        let response = call(
            &McpServer::new(root.path().to_path_buf()),
            101,
            "grimoire_import_css",
            json!({
                "content": ".new { color:red; }", "import_name": "rollback", "replace": replace
            }),
        );
        let data = &response["result"]["structuredContent"]["data"];
        assert_eq!(data["valid"], false);
        assert_eq!(data["rolled_back"], true);
        if replace {
            assert_eq!(fs::read(path).unwrap(), original);
        } else {
            assert!(!path.exists());
        }
    }
}

#[test]
fn imported_css_keeps_selector_relations_in_the_actual_build() {
    for (input, selector) in [
        (".card.active { color: red; }", ".card.active{"),
        (".card .child { color: red; }", ".card .child{"),
        (
            ".card > .child::before { content: \"x\"; }",
            ".card>.child:before{",
        ),
    ] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            r#"<div class="card active"><span class="child"></span></div>"#,
        )
        .unwrap();
        let response = call(
            &McpServer::new(root.path().to_path_buf()),
            102,
            "grimoire_import_css",
            json!({
                "content": input, "import_name": "selectors"
            }),
        );
        assert_eq!(
            response["result"]["structuredContent"]["data"]["valid"], true,
            "{response}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        assert!(css.contains(selector), "{input}: {css}");
        assert!(
            !css.starts_with(".active{") && !css.contains("}.active{"),
            "compound class must not become independent: {css}"
        );
    }
}

#[test]
fn unsupported_css_import_does_not_publish_or_replace_a_scroll_file() {
    let root = fixture();
    let path = root
        .path()
        .join("grimoire/config/grimoire.unsupported.scrolls.json");
    let server = McpServer::new(root.path().to_path_buf());
    let original = br#"{"scrolls":[{"name":"old","spells":["color=blue"]}]}"#;
    for replace in [false, true] {
        if replace {
            fs::write(&path, original).unwrap();
        }
        let response = call(
            &server,
            103,
            "grimoire_import_css",
            json!({
                "content": ".card { color: red; } @layer utilities { .hidden { display: none; } }",
                "import_name": "unsupported", "replace": replace
            }),
        );
        assert_eq!(response["result"]["isError"], true);
        assert!(
            response["result"]["structuredContent"]["error"]
                .as_str()
                .unwrap()
                .contains("@layer")
        );
        if replace {
            assert_eq!(fs::read(&path).unwrap(), original);
        } else {
            assert!(!path.exists());
        }
    }
}

#[test]
fn migrated_media_lists_keep_all_alternatives_in_engine_generated_css() {
    use lightningcss::{rules::CssRule, stylesheet::StyleSheet, traits::ToCss};
    let root = fixture();
    let response = call(
        &McpServer::new(root.path().to_path_buf()),
        104,
        "grimoire_transmute_css",
        json!({
            "content": "@media print, screen { @media (min-width: 768px), (orientation: landscape) { .card, .other { color: red; } } }"
        }),
    );
    let data = &response["result"]["structuredContent"]["data"];
    assert_eq!(data["valid"], true, "{response}");
    for item in data["validation"]["items"].as_array().unwrap() {
        let css = item["css"].as_str().unwrap();
        let sheet = StyleSheet::parse(css, Default::default()).unwrap();
        assert_eq!(sheet.rules.0.len(), 1);
        let CssRule::Media(media) = &sheet.rules.0[0] else {
            panic!("media condition was lost: {css}")
        };
        assert_eq!(media.query.media_queries.len(), 4, "{css}");
        let queries = media
            .query
            .media_queries
            .iter()
            .map(|q| q.to_css_string(Default::default()).unwrap())
            .collect::<Vec<_>>();
        for medium in ["print", "screen"] {
            for condition in ["768px", "landscape"] {
                assert!(
                    queries
                        .iter()
                        .any(|q| q.contains(medium) && q.contains(condition)),
                    "{queries:?}"
                );
            }
        }
    }
}

#[test]
fn migrated_comment_separated_dimensions_compile_as_two_values() {
    let root = fixture();
    let response = call(
        &McpServer::new(root.path().to_path_buf()),
        105,
        "grimoire_transmute_css",
        json!({
            "content": ".card { margin: 1px/**/2px; }"
        }),
    );
    let data = &response["result"]["structuredContent"]["data"];
    assert_eq!(data["valid"], true, "{response}");
    assert!(
        data["validation"]["items"][0]["css"]
            .as_str()
            .unwrap()
            .contains("margin: 1px 2px")
    );
}

#[test]
fn quoted_values_and_selector_whitespace_cannot_be_reinterpreted_as_spell_prefixes() {
    let root = fixture();
    for (token, expected) in [
        ("content='a:b'", "content: \"a:b\""),
        ("content='a}b'", "content: \"a}b\""),
        ("content='a__b'", "content: \"a  b\""),
        ("{__.child}color=red", " .child"),
    ] {
        let explained = Analyzer::explain_class_token(root.path(), token).unwrap();
        assert!(
            explained.css.contains(expected),
            "{token}: {}",
            explained.css
        );
    }
}

#[test]
fn imported_css_preserves_literal_underscores_in_selectors_and_values() {
    let root = fixture();
    fs::write(
        root.path().join("src/index.html"),
        r#"<div class="card"></div>"#,
    )
    .unwrap();
    let response = call(
        &McpServer::new(root.path().to_path_buf()),
        110,
        "grimoire_import_css",
        json!({"import_name":"literals", "content":r#"
            .card .item_name[data-label="a_b"] { content: "hello_world"; }
            .card { background-image: url("my_image.png"); --my_var: 1px; width: var(--my_var); }
            .card .escaped\_name { content: "escaped\_value"; }
        "#}),
    );
    assert_eq!(
        response["result"]["structuredContent"]["data"]["valid"], true,
        "{response}"
    );
    let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
    for expected in [
        ".card .item_name[data-label=a_b]",
        "hello_world",
        "my_image.png",
        "var(--my_var)",
        ".card .escaped_name",
        "escaped_value",
    ] {
        assert!(css.contains(expected), "missing {expected}: {css}");
    }
}

#[test]
fn imported_css_keeps_the_last_duplicate_declaration_in_cascade_order() {
    for input in [
        ".card {margin:1px;margin-left:2px;margin:1px}",
        ".card {margin:1px} .card {margin-left:2px} .card {margin:1px}",
        "@media print {.card {margin:1px} .card {margin-left:2px} .card {margin:1px}}",
    ] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            r#"<div class="card"></div>"#,
        )
        .unwrap();
        let response = call(
            &McpServer::new(root.path().to_path_buf()),
            111,
            "grimoire_import_css",
            json!({"import_name":"cascade", "content":input}),
        );
        assert_eq!(
            response["result"]["structuredContent"]["data"]["valid"], true,
            "{response}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        assert!(css.contains(".card{margin:1px}"), "{input}: {css}");
    }
}

#[test]
fn uncallable_scroll_names_are_rejected_before_import_publication() {
    for input in [
        r#".a\:b {color:red}"#,
        r#".\31 23 {color:red}"#,
        ".a__b {color:red}",
        ".foo\u{00a0}bar {color:red}",
        r".foo\a0 bar {color:red}",
        ".foo\u{2003}bar {color:red}",
    ] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            "<div class=\"foo\u{00a0}bar\"></div>",
        )
        .unwrap();
        let path = root
            .path()
            .join("grimoire/config/grimoire.names.scrolls.json");
        let original = br#"{"scrolls":[{"name":"old","spells":["color=blue"]}]}"#;
        let expected_error =
            grimoire_css_lib::transmutator::transmute_css(input, Default::default())
                .unwrap_err()
                .to_string();
        let preview = call(
            &McpServer::new(root.path().to_path_buf()),
            111,
            "grimoire_transmute_css",
            json!({"content":input}),
        );
        assert_eq!(preview["result"]["isError"], true, "{preview}");
        assert_eq!(
            preview["result"]["structuredContent"]["error"],
            expected_error
        );
        for replace in [false, true] {
            if replace {
                fs::write(&path, original).unwrap();
            }
            let response = call(
                &McpServer::new(root.path().to_path_buf()),
                112,
                "grimoire_import_css",
                json!({"import_name":"names", "content":input, "replace":replace}),
            );
            assert_eq!(response["result"]["isError"], true, "{input}: {response}");
            assert_eq!(
                response["result"]["structuredContent"]["error"],
                expected_error
            );
            assert!(
                response["result"]["structuredContent"]["error"]
                    .as_str()
                    .unwrap()
                    .contains("Scroll name"),
                "{response}"
            );
            if replace {
                assert_eq!(fs::read(&path).unwrap(), original);
            } else {
                assert!(!path.exists());
            }
            assert!(!root.path().join("dist/main.css").exists());
        }
    }
}

#[test]
fn migration_preserves_dollar_literals_without_changing_grimoire_variables() {
    let root = fixture();
    fs::write(
        root.path().join("src/index.html"),
        r#"<div class="card"></div>"#,
    )
    .unwrap();
    let server = McpServer::new(root.path().to_path_buf());
    let direct = Analyzer::explain_class_token(root.path(), "color=$primary").unwrap();
    assert!(direct.css.contains("color: red"), "{}", direct.css);
    let response = call(
        &server,
        120,
        "grimoire_import_css",
        json!({
            "import_name":"dollars",
        "content":r#".card::before {content:"$primary"} .card::after {content:"\$primary"} .card {background-image:url("$primary.svg")} .card[data-name$="_end"] {color:red}"#
        }),
    );
    assert_eq!(
        response["result"]["structuredContent"]["data"]["valid"], true,
        "{response}"
    );
    let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
    assert!(css.contains("content:\"$primary\""), "{css}");
    assert!(css.contains("$primary.svg"), "{css}");
    assert!(css.contains("[data-name$=_end]"), "{css}");
    assert!(!css.contains("#ff0000"), "{css}");
}

#[test]
fn migration_accepts_empty_custom_properties_in_the_actual_build() {
    let root = fixture();
    fs::write(
        root.path().join("src/index.html"),
        r#"<div class="card"></div>"#,
    )
    .unwrap();
    let response = call(
        &McpServer::new(root.path().to_path_buf()),
        121,
        "grimoire_import_css",
        json!({
            "import_name":"empty", "content":".card {--empty: ; color:red}"
        }),
    );
    assert_eq!(
        response["result"]["structuredContent"]["data"]["valid"], true,
        "{response}"
    );
    let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
    assert!(css.contains("--empty:"), "{css}");
    assert!(css.contains("color:red"), "{css}");
}

#[test]
fn migration_rejects_component_scroll_collisions_in_preview_and_import() {
    let root = fixture();
    let config_dir = root.path().join("grimoire/config");
    fs::write(
        config_dir.join("grimoire.custom.scrolls.json"),
        r#"{"scrolls":[{"name":"color","spells":[],"spellsByArgs":{"1":["background=$1"]}}]}"#,
    )
    .unwrap();
    let server = McpServer::new(root.path().to_path_buf());
    let direct = Analyzer::explain_class_token(root.path(), "color=red").unwrap();
    assert!(direct.css.contains("background: red"), "{}", direct.css);
    let target = config_dir.join("grimoire.collision.scrolls.json");
    let original = br#"{"scrolls":[{"name":"old","spells":["display=block"]}]}"#;
    for replace in [false, true] {
        if replace {
            fs::write(&target, original).unwrap();
        }
        for tool in ["grimoire_transmute_css", "grimoire_import_css"] {
            let mut args = json!({"content":".card {color:red}"});
            if tool == "grimoire_import_css" {
                args["import_name"] = json!("collision");
                args["replace"] = json!(replace);
            }
            let response = call(&server, 122, tool, args);
            assert_eq!(response["result"]["isError"], true, "{response}");
            assert!(
                response["result"]["structuredContent"]["error"]
                    .as_str()
                    .unwrap()
                    .contains("component/Scroll conflict"),
                "{response}"
            );
        }
        if replace {
            assert_eq!(fs::read(&target).unwrap(), original);
        } else {
            assert!(!target.exists());
        }
    }
}

#[test]
fn replacement_validates_against_the_scrolls_that_will_remain() {
    let root = fixture();
    fs::write(
        root.path().join("src/index.html"),
        r#"<div class="card"></div>"#,
    )
    .unwrap();
    let target = root
        .path()
        .join("grimoire/config/grimoire.replace.scrolls.json");
    fs::write(
        &target,
        r#"{"scrolls":[{"name":"color","spells":["background=blue"]}]}"#,
    )
    .unwrap();
    let response = call(
        &McpServer::new(root.path().to_path_buf()),
        123,
        "grimoire_import_css",
        json!({
            "content":".card {color:red}", "import_name":"replace", "replace":true
        }),
    );
    assert_eq!(
        response["result"]["structuredContent"]["data"]["valid"], true,
        "{response}"
    );
    assert_eq!(
        fs::read_to_string(root.path().join("dist/main.css")).unwrap(),
        ".card{color:red}"
    );
}

#[test]
fn migration_preserves_dollar_delimiter_tokens_in_custom_properties() {
    use lightningcss::{properties::Property, rules::CssRule, stylesheet::StyleSheet};
    let root = fixture();
    fs::write(
        root.path().join("src/index.html"),
        r#"<div class="card"></div>"#,
    )
    .unwrap();
    let source = ".card {--cash:$primary}";
    let response = call(
        &McpServer::new(root.path().to_path_buf()),
        124,
        "grimoire_import_css",
        json!({
            "content":source, "import_name":"cash"
        }),
    );
    assert_eq!(
        response["result"]["structuredContent"]["data"]["valid"], true,
        "{response}"
    );
    let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
    let before = StyleSheet::parse(source, Default::default()).unwrap();
    let after = StyleSheet::parse(&css, Default::default()).unwrap();
    let (CssRule::Style(before), CssRule::Style(after)) = (&before.rules.0[0], &after.rules.0[0])
    else {
        panic!("missing style rule: {css}")
    };
    let (Property::Custom(before), Property::Custom(after)) = (
        &before.declarations.declarations[0],
        &after.declarations.declarations[0],
    ) else {
        panic!("missing custom property: {css}")
    };
    assert_eq!(
        before.value, after.value,
        "CSS token types and values must survive: {css}"
    );
}

#[test]
fn imported_css_preserves_decoded_string_values_and_line_continuations() {
    use lightningcss::{rules::CssRule, stylesheet::StyleSheet};
    for value in [
        "a\tb",
        "a\\\nb",
        "a\\\r\nb",
        "a\\\rb",
        "a\\\u{000c}b",
        "a\\9 b",
        "a\\a b",
        "a\u{00a0}b",
        "a\\\"b",
        "a\\\\b",
        "a_$primary b",
        "_ x",
        "$ x",
        "( x",
    ] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            r#"<div class="card"></div>"#,
        )
        .unwrap();
        let source = format!(".card::before {{content:\"{value}\"}}");
        let response = call(
            &McpServer::new(root.path().to_path_buf()),
            125,
            "grimoire_import_css",
            json!({"content":source,"import_name":"strings"}),
        );
        assert_eq!(
            response["result"]["structuredContent"]["data"]["valid"], true,
            "{source:?}: {response}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        let before = StyleSheet::parse(&source, Default::default()).unwrap();
        let after = StyleSheet::parse(&css, Default::default()).unwrap();
        let (CssRule::Style(before), CssRule::Style(after)) =
            (&before.rules.0[0], &after.rules.0[0])
        else {
            panic!("missing rule: {css}")
        };
        assert_eq!(
            before.declarations.declarations, after.declarations.declarations,
            "string data changed: {source:?} -> {css:?}"
        );
    }
}

#[test]
fn imported_css_preserves_decoded_attribute_string_values() {
    use lightningcss::{rules::CssRule, stylesheet::StyleSheet};
    for value in [
        "a\tb",
        "a\\\nb",
        "a\\\r\nb",
        "a\\\u{000c}b",
        "a\\9 b",
        "a\u{00a0}b",
        "a_b",
    ] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            r#"<div class="card"></div>"#,
        )
        .unwrap();
        let source = format!(".card[data-label=\"{value}\"] {{color:red}}");
        let response = call(
            &McpServer::new(root.path().to_path_buf()),
            126,
            "grimoire_import_css",
            json!({"content":source,"import_name":"attributes"}),
        );
        assert_eq!(
            response["result"]["structuredContent"]["data"]["valid"], true,
            "{source:?}: {response}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        let before = StyleSheet::parse(&source, Default::default()).unwrap();
        let after = StyleSheet::parse(&css, Default::default()).unwrap();
        let (CssRule::Style(before), CssRule::Style(after)) =
            (&before.rules.0[0], &after.rules.0[0])
        else {
            panic!("missing rule: {css}")
        };
        assert_eq!(
            before.selectors, after.selectors,
            "selector string data changed: {source:?} -> {css:?}"
        );
    }
}

#[test]
fn cascade_rejection_is_shared_by_preview_and_import_without_file_changes() {
    for replace in [false, true] {
        let root = fixture();
        let path = root
            .path()
            .join("grimoire/config/grimoire.cascade.scrolls.json");
        let original = br#"{"scrolls":[{"name":"old","spells":["color=red"]}]}"#;
        if replace {
            fs::write(&path, original).unwrap();
        }
        let server = McpServer::new(root.path().to_path_buf());
        let source = ".a{color:red}.b{color:blue}.a{color:green}";
        let expected = grimoire_css_lib::transmutator::transmute_css(source, Default::default())
            .unwrap_err()
            .to_string();
        for tool in ["grimoire_transmute_css", "grimoire_import_css"] {
            let args = if tool == "grimoire_import_css" {
                json!({"content":source,"import_name":"cascade","replace":replace})
            } else {
                json!({"content":source})
            };
            let response = call(&server, 127, tool, args);
            assert_eq!(response["result"]["isError"], true, "{response}");
            assert_eq!(
                response["result"]["structuredContent"]["error"], expected,
                "{response}"
            );
            if replace {
                assert_eq!(fs::read(&path).unwrap(), original);
            } else {
                assert!(!path.exists());
            }
            assert!(!root.path().join("dist/main.css").exists());
        }
    }
}

#[test]
fn order_independent_imports_build_with_either_html_class_order() {
    for classes in ["a b", "b a"] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            format!("<div class=\"{classes}\"></div>"),
        )
        .unwrap();
        let response = call(
            &McpServer::new(root.path().to_path_buf()),
            128,
            "grimoire_import_css",
            json!({"content":".a{color:red}.b{opacity:0.5}.a{color:green}","import_name":"independent"}),
        );
        assert_eq!(
            response["result"]["structuredContent"]["data"]["valid"], true,
            "{response}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        assert!(css.contains(".a{color:green}"), "{classes}: {css}");
        assert!(css.contains(".b{opacity:.5}"), "{classes}: {css}");
    }
}

#[test]
fn ordinary_scroll_order_and_shared_css_fallback_remain_available() {
    for classes in ["a b", "b a"] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            format!("<div class=\"{classes}\"></div>"),
        )
        .unwrap();
        let original_css = root.path().join("legacy.css");
        fs::write(&original_css, ".a{color:red}.b{color:blue}.a{color:green}").unwrap();
        let config_path = root.path().join("grimoire/config/grimoire.config.json");
        let mut config: Value = serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
        config["scrolls"] = json!([
            {"name":"a","spells":["color=red"]},
            {"name":"b","spells":["color=blue"]}
        ]);
        let shared_output = root.path().join("dist/legacy.css");
        config["shared"] = json!([{"outputPath":shared_output,"styles":[original_css]}]);
        fs::write(config_path, serde_json::to_vec_pretty(&config).unwrap()).unwrap();
        build_with_options(root.path(), false).unwrap();
        let ordinary = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        let (a, b) = (ordinary.find(".a{").unwrap(), ordinary.find(".b{").unwrap());
        assert_eq!(
            a < b,
            classes == "a b",
            "ordinary Scroll order changed: {ordinary}"
        );
        let legacy = fs::read_to_string(shared_output).unwrap();
        assert!(
            legacy.find(".b{color:#00f}").unwrap() < legacy.find(".a{color:green}").unwrap(),
            "source cascade changed: {legacy}"
        );
    }
}

#[test]
fn imported_css_preserves_whitespace_characters_inside_identifiers() {
    use lightningcss::{rules::CssRule, stylesheet::StyleSheet};
    for source in [
        ".card .foo\u{00a0}bar{color:red}",
        r".card{font-family:O\'Reilly}",
        r#".card{--name:escaped\"quote}"#,
        ".card .foo\u{00a0}{color:red}",
        ".card .foo\u{2003}bar{color:red}",
        ".card .foo\\\tbar{color:red}",
        ".card .foo\\ bar{color:red}",
        ".card .foo\\ {color:red}",
        ".card{--name:foo\\ }",
        ".card{font-family:foo\u{00a0}bar}",
        ".card{font-family:\u{00a0}foo\u{00a0}}",
        ".card{--name:\u{00a0}foo\u{00a0}}",
    ] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            r#"<div class="card"></div>"#,
        )
        .unwrap();
        let response = call(
            &McpServer::new(root.path().to_path_buf()),
            129,
            "grimoire_import_css",
            json!({"content":source,"import_name":"identifier"}),
        );
        assert_eq!(
            response["result"]["structuredContent"]["data"]["valid"], true,
            "{source:?}: {response}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        let before = StyleSheet::parse(source, Default::default()).unwrap();
        let after = StyleSheet::parse(&css, Default::default()).unwrap();
        let (CssRule::Style(before), CssRule::Style(after)) =
            (&before.rules.0[0], &after.rules.0[0])
        else {
            panic!("missing style: {css}")
        };
        assert_eq!(before.selectors, after.selectors, "{source:?} -> {css:?}");
        assert_eq!(
            before.declarations.declarations, after.declarations.declarations,
            "{source:?} -> {css:?}"
        );
    }
}

#[test]
fn relative_url_file_import_rejects_before_publication_or_replacement() {
    for replace in [false, true] {
        let root = fixture();
        let styles = root.path().join("src/styles");
        fs::create_dir_all(&styles).unwrap();
        fs::write(
            styles.join("source.css"),
            r#".card{background-image:url("./image.png")}"#,
        )
        .unwrap();
        fs::write(styles.join("image.png"), b"image fixture").unwrap();
        let target = root
            .path()
            .join("grimoire/config/grimoire.urls.scrolls.json");
        let previous = br#"{"scrolls":[{"name":"old","spells":["color=red"]}]}"#;
        if replace {
            fs::write(&target, previous).unwrap();
        }
        let response = call(
            &McpServer::new(root.path().to_path_buf()),
            130,
            "grimoire_import_css",
            json!({"paths":["src/styles/source.css"],"import_name":"urls","replace":replace}),
        );
        assert_eq!(response["result"]["isError"], true, "{response}");
        assert!(
            response["result"]["structuredContent"]["error"]
                .as_str()
                .unwrap()
                .contains("URL"),
            "{response}"
        );
        if replace {
            assert_eq!(fs::read(&target).unwrap(), previous);
        } else {
            assert!(!target.exists());
        }
        assert!(!root.path().join("dist/main.css").exists());
    }
}

#[test]
fn imported_css_keeps_function_lookalikes_as_css_data() {
    use lightningcss::{rules::CssRule, stylesheet::StyleSheet};
    for declaration in [
        r#"content: "mfs(14px 16px 380px 800px)""#,
        r#"content: 'mrs(14px 16px 380px 800px)'"#,
        r#"content: "mfs(invalid) mrs(invalid)""#,
        r#"content: "m\66 s(14px 16px 380px 800px)""#,
        "--value: g-invert(#ffffff)",
        "--value: g-lighten(red 10)",
        "--value: g-invert(invalid)",
        "--value: mfs(14px 16px 380px 800px)",
        "--value: mrs(14px 16px 380px 800px)",
        "--value: mfs(invalid)",
        "--value: vendor-mfs(invalid)",
        r"--value: \61 mrs(invalid)",
        "--value: var(--other, mfs(invalid))",
        r"--value: mf\73 (14px 16px 380px 800px)",
        "--value: g-future(1)",
        "--value: calc(1px + 2px)",
        "--value: a_ b",
        "--value: a\u{00a0} b",
    ] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            r#"<div class="card"></div>"#,
        )
        .unwrap();
        let source = format!(".card::before {{{declaration}}}");
        let response = call(
            &McpServer::new(root.path().to_path_buf()),
            130,
            "grimoire_import_css",
            json!({"content":source,"import_name":"functions"}),
        );
        assert_eq!(
            response["result"]["structuredContent"]["data"]["valid"], true,
            "{source}: {response}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        let before = StyleSheet::parse(&source, Default::default()).unwrap();
        let after = StyleSheet::parse(&css, Default::default()).unwrap();
        assert_eq!(after.rules.0.len(), 1, "unexpected generated rules: {css}");
        let (CssRule::Style(before), CssRule::Style(after)) =
            (&before.rules.0[0], &after.rules.0[0])
        else {
            panic!("missing rule: {css}")
        };
        assert_eq!(
            before.declarations.declarations, after.declarations.declarations,
            "CSS data changed: {source} -> {css}"
        );
    }
}

#[test]
fn explicit_grimoire_function_calls_still_evaluate() {
    let root = fixture();
    let result = Analyzer::validate_spells(
        root.path(),
        &[
            "width=mfs(14px_16px_380px_800px)".into(),
            "width=mrs(14px_16px_380px_800px)".into(),
            "color=g-invert(#ffffff)".into(),
        ],
    )
    .unwrap();
    let result = serde_json::to_value(result).unwrap();
    assert_eq!(result["valid"], true, "{result}");
    assert!(
        result["items"][0]["css"]
            .as_str()
            .unwrap()
            .contains("clamp(")
    );
    assert!(
        result["items"][1]["css"]
            .as_str()
            .unwrap()
            .contains("@media")
    );
    assert!(result["items"][2]["css"].as_str().unwrap().contains("#000"));
}

#[test]
fn migrated_animation_references_do_not_inject_grimoire_keyframes() {
    use lightningcss::{rules::CssRule, stylesheet::StyleSheet};
    for declaration in [
        "animation: bounce-bottom 1s",
        "animation-name: bounce-bottom",
        "animation: 1s ease-in project-animation",
        "animation-name: project-animation, bounce-bottom",
        r#"animation-name: "hello bounce-bottom world""#,
        "animation: var(--motion, bounce-bottom 1s )",
        r"animation-name: bounce\2d bottom",
        "animation: none 1s steps(2, end)",
    ] {
        let root = fixture();
        fs::create_dir_all(root.path().join("grimoire/animations")).unwrap();
        fs::write(
            root.path()
                .join("grimoire/animations/project-animation.css"),
            "@keyframes project-animation { from { opacity: 0 } to { opacity: 1 } } .GRIMOIRE_CSS_ANIMATION { animation-name: project-animation; }",
        )
        .unwrap();
        fs::write(
            root.path().join("src/index.html"),
            r#"<div class="card"></div>"#,
        )
        .unwrap();
        let source = format!(".card {{{declaration}}}");
        let result = call(
            &McpServer::new(root.path().to_path_buf()),
            140,
            "grimoire_import_css",
            json!({"content":source,"import_name":"animations"}),
        );
        assert_eq!(
            result["result"]["structuredContent"]["data"]["valid"], true,
            "{source}: {result}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        let original = StyleSheet::parse(&source, Default::default()).unwrap();
        // The optimizer may serialize quoted animation names as equivalent idents.
        let normalized = original
            .to_css(lightningcss::stylesheet::PrinterOptions {
                minify: true,
                ..Default::default()
            })
            .unwrap()
            .code;
        let before = StyleSheet::parse(&normalized, Default::default()).unwrap();
        let after = StyleSheet::parse(&css, Default::default()).unwrap();
        assert_eq!(after.rules.0.len(), 1, "unexpected injected rules: {css}");
        let (CssRule::Style(before), CssRule::Style(after)) =
            (&before.rules.0[0], &after.rules.0[0])
        else {
            panic!("missing style: {css}")
        };
        assert_eq!(
            before.declarations.declarations, after.declarations.declarations,
            "{source} -> {css}"
        );
    }
}

#[test]
fn explicit_grimoire_animations_still_include_their_keyframes() {
    let root = fixture();
    fs::create_dir_all(root.path().join("grimoire/animations")).unwrap();
    fs::write(
        root.path()
            .join("grimoire/animations/project-animation.css"),
        "@keyframes project-animation { from { opacity: 0 } to { opacity: 1 } } .GRIMOIRE_CSS_ANIMATION { animation-name: project-animation; }",
    )
    .unwrap();
    for (token, name) in [
        ("animation=bounce-bottom_1s", "bounce-bottom"),
        ("animation-name=project-animation", "project-animation"),
    ] {
        let result = Analyzer::explain_class_token(root.path(), token).unwrap();
        let value = serde_json::to_value(result).unwrap();
        assert!(
            value["css"]
                .as_str()
                .unwrap()
                .contains(&format!("@keyframes {name}")),
            "{value}"
        );
    }
}

#[test]
fn migrated_property_names_follow_css_case_rules() {
    let root = fixture();
    fs::write(
        root.path().join("src/index.html"),
        r#"<div class="card"></div>"#,
    )
    .unwrap();
    let result = call(
        &McpServer::new(root.path().to_path_buf()),
        141,
        "grimoire_import_css",
        json!({"content":".card { COLOR: red; DiSpLaY: flex; --Primary: red; --primary: blue; }","import_name":"case"}),
    );
    assert_eq!(
        result["result"]["structuredContent"]["data"]["valid"], true,
        "{result}"
    );
    let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
    for expected in [
        "color:red",
        "display:flex",
        "--Primary:red",
        "--primary:blue",
    ] {
        assert!(css.contains(expected), "{css}");
    }
}

#[test]
fn css_property_aliases_are_rejected_before_import_publication() {
    for property in ["c", "W", "disp", "anim", "g-anim"] {
        let source = format!(".card {{{property}:initial}}");
        let root = fixture();
        let target = root
            .path()
            .join("grimoire/config/grimoire.alias.scrolls.json");
        let original = br#"{"scrolls":[{"name":"previous","spells":["color=red"]}]}"#;
        fs::write(&target, original).unwrap();
        let server = McpServer::new(root.path().to_path_buf());
        for tool in ["grimoire_transmute_css", "grimoire_import_css"] {
            let args = if tool == "grimoire_import_css" {
                json!({"content":source,"import_name":"alias","replace":true})
            } else {
                json!({"content":source})
            };
            let result = call(&server, 150, tool, args);
            assert_eq!(result["result"]["isError"], true, "{source}: {result}");
            assert!(
                result["result"]["structuredContent"]["error"]
                    .as_str()
                    .unwrap()
                    .contains("CSS property"),
                "{result}"
            );
            assert_eq!(fs::read(&target).unwrap(), original);
            assert!(!root.path().join("dist/main.css").exists());
        }
    }
}

#[test]
fn migrated_media_types_do_not_activate_grimoire_breakpoints() {
    use lightningcss::{rules::CssRule, stylesheet::StyleSheet, traits::ToCss};
    for name in ["sm", "md", "lg", "xl"] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            "<div class=\"card\"></div>",
        )
        .unwrap();
        let result = call(
            &McpServer::new(root.path().to_path_buf()),
            151,
            "grimoire_import_css",
            json!({"content":format!("@media {name} {{.card{{color:red}}}}"),"import_name":"media"}),
        );
        assert_eq!(
            result["result"]["structuredContent"]["data"]["valid"], true,
            "{result}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        let sheet = StyleSheet::parse(&css, Default::default()).unwrap();
        let CssRule::Media(media) = &sheet.rules.0[0] else {
            panic!("{css}")
        };
        assert_eq!(
            media.query.to_css_string(Default::default()).unwrap(),
            name,
            "{css}"
        );
    }
}

#[test]
fn authored_spells_preserve_escapes_aliases_and_breakpoints() {
    let root = fixture();
    let tokens = [
        r"font-family=O\'Reilly",
        r#"--name=escaped\"quote"#,
        "c=red",
        "w=10px",
        "sm__disp=block",
    ];
    let result = Analyzer::validate_spells(root.path(), &tokens.map(str::to_string)).unwrap();
    assert!(result.valid, "{result:?}");
    assert!(result.items[2].css.as_ref().unwrap().contains("color:"));
    assert!(result.items[3].css.as_ref().unwrap().contains("width:"));
    assert!(result.items[4].css.as_ref().unwrap().contains("640px"));
    let invalid = Analyzer::validate_spells(
        root.path(),
        &["content='unclosed".into(), "width=calc(1px".into()],
    )
    .unwrap();
    assert!(invalid.items.iter().all(|item| !item.valid));
}

#[test]
fn imported_unicode_scroll_names_are_found_in_html_and_built() {
    use lightningcss::{rules::CssRule, selector::Component, stylesheet::StyleSheet};
    for name in ["карточка", "café", "卡片", "foo\u{200b}bar"] {
        let root = fixture();
        fs::write(
            root.path().join("src/index.html"),
            format!("<div class=\"{name}\"></div>"),
        )
        .unwrap();
        let result = call(
            &McpServer::new(root.path().to_path_buf()),
            160,
            "grimoire_import_css",
            json!({"content":format!(".{name}{{color:red}}"),"import_name":"unicode"}),
        );
        assert_eq!(
            result["result"]["structuredContent"]["data"]["valid"], true,
            "{result}"
        );
        let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
        let sheet = StyleSheet::parse(&css, Default::default()).unwrap();
        assert_eq!(sheet.rules.0.len(), 1, "{name}: {css}");
        let CssRule::Style(style) = &sheet.rules.0[0] else {
            panic!("{css}")
        };
        assert!(
            matches!(style.selectors.0[0].iter_raw_match_order().next(), Some(Component::Class(class)) if class.0.as_ref() == name),
            "{css}"
        );
        assert!(css.contains("color:red"), "{css}");
    }
}

#[test]
fn unsupported_custom_property_names_cannot_publish_or_replace_an_import() {
    let source = r".card{--foo\=bar:red;color:var(--foo\=bar)}";
    let expected_error = grimoire_css_lib::transmutator::transmute_css(source, Default::default())
        .unwrap_err()
        .to_string();
    let root = fixture();
    fs::write(
        root.path().join("src/index.html"),
        "<div class=\"card\"></div>",
    )
    .unwrap();
    let target = root
        .path()
        .join("grimoire/config/grimoire.property.scrolls.json");
    let original = br#"{"scrolls":[{"name":"old","spells":["color=blue"]}]}"#;
    let server = McpServer::new(root.path().to_path_buf());
    for replace in [false, true] {
        if replace {
            fs::write(&target, original).unwrap();
        }
        for tool in ["grimoire_transmute_css", "grimoire_import_css"] {
            let args = if tool == "grimoire_transmute_css" {
                json!({"content":source})
            } else {
                json!({"content":source,"import_name":"property","replace":replace})
            };
            let result = call(&server, 170, tool, args);
            assert_eq!(result["result"]["isError"], true, "{result}");
            assert_eq!(
                result["result"]["structuredContent"]["error"],
                expected_error
            );
            assert!(
                result["result"]["structuredContent"]["error"]
                    .as_str()
                    .unwrap()
                    .contains("custom property name"),
                "{result}"
            );
            if replace {
                assert_eq!(fs::read(&target).unwrap(), original);
            } else {
                assert!(!target.exists());
            }
            assert!(!root.path().join("dist/main.css").exists());
        }
    }
}

#[test]
fn imported_custom_property_names_and_references_survive_build() {
    use lightningcss::{rules::CssRule, stylesheet::StyleSheet};
    let source = r".card{--Foo:red;--foo_bar:blue;--foo--bar:green;--fo\6f :black;color:var(--Foo);background-color:var(--foo_bar);border-color:var(--foo--bar);outline-color:var(--fo\6f )}";
    let root = fixture();
    fs::write(
        root.path().join("src/index.html"),
        r#"<div class="card"></div>"#,
    )
    .unwrap();
    let response = call(
        &McpServer::new(root.path().to_path_buf()),
        171,
        "grimoire_import_css",
        json!({"content":source,"import_name":"properties"}),
    );
    assert_eq!(
        response["result"]["structuredContent"]["data"]["valid"], true,
        "{response}"
    );
    let css = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
    let before = StyleSheet::parse(source, Default::default()).unwrap();
    let after = StyleSheet::parse(&css, Default::default()).unwrap();
    let (CssRule::Style(before), CssRule::Style(after)) = (&before.rules.0[0], &after.rules.0[0])
    else {
        panic!("missing style: {css}")
    };
    assert_eq!(
        before.declarations.declarations, after.declarations.declarations,
        "{css}"
    );
}

#[test]
fn commented_scroll_prefixes_keep_rules_in_mcp_validation_and_build() {
    for (spell, expected) in [
        ("{/*don't*/_p}color=red", ".card p{color:red}"),
        (
            "(min-width:600px)/*don't*/__color=red",
            "@media (width>=600px){.card{color:red}}",
        ),
    ] {
        let root = fixture();
        let path = root.path().join("grimoire/config/grimoire.config.json");
        let mut config: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        config["scrolls"] = json!([{"name":"card", "spells":[spell]}]);
        fs::write(path, serde_json::to_vec(&config).unwrap()).unwrap();
        fs::write(
            root.path().join("src/index.html"),
            "<div class=\"card\"><p>Test</p></div>",
        )
        .unwrap();
        let server = McpServer::new(root.path().to_path_buf());
        let response = call(
            &server,
            172,
            "grimoire_validate_spells",
            json!({"tokens":["card"]}),
        );
        let data = &response["result"]["structuredContent"]["data"];
        assert_eq!(data["valid"], true, "{response}");
        assert_eq!(data["items"][0]["expanded_spells"], json!([spell]));
        assert!(
            data["items"][0]["css"]
                .as_str()
                .unwrap()
                .contains("color: red"),
            "{response}"
        );
        let response = call(&server, 173, "grimoire_build", json!({}));
        assert_eq!(response["result"]["isError"], false, "{response}");
        assert_eq!(
            fs::read_to_string(root.path().join("dist/main.css")).unwrap(),
            expected
        );
    }
}
