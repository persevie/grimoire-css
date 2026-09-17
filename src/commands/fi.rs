#[cfg(feature = "analyzer")]
use crate::analyzer::Analyzer;
use crate::{buffer::add_message, commands::shorten::shorten, core::GrimoireCssError};

use serde_json::json;
#[cfg(feature = "analyzer")]
use std::path::Path;
use std::path::PathBuf;

fn take_flag_value(args: &mut Vec<String>, flag: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == flag)?;
    if pos + 1 >= args.len() {
        return None;
    }
    args.remove(pos);
    Some(args.remove(pos))
}

/// `fi` command runner.
///
/// Intended to be scriptable. If `--json` is provided, prints **only JSON**.
pub fn run_fi_cli(mut args: Vec<String>) -> Result<(), GrimoireCssError> {
    // args: [bin, "fi", ...]
    let _bin = args
        .first()
        .cloned()
        .unwrap_or_else(|| "grimoire_css".to_string());

    // Remove bin + mode.
    if !args.is_empty() {
        args.remove(0);
    }
    if !args.is_empty() {
        args.remove(0);
    }

    let json_output = if let Some(pos) = args.iter().position(|a| a == "--json") {
        args.remove(pos);
        true
    } else {
        false
    };

    let cwd = std::env::current_dir()?;
    let current_dir = if let Some(root) = take_flag_value(&mut args, "--root") {
        let p = PathBuf::from(root);
        if p.is_absolute() { p } else { cwd.join(p) }
    } else {
        cwd
    };

    #[cfg(feature = "analyzer")]
    let top: usize = take_flag_value(&mut args, "--top")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(30);

    #[cfg(feature = "analyzer")]
    let min_support: usize = take_flag_value(&mut args, "--min-support")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(3);
    #[cfg(feature = "analyzer")]
    let min_items: usize = take_flag_value(&mut args, "--min-items")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(2);

    #[cfg(feature = "analyzer")]
    let token_filter = take_flag_value(&mut args, "--token");

    // Subcommand parsing.
    let command = args.first().cloned().unwrap_or_else(|| "help".to_string());

    let result: Result<serde_json::Value, GrimoireCssError> = match command.as_str() {
        "shorten" => {
            shorten(&current_dir)?;
            Ok(json!({"ok": true}))
        }

        #[cfg(feature = "analyzer")]
        "config" => {
            ensure_root_has_config(&current_dir)?;
            let sub = args.get(1).map(|s| s.as_str()).unwrap_or("");
            match sub {
                "summary" => Analyzer::config_summary(&current_dir)
                    .map(|res| serde_json::to_value(res).unwrap()),
                _ => Ok(json!({
                    "error": "Unknown config target",
                    "help": "Usage: grimoire_css fi config summary [--json]"
                })),
            }
        }

        #[cfg(feature = "analyzer")]
        "index" => {
            ensure_root_has_config(&current_dir)?;
            Analyzer::index(&current_dir, top).map(|res| serde_json::to_value(res).unwrap())
        }

        #[cfg(feature = "analyzer")]
        "lint" => {
            ensure_root_has_config(&current_dir)?;
            Analyzer::lint(&current_dir).map(|res| serde_json::to_value(res).unwrap())
        }

        #[cfg(feature = "analyzer")]
        "dry" => {
            ensure_root_has_config(&current_dir)?;
            Analyzer::dry_candidates(&current_dir, min_support, min_items)
                .map(|res| serde_json::to_value(res).unwrap())
        }

        #[cfg(feature = "analyzer")]
        "refs" => {
            ensure_root_has_config(&current_dir)?;

            let target = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if target.is_empty() {
                Ok(json!({
                    "error": "Missing target",
                    "help": "Usage: grimoire_css fi refs <query> [--json]"
                }))
            } else {
                Analyzer::refs(&current_dir, target)
            }
        }

        #[cfg(feature = "analyzer")]
        "stats" => {
            ensure_root_has_config(&current_dir)?;

            let group = args.get(1).map(|s| s.as_str()).unwrap_or("");

            if !group.is_empty() && !matches!(group, "spells" | "scrolls" | "vars") {
                Ok(json!({
                    "error": "Unknown stats group",
                    "help": "Usage: grimoire_css fi stats [spells|scrolls|vars] [--top N] [--token <name>] [--json]"
                }))
            } else {
                Analyzer::stats(
                    &current_dir,
                    if group.is_empty() { None } else { Some(group) },
                    token_filter.as_deref(),
                    top,
                )
            }
        }

        #[cfg(feature = "analyzer")]
        "list" => {
            ensure_root_has_config(&current_dir)?;
            let sub = args.get(1).map(|s| s.as_str()).unwrap_or("");
            match sub {
                "scrolls" => {
                    let cfg = Analyzer::load_config(&current_dir)?;
                    let mut names: Vec<String> = cfg
                        .scrolls
                        .as_ref()
                        .map(|m| m.keys().cloned().collect())
                        .unwrap_or_default();
                    names.sort();
                    Ok(json!({"scrolls": names}))
                }
                "vars" => Analyzer::list_grimoire_variables(&current_dir)
                    .map(|vars| json!({"vars": vars})),
                _ => Ok(json!({
                    "error": "Unknown list target",
                    "help": "Usage: grimoire_css fi list scrolls | list vars [--json]"
                })),
            }
        }

        #[cfg(feature = "analyzer")]
        "explain" => {
            ensure_root_has_config(&current_dir)?;

            let arg1 = args.get(1).cloned().unwrap_or_default();
            if arg1.is_empty() {
                Ok(json!({
                    "error": "Missing target",
                    "help": "Usage: grimoire_css fi explain <token> [--json]"
                }))
            } else {
                Analyzer::explain_class_token(&current_dir, &arg1)
                    .map(|res| serde_json::to_value(res).unwrap())
            }
        }

        _ => Ok(json!({
            "help": {
                "commands": [
                    "shorten",
                    "list scrolls",
                    "list vars",
                    "explain <token>",
                    "index [--top N]",
                    "lint",
                    "dry [--min-support N] [--min-items N]",
                    "config summary",
                    "refs <query>",
                    "stats [spells|scrolls|vars] [--top N] [--token <name>]"
                ],
                "examples": [
                    "grimoire_css fi shorten",
                    "grimoire_css fi list scrolls --json",
                    "grimoire_css fi list vars --json",
                    "grimoire_css fi explain \"box=10px_20px\" --json",
                    "grimoire_css fi index --top 50 --json",
                    "grimoire_css fi lint --json",
                    "grimoire_css fi config summary --json",
                    "grimoire_css fi dry --min-support 3 --min-items 2 --json",
                    "grimoire_css fi refs box --json",
                    "grimoire_css fi refs $spacing-unit --json",
                    "grimoire_css fi refs bg-c=$primary --json",
                    "grimoire_css fi stats --top 40 --json",
                    "grimoire_css fi stats scrolls --top 40 --json",
                    "grimoire_css fi stats --token box --json"
                ]
            }
        })),
    };

    match result {
        Ok(v) => {
            if json_output {
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
            } else {
                // Default to pretty JSON (still human readable, still scriptable).
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
            }

            // Also surface any buffered messages (e.g. shorten summary).
            // This is kept out of --json mode by design.
            if !json_output {
                add_message("Tip: pass --json for clean machine output".to_string());
            }

            Ok(())
        }
        Err(e) => {
            // Keep error output machine-readable.
            let v = json!({
                "error": e.to_string(),
                "hint": "Pass --root <dir> pointing to a repo folder that contains grimoire/config/grimoire.config.json"
            });
            println!("{}", serde_json::to_string_pretty(&v).unwrap());
            Err(e)
        }
    }
}

