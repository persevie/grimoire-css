#![cfg(feature = "lsp")]

use serde_json::Value;
use tempfile::tempdir;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
    time::{Duration, timeout},
};

fn write_repo_for_lsp() -> tempfile::TempDir {
    let dir = tempdir().expect("tempdir");

    let html = dir.path().join("src/index.html");
    std::fs::create_dir_all(html.parent().unwrap()).unwrap();
    std::fs::write(&html, r#"<div class="display=flex"></div>"#).unwrap();

    std::fs::create_dir_all(dir.path().join("grimoire/config")).unwrap();
    std::fs::write(
        dir.path().join("grimoire/config/grimoire.config.json"),
        format!(
            "{{\n  \"projects\": [{{\n    \"projectName\": \"main\",\n    \"inputPaths\": [\"{}\"]\n  }}]\n}}",
            html.to_string_lossy()
        ),
    )
    .unwrap();

    dir
}

fn write_repo_without_config_for_lsp() -> tempfile::TempDir {
    let dir = tempdir().expect("tempdir");

    let html = dir.path().join("src/index.html");
    std::fs::create_dir_all(html.parent().unwrap()).unwrap();
    std::fs::write(&html, r#"<div class=\"display=flex\"></div>"#).unwrap();

    dir
}

fn lsp_message(body: &Value) -> Vec<u8> {
    let json = serde_json::to_vec(body).expect("json");
    let mut out = Vec::new();
    out.extend_from_slice(format!("Content-Length: {}\r\n\r\n", json.len()).as_bytes());
    out.extend_from_slice(&json);
    out
}

async fn read_one_lsp_message(stdout: &mut tokio::process::ChildStdout) -> Value {
    let mut header = Vec::<u8>::new();

    loop {
        let mut b = [0u8; 1];
        stdout.read_exact(&mut b).await.expect("read header byte");
        header.push(b[0]);
        if header.len() >= 4 && &header[header.len() - 4..] == b"\r\n\r\n" {
            break;
        }
        assert!(header.len() < 16 * 1024, "header too large");
    }

    let header_str = String::from_utf8_lossy(&header);
    let mut content_len: usize = 0;
    for line in header_str.split("\r\n") {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Content-Length:") {
            content_len = rest.trim().parse::<usize>().expect("content length");
        }
    }
    assert!(content_len > 0, "missing Content-Length");

    let mut body = vec![0u8; content_len];
    stdout.read_exact(&mut body).await.expect("read body");
    serde_json::from_slice(&body).expect("json body")
}

async fn read_response_by_id(stdout: &mut tokio::process::ChildStdout, id: i64) -> Value {
    loop {
        let msg = timeout(Duration::from_secs(3), read_one_lsp_message(stdout))
            .await
            .expect("timeout");

        if msg.get("id").and_then(|v| v.as_i64()) == Some(id) {
            return msg;
        }
        // Ignore notifications / other responses.
    }
}

#[tokio::test]
async fn lsp_stdio_initialize_advertises_canonical_commands() {
    let repo = write_repo_for_lsp();
    let exe = env!("CARGO_BIN_EXE_grimoire_css_lsp");

    let mut child = Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lsp");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    let root_uri = format!("file://{}", repo.path().to_string_lossy());

    let init = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "rootUri": root_uri,
            "capabilities": {}
        }
    });

    stdin.write_all(&lsp_message(&init)).await.unwrap();
    let resp = read_response_by_id(&mut stdout, 1).await;

    let cmds = resp["result"]["capabilities"]["executeCommandProvider"]["commands"]
        .as_array()
        .expect("commands array");

    assert!(cmds.iter().any(|v| v == "grimoirecss.explain"));
    assert!(cmds.iter().any(|v| v == "grimoirecss.refs"));
    assert!(cmds.iter().any(|v| v == "grimoirecss.stats"));

    let shutdown = serde_json::json!({"jsonrpc":"2.0","id": 2, "method":"shutdown","params": null});
    stdin.write_all(&lsp_message(&shutdown)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 2).await;

    let exit = serde_json::json!({"jsonrpc":"2.0","method":"exit","params": null});
    stdin.write_all(&lsp_message(&exit)).await.unwrap();

    let _ = timeout(Duration::from_secs(2), child.wait()).await;
}

