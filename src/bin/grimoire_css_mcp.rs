use grimoire_css_lib::mcp::McpServer;
use serde_json::Value;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args_os().skip(1);
    let root = match (args.next(), args.next(), args.next()) {
        (None, None, None) => std::env::current_dir(),
        (Some(flag), Some(root), None) if flag == "--root" => Ok(PathBuf::from(root)),
        _ => {
            eprintln!("usage: grimoire_css_mcp [--root PROJECT_ROOT]");
            std::process::exit(2);
        }
    };
    let server = match root {
        Ok(root) => McpServer::new(root),
        Err(error) => {
            eprintln!("cannot determine MCP project root: {error}");
            std::process::exit(1);
        }
    };

    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines() {
        let response = match line {
            Ok(line) => match serde_json::from_str::<Value>(&line) {
                Ok(request) => server.dispatch(request),
                Err(_) => Some(serde_json::json!({
                    "jsonrpc":"2.0",
                    "id":null,
                    "error":{"code":-32700,"message":"Parse error"}
                })),
            },
            Err(_) => break,
        };
        if let Some(response) = response
            && (serde_json::to_writer(&mut stdout, &response).is_err()
                || stdout.write_all(b"\n").is_err()
                || stdout.flush().is_err())
        {
            break;
        }
    }
}