/// Helper used by `shorten` and other commands.
#[cfg(feature = "analyzer")]
pub fn ensure_root_has_config(current_dir: &Path) -> Result<(), GrimoireCssError> {
    let cfg_path = current_dir
        .join("grimoire")
        .join("config")
        .join("grimoire.config.json");
    if cfg_path.is_file() {
        Ok(())
    } else {
        Err(GrimoireCssError::InvalidInput(format!(
            "Missing config at {}",
            cfg_path.display()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::tempdir;

    #[cfg(feature = "analyzer")]
    #[test]
    fn ensure_root_has_config_missing_is_error() {
        let dir = tempdir().unwrap();
        let err = ensure_root_has_config(dir.path()).unwrap_err();
        assert!(err.to_string().contains("Missing config"));
    }

    #[cfg(feature = "analyzer")]
    #[test]
    fn ensure_root_has_config_present_is_ok() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("grimoire/config")).unwrap();
        fs::write(
            dir.path().join("grimoire/config/grimoire.config.json"),
            r#"{ "projects": [{"projectName":"main","inputPaths":[]}] }"#,
        )
        .unwrap();
        ensure_root_has_config(dir.path()).unwrap();
    }

    #[test]
    fn run_fi_cli_help_is_ok() {
        run_fi_cli(vec!["grimoire_css".to_string(), "fi".to_string()]).unwrap();
    }

    #[cfg(feature = "analyzer")]
    #[test]
    fn run_fi_cli_requires_config_for_analyzer_commands() {
        let dir = tempdir().unwrap();

        let err = run_fi_cli(vec![
            "grimoire_css".to_string(),
            "fi".to_string(),
            "--root".to_string(),
            dir.path().to_string_lossy().to_string(),
            "--json".to_string(),
            "lint".to_string(),
        ])
        .unwrap_err();

        assert!(err.to_string().contains("Missing config"));
    }

    #[test]
    fn run_fi_cli_refs_and_stats_spells_work() {
        let dir = tempdir().unwrap();

        let input_file = dir.path().join("src/index.html");
        fs::create_dir_all(input_file.parent().unwrap()).unwrap();
        fs::write(&input_file, r#"<div class=\"display=flex\"></div>"#).unwrap();

        fs::create_dir_all(dir.path().join("grimoire/config")).unwrap();
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
            serde_json::to_string_pretty(&cfg).unwrap(),
        )
        .unwrap();

        run_fi_cli(vec![
            "grimoire_css".to_string(),
            "fi".to_string(),
            "--root".to_string(),
            dir.path().to_string_lossy().to_string(),
            "--json".to_string(),
            "refs".to_string(),
            "display=flex".to_string(),
        ])
        .unwrap();

        run_fi_cli(vec![
            "grimoire_css".to_string(),
            "fi".to_string(),
            "--root".to_string(),
            dir.path().to_string_lossy().to_string(),
            "--json".to_string(),
            "stats".to_string(),
            "--token".to_string(),
            "display=flex".to_string(),
        ])
        .unwrap();
    }
}