#[tokio::test]
async fn lsp_stdio_execute_command_refs_and_stats_work() {
    let repo = write_repo_for_lsp();
    let exe = env!("CARGO_BIN_EXE_grimoire_css_lsp");

    let mut child = Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lsp");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    let root_uri = format!("file://{}", repo.path().to_string_lossy());

    let init = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "rootUri": root_uri,
            "capabilities": {}
        }
    });
    stdin.write_all(&lsp_message(&init)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 1).await;

    let refs = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 10,
        "method": "workspace/executeCommand",
        "params": {
            "command": "grimoirecss.refs",
            "arguments": ["display=flex", true]
        }
    });
    stdin.write_all(&lsp_message(&refs)).await.unwrap();
    let resp = read_response_by_id(&mut stdout, 10).await;
    assert_eq!(resp["result"]["query"], "display=flex");

    let stats = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 11,
        "method": "workspace/executeCommand",
        "params": {
            "command": "grimoirecss.stats",
            "arguments": [{"token":"display=flex","silent": true, "top": 10}]
        }
    });
    stdin.write_all(&lsp_message(&stats)).await.unwrap();
    let resp = read_response_by_id(&mut stdout, 11).await;
    assert_eq!(resp["result"]["token"]["kind"], "spell");
    assert_eq!(resp["result"]["token"]["name"], "display=flex");
    assert_eq!(resp["result"]["token"]["count"], 1);

    let shutdown = serde_json::json!({"jsonrpc":"2.0","id": 2, "method":"shutdown","params": null});
    stdin.write_all(&lsp_message(&shutdown)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 2).await;

    let exit = serde_json::json!({"jsonrpc":"2.0","method":"exit","params": null});
    stdin.write_all(&lsp_message(&exit)).await.unwrap();

    let _ = timeout(Duration::from_secs(2), child.wait()).await;
}

#[tokio::test]
async fn lsp_stdio_execute_command_unknown_returns_error() {
    let repo = write_repo_for_lsp();
    let exe = env!("CARGO_BIN_EXE_grimoire_css_lsp");

    let mut child = Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lsp");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    let root_uri = format!("file://{}", repo.path().to_string_lossy());

    let init = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "rootUri": root_uri,
            "capabilities": {}
        }
    });
    stdin.write_all(&lsp_message(&init)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 1).await;

    let unknown = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 99,
        "method": "workspace/executeCommand",
        "params": {
            "command": "grimoirecss.__unknown",
            "arguments": []
        }
    });
    stdin.write_all(&lsp_message(&unknown)).await.unwrap();
    let resp = read_response_by_id(&mut stdout, 99).await;

    assert!(
        resp.get("error").is_some(),
        "expected JSON-RPC error response"
    );
    assert_eq!(resp["error"]["code"].as_i64(), Some(-32601));

    let shutdown = serde_json::json!({"jsonrpc":"2.0","id": 2, "method":"shutdown","params": null});
    stdin.write_all(&lsp_message(&shutdown)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 2).await;

    let exit = serde_json::json!({"jsonrpc":"2.0","method":"exit","params": null});
    stdin.write_all(&lsp_message(&exit)).await.unwrap();

    let _ = timeout(Duration::from_secs(2), child.wait()).await;
}

#[tokio::test]
async fn lsp_stdio_execute_command_explain_missing_token_is_invalid_params() {
    let repo = write_repo_for_lsp();
    let exe = env!("CARGO_BIN_EXE_grimoire_css_lsp");

    let mut child = Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lsp");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    let root_uri = format!("file://{}", repo.path().to_string_lossy());

    let init = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "rootUri": root_uri,
            "capabilities": {}
        }
    });
    stdin.write_all(&lsp_message(&init)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 1).await;

    let explain = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 100,
        "method": "workspace/executeCommand",
        "params": {
            "command": "grimoirecss.explain",
            "arguments": []
        }
    });
    stdin.write_all(&lsp_message(&explain)).await.unwrap();
    let resp = read_response_by_id(&mut stdout, 100).await;

    assert!(
        resp.get("error").is_some(),
        "expected JSON-RPC error response"
    );
    assert_eq!(resp["error"]["code"].as_i64(), Some(-32602));

    let shutdown = serde_json::json!({"jsonrpc":"2.0","id": 2, "method":"shutdown","params": null});
    stdin.write_all(&lsp_message(&shutdown)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 2).await;

    let exit = serde_json::json!({"jsonrpc":"2.0","method":"exit","params": null});
    stdin.write_all(&lsp_message(&exit)).await.unwrap();

    let _ = timeout(Duration::from_secs(2), child.wait()).await;
}

#[tokio::test]
async fn lsp_stdio_execute_command_refs_missing_query_is_invalid_params() {
    let repo = write_repo_for_lsp();
    let exe = env!("CARGO_BIN_EXE_grimoire_css_lsp");

    let mut child = Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lsp");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    let root_uri = format!("file://{}", repo.path().to_string_lossy());

    let init = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "rootUri": root_uri,
            "capabilities": {}
        }
    });
    stdin.write_all(&lsp_message(&init)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 1).await;

    let refs = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 101,
        "method": "workspace/executeCommand",
        "params": {
            "command": "grimoirecss.refs",
            "arguments": []
        }
    });
    stdin.write_all(&lsp_message(&refs)).await.unwrap();
    let resp = read_response_by_id(&mut stdout, 101).await;

    assert!(
        resp.get("error").is_some(),
        "expected JSON-RPC error response"
    );
    assert_eq!(resp["error"]["code"].as_i64(), Some(-32602));

    let shutdown = serde_json::json!({"jsonrpc":"2.0","id": 2, "method":"shutdown","params": null});
    stdin.write_all(&lsp_message(&shutdown)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 2).await;

    let exit = serde_json::json!({"jsonrpc":"2.0","method":"exit","params": null});
    stdin.write_all(&lsp_message(&exit)).await.unwrap();

    let _ = timeout(Duration::from_secs(2), child.wait()).await;
}

#[tokio::test]
async fn lsp_stdio_execute_command_stats_unknown_token_returns_hint_object() {
    let repo = write_repo_for_lsp();
    let exe = env!("CARGO_BIN_EXE_grimoire_css_lsp");

    let mut child = Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lsp");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    let root_uri = format!("file://{}", repo.path().to_string_lossy());

    let init = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "rootUri": root_uri,
            "capabilities": {}
        }
    });
    stdin.write_all(&lsp_message(&init)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 1).await;

    let stats = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 102,
        "method": "workspace/executeCommand",
        "params": {
            "command": "grimoirecss.stats",
            "arguments": [{"token":"definitely-not-a-real-token","silent": true}]
        }
    });
    stdin.write_all(&lsp_message(&stats)).await.unwrap();
    let resp = read_response_by_id(&mut stdout, 102).await;

    assert!(resp.get("error").is_none(), "expected successful response");
    assert_eq!(resp["result"]["token"]["error"], "Unknown token");
    assert!(
        resp["result"]["token"]["hint"]
            .as_str()
            .unwrap_or("")
            .contains("scroll")
    );

    let shutdown = serde_json::json!({"jsonrpc":"2.0","id": 2, "method":"shutdown","params": null});
    stdin.write_all(&lsp_message(&shutdown)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 2).await;

    let exit = serde_json::json!({"jsonrpc":"2.0","method":"exit","params": null});
    stdin.write_all(&lsp_message(&exit)).await.unwrap();

    let _ = timeout(Duration::from_secs(2), child.wait()).await;
}

#[tokio::test]
async fn lsp_stdio_execute_command_requires_workspace_root() {
    let exe = env!("CARGO_BIN_EXE_grimoire_css_lsp");

    let mut child = Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lsp");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    // Intentionally skip `initialize` so the server has no workspace root.
    let cases: &[(&str, Value)] = &[
        ("grimoirecss.listScrolls", serde_json::json!([])),
        ("grimoirecss.listVars", serde_json::json!([])),
        ("grimoirecss.index", serde_json::json!([])),
        ("grimoirecss.lint", serde_json::json!([])),
        ("grimoirecss.configSummary", serde_json::json!([])),
        ("grimoirecss.dryCandidates", serde_json::json!([])),
        ("grimoirecss.refs", serde_json::json!(["display=flex"])),
        ("grimoirecss.explain", serde_json::json!(["display=flex"])),
        (
            "grimoirecss.stats",
            serde_json::json!([{"token":"display=flex"}]),
        ),
    ];

    for (i, (command, arguments)) in cases.iter().enumerate() {
        let id = 2000 + i as i64;
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "workspace/executeCommand",
            "params": {
                "command": command,
                "arguments": arguments
            }
        });
        stdin.write_all(&lsp_message(&req)).await.unwrap();
        let resp = read_response_by_id(&mut stdout, id).await;
        assert!(resp.get("error").is_some(), "expected error for {command}");
        let code = resp["error"]["code"].as_i64();
        assert!(
            matches!(code, Some(-32002) | Some(-32602)),
            "unexpected error code for {command}: {code:?}"
        );
        let msg = resp["error"]["message"].as_str().unwrap_or("");
        assert!(
            msg.contains("initialized") || msg.contains("workspace root"),
            "unexpected error message for {command}: {msg}"
        );
    }

    // Send exit; don't block the test if the server doesn't terminate immediately.
    let exit = serde_json::json!({"jsonrpc":"2.0","method":"exit","params": null});
    stdin.write_all(&lsp_message(&exit)).await.unwrap();
    let _ = timeout(Duration::from_secs(2), child.wait()).await;
}

#[tokio::test]
async fn lsp_stdio_execute_command_missing_config_is_internal_error() {
    let repo = write_repo_without_config_for_lsp();
    let exe = env!("CARGO_BIN_EXE_grimoire_css_lsp");

    let mut child = Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lsp");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    let root_uri = format!("file://{}", repo.path().to_string_lossy());

    let init = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "rootUri": root_uri,
            "capabilities": {}
        }
    });
    stdin.write_all(&lsp_message(&init)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 1).await;

    // Commands that are expected to require a repo config.
    let cases: &[(&str, Value)] = &[
        ("grimoirecss.listScrolls", serde_json::json!([])),
        ("grimoirecss.listVars", serde_json::json!([])),
        ("grimoirecss.index", serde_json::json!([50])),
        ("grimoirecss.lint", serde_json::json!([true])),
        ("grimoirecss.configSummary", serde_json::json!([true])),
        ("grimoirecss.dryCandidates", serde_json::json!([2, 2, true])),
        (
            "grimoirecss.refs",
            serde_json::json!(["display=flex", true]),
        ),
        (
            "grimoirecss.stats",
            serde_json::json!([{"token":"display=flex","silent": true}]),
        ),
    ];

    for (i, (command, arguments)) in cases.iter().enumerate() {
        let id = 3000 + i as i64;
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "workspace/executeCommand",
            "params": {
                "command": command,
                "arguments": arguments
            }
        });
        stdin.write_all(&lsp_message(&req)).await.unwrap();
        let resp = read_response_by_id(&mut stdout, id).await;

        assert!(resp.get("error").is_some(), "expected error for {command}");
        assert_eq!(resp["error"]["code"].as_i64(), Some(-32603));

        // The server includes the underlying error string in `error.data.error`.
        let underlying = resp["error"]["data"]["error"].as_str().unwrap_or("");
        assert!(
            !underlying.is_empty(),
            "expected error.data.error for {command}"
        );
        assert!(
            underlying.contains("config")
                || underlying.contains("grimoire")
                || underlying.contains("No such file")
                || underlying.contains("os error"),
            "unexpected underlying error for {command}: {underlying}"
        );
    }

    let shutdown = serde_json::json!({"jsonrpc":"2.0","id": 2, "method":"shutdown","params": null});
    stdin.write_all(&lsp_message(&shutdown)).await.unwrap();
    let _ = read_response_by_id(&mut stdout, 2).await;

    let exit = serde_json::json!({"jsonrpc":"2.0","method":"exit","params": null});
    stdin.write_all(&lsp_message(&exit)).await.unwrap();

    let _ = timeout(Duration::from_secs(2), child.wait()).await;
}
