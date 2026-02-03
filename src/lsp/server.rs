//! Feature-gated LSP server implementation.
//!
//! This module is used by the `grimoire_css_lsp` binary.

use crate::{Spell, analyzer::Analyzer};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;
use tower_lsp::{
    Client, LanguageServer, LspService, Server,
    jsonrpc::{Error as JsonRpcError, Result as JsonRpcResult},
    lsp_types::request::RegisterCapability,
    lsp_types::*,
};

fn lsp_execute_commands() -> Vec<String> {
    vec![
        // Canonical (mirrors `fi`)
        "grimoirecss.listScrolls".to_string(),
        "grimoirecss.listVars".to_string(),
        // UI-focused (for IDE clients)
        "grimoirecss.explorerIndex".to_string(),
        "grimoirecss.entityDetails".to_string(),
        // Editor UI helpers
        "grimoirecss.documentSpells".to_string(),
        "grimoirecss.index".to_string(),
        "grimoirecss.lint".to_string(),
        "grimoirecss.configSummary".to_string(),
        "grimoirecss.dryCandidates".to_string(),
        "grimoirecss.dryCreateScroll".to_string(),
        "grimoirecss.refs".to_string(),
        "grimoirecss.explain".to_string(),
        "grimoirecss.stats".to_string(),
    ]
}

#[derive(Debug, Default)]
struct State {
    root_dir: Option<PathBuf>,
    open_documents: HashMap<PathBuf, String>,
    cache: AnalysisCache,
}

#[derive(Debug, Default)]
struct AnalysisCache {
    index0: Option<crate::analyzer::IndexResult>,
    lint: Option<crate::analyzer::LintResult>,
    refs_scroll: HashMap<String, Vec<crate::analyzer::ScrollReference>>,
    refs_gvar: HashMap<String, Vec<crate::analyzer::GrimoireVariableReference>>,
    refs_spell: HashMap<String, Vec<crate::analyzer::SpellReference>>,
    list_gvars: Option<Vec<crate::analyzer::GrimoireVariableDefinition>>,
}

impl AnalysisCache {
    fn invalidate(&mut self) {
        self.index0 = None;
        self.lint = None;
        self.refs_scroll.clear();
        self.refs_gvar.clear();
        self.refs_spell.clear();
        self.list_gvars = None;
    }
}

#[derive(Debug)]
struct Backend {
    client: Client,
    state: Arc<RwLock<State>>,
}

#[derive(Debug, Clone)]
enum EntityAtCursor {
    Scroll(String),
    GrimoireVariable(String),
}

impl Backend {
    async fn root_dir(&self) -> Option<PathBuf> {
        self.state.read().await.root_dir.clone()
    }

    async fn set_root_dir(&self, root: PathBuf) {
        let mut state = self.state.write().await;
        state.root_dir = Some(root);
        state.cache.invalidate();
    }

    async fn update_open_document(&self, file_path: PathBuf, content: String) {
        let mut state = self.state.write().await;
        state.open_documents.insert(file_path, content);
    }

    async fn remove_open_document(&self, file_path: &Path) {
        self.state.write().await.open_documents.remove(file_path);
    }

    async fn invalidate_cache(&self) {
        self.state.write().await.cache.invalidate();
    }

    async fn read_document_text(&self, file_path: &Path) -> std::io::Result<String> {
        if let Some(s) = self.state.read().await.open_documents.get(file_path) {
            return Ok(s.clone());
        }

        std::fs::read_to_string(file_path)
    }

    async fn cached_index0(&self, root: &Path) -> JsonRpcResult<crate::analyzer::IndexResult> {
        if let Some(idx) = self.state.read().await.cache.index0.clone() {
            return Ok(idx);
        }

        let computed = Analyzer::index(root, 0).map_err(to_jsonrpc)?;
        self.state.write().await.cache.index0 = Some(computed.clone());
        Ok(computed)
    }

    async fn cached_lint(&self, root: &Path) -> JsonRpcResult<crate::analyzer::LintResult> {
        if let Some(lint) = self.state.read().await.cache.lint.clone() {
            return Ok(lint);
        }

        let computed = Analyzer::lint(root).map_err(to_jsonrpc)?;
        self.state.write().await.cache.lint = Some(computed.clone());
        Ok(computed)
    }

    async fn cached_refs_scroll(
        &self,
        root: &Path,
        scroll_name: &str,
    ) -> JsonRpcResult<Vec<crate::analyzer::ScrollReference>> {
        if let Some(v) = self
            .state
            .read()
            .await
            .cache
            .refs_scroll
            .get(scroll_name)
            .cloned()
        {
            return Ok(v);
        }

        // Use cached index (fast) instead of rescanning for each scroll.
        let idx = self.cached_index0(root).await?;
        let mut refs: Vec<crate::analyzer::ScrollReference> = idx
            .scroll_references
            .into_iter()
            .filter(|r| r.scroll == scroll_name)
            .collect();
        refs.sort_by(|a, b| {
            a.occurrence
                .file
                .cmp(&b.occurrence.file)
                .then_with(|| a.occurrence.byte_offset.cmp(&b.occurrence.byte_offset))
        });

        self.state
            .write()
            .await
            .cache
            .refs_scroll
            .insert(scroll_name.to_string(), refs.clone());
        Ok(refs)
    }

    async fn cached_refs_grimoire_variable(
        &self,
        root: &Path,
        var_name: &str,
    ) -> JsonRpcResult<Vec<crate::analyzer::GrimoireVariableReference>> {
        if let Some(v) = self
            .state
            .read()
            .await
            .cache
            .refs_gvar
            .get(var_name)
            .cloned()
        {
            return Ok(v);
        }

        let computed = Analyzer::refs_grimoire_variable(root, var_name).map_err(to_jsonrpc)?;
        self.state
            .write()
            .await
            .cache
            .refs_gvar
            .insert(var_name.to_string(), computed.clone());
        Ok(computed)
    }

    async fn cached_refs_spell(
        &self,
        root: &Path,
        raw_spell: &str,
    ) -> JsonRpcResult<Vec<crate::analyzer::SpellReference>> {
        if let Some(v) = self
            .state
            .read()
            .await
            .cache
            .refs_spell
            .get(raw_spell)
            .cloned()
        {
            return Ok(v);
        }

        let computed = Analyzer::refs_spell(root, raw_spell).map_err(to_jsonrpc)?;
        self.state
            .write()
            .await
            .cache
            .refs_spell
            .insert(raw_spell.to_string(), computed.clone());
        Ok(computed)
    }

    async fn cached_list_grimoire_variables(
        &self,
        root: &Path,
    ) -> JsonRpcResult<Vec<crate::analyzer::GrimoireVariableDefinition>> {
        if let Some(v) = self.state.read().await.cache.list_gvars.clone() {
            return Ok(v);
        }

        let computed = Analyzer::list_grimoire_variables(root).map_err(to_jsonrpc)?;
        self.state.write().await.cache.list_gvars = Some(computed.clone());
        Ok(computed)
    }

    async fn load_config(&self) -> JsonRpcResult<crate::config::ConfigFs> {
        let root = self
            .root_dir()
            .await
            .ok_or_else(|| JsonRpcError::invalid_params("Missing workspace root"))?;

        Analyzer::load_config(&root).map_err(to_jsonrpc)
    }

    fn uri_to_path(uri: &Url) -> Option<PathBuf> {
        uri.to_file_path().ok()
    }

    fn path_to_uri(path: &Path) -> Option<Url> {
        Url::from_file_path(path).ok()
    }

    async fn token_at_position(
        &self,
        file_path: &Path,
        position: Position,
    ) -> JsonRpcResult<Option<(String, (usize, usize), String, usize)>> {
        let content = self
            .read_document_text(file_path)
            .await
            .map_err(to_jsonrpc)?;

        let mut offset = position_to_offset_utf16(&content, position).ok_or_else(|| {
            JsonRpcError::invalid_params("Position is out of bounds for this file")
        })?;

        if offset > 0 && offset == content.len() {
            offset -= 1;
        }

        // If we're inside a JSON string literal (e.g. scroll spell in config), extract the whole
        // decoded string so Spell parsing and `:`-containing tokens work.
        if file_path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("json"))
            && let Some((decoded, start, len)) =
                extract_json_string_literal_at_byte_offset(&content, offset)
        {
            return Ok(Some((decoded, (start, len), content, offset)));
        }

        let Some((token, start, len)) = extract_token_at_byte_offset(&content, offset) else {
            return Ok(None);
        };

        Ok(Some((token, (start, len), content, offset)))
    }

    async fn resolve_entity_at(
        &self,
        file_path: &Path,
        position: Position,
    ) -> JsonRpcResult<Option<EntityAtCursor>> {
        let Some((token, (start, len), content, offset)) =
            self.token_at_position(file_path, position).await?
        else {
            return Ok(None);
        };

        // Prefer the most specific thing under the cursor: `$var`, then scroll name.
        if let Some(var) = extract_dollar_identifier_at(&content, offset) {
            return Ok(Some(EntityAtCursor::GrimoireVariable(var)));
        }

        // Templated scroll invocation: `g!scroll-name;` should behave like a scroll entity
        // for definition/refs/details, while still allowing `Explain token` to use the full token.
        if let Some(scroll_name) = Self::extract_templated_scroll_name(&token) {
            return Ok(Some(EntityAtCursor::Scroll(scroll_name)));
        }

        let cfg = self.load_config().await?;
        let spell = Spell::new(&token, &cfg.shared_spells, &cfg.scrolls, (start, len), None)
            .map_err(to_jsonrpc)?;

        let Some(spell) = spell else {
            return Ok(None);
        };

        if spell.scroll_spells.is_some() && !spell.component().is_empty() {
            Ok(Some(EntityAtCursor::Scroll(spell.component().to_string())))
        } else {
            Ok(None)
        }
    }

    fn extract_templated_scroll_name(token: &str) -> Option<String> {
        let t = token.trim();
        let rest = t.strip_prefix("g!")?;
        let name = rest.strip_suffix(';')?;
        if name.is_empty() {
            return None;
        }

        // Keep this strict: templated scroll invocation grammar is `g!<scroll-name>;`.
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
        {
            return None;
        }

        Some(name.to_string())
    }

    async fn find_scroll_definition(&self, scroll_name: &str) -> JsonRpcResult<Option<Location>> {
        Ok(self
            .find_scroll_definitions(scroll_name)
            .await?
            .into_iter()
            .next())
    }

    async fn find_scroll_definitions(&self, scroll_name: &str) -> JsonRpcResult<Vec<Location>> {
        let root = self
            .root_dir()
            .await
            .ok_or_else(|| JsonRpcError::invalid_params("Missing workspace root"))?;

        let config_dir = root.join("grimoire").join("config");
        let main_config = config_dir.join("grimoire.config.json");

        // Search in main config first, then in external scroll files.
        let mut files = vec![main_config];
        if let Ok(entries) = glob::glob(
            config_dir
                .join("grimoire.*.scrolls.json")
                .to_string_lossy()
                .as_ref(),
        ) {
            for p in entries.flatten() {
                files.push(p);
            }
        }

        let mut out = Vec::new();

        for file in files {
            let Ok(content) = self.read_document_text(&file).await else {
                continue;
            };

            // Precise search: only within the `scrolls: [...]` array.
            let Some((start, end)) = find_scroll_name_value_in_scrolls_array(&content, scroll_name)
            else {
                continue;
            };

            let start_pos = offset_to_position_utf16(&content, start);
            let end_pos = offset_to_position_utf16(&content, end);
            if let (Some(start_pos), Some(end_pos)) = (start_pos, end_pos)
                && let Some(uri) = Self::path_to_uri(&file)
            {
                out.push(Location {
                    uri,
                    range: Range {
                        start: start_pos,
                        end: end_pos,
                    },
                });
            }
        }

        Ok(out)
    }

    async fn find_grimoire_variable_definition(
        &self,
        var_name: &str,
    ) -> JsonRpcResult<Option<Location>> {
        Ok(self
            .find_grimoire_variable_definitions(var_name)
            .await?
            .into_iter()
            .next())
    }

    async fn find_grimoire_variable_definitions(
        &self,
        var_name: &str,
    ) -> JsonRpcResult<Vec<Location>> {
        let root = self
            .root_dir()
            .await
            .ok_or_else(|| JsonRpcError::invalid_params("Missing workspace root"))?;

        let config_dir = root.join("grimoire").join("config");
        let main_config = config_dir.join("grimoire.config.json");

        // main config first, then external variable files.
        let mut files = Vec::new();
        if main_config.is_file() {
            files.push(main_config);
        }

        if let Ok(entries) = glob::glob(
            config_dir
                .join("grimoire.*.variables.json")
                .to_string_lossy()
                .as_ref(),
        ) {
            for p in entries.flatten() {
                if p.is_file() {
                    files.push(p);
                }
            }
        }

        let mut out = Vec::new();

        for file in files {
            let Ok(content) = self.read_document_text(&file).await else {
                continue;
            };

            let Some((start, end)) =
                find_object_member_key_in_named_object(&content, "variables", var_name)
            else {
                continue;
            };

            let Some(start_pos) = offset_to_position_utf16(&content, start) else {
                continue;
            };
            let Some(end_pos) = offset_to_position_utf16(&content, end) else {
                continue;
            };
            let Some(uri) = Self::path_to_uri(&file) else {
                continue;
            };

            out.push(Location {
                uri,
                range: Range {
                    start: start_pos,
                    end: end_pos,
                },
            });
        }

        Ok(out)
    }
}

fn parse_version_triplet(input: &str) -> Option<(u64, u64, u64)> {
    let s = input.trim().strip_prefix('v').unwrap_or(input.trim());
    let mut it = s.split('.');
    let major: u64 = it.next()?.parse().ok()?;
    let minor: u64 = it.next()?.parse().ok()?;
    let patch: u64 = it.next()?.parse().ok()?;
    Some((major, minor, patch))
}

fn is_newer_version(latest: &str, current: &str) -> bool {
    match (
        parse_version_triplet(latest),
        parse_version_triplet(current),
    ) {
        (Some(l), Some(c)) => l > c,
        _ => false,
    }
}

fn fetch_latest_release_version_blocking() -> Option<String> {
    let url = "https://api.github.com/repos/persevie/grimoire-css/releases/latest";

    // GitHub API requires a User-Agent.
    let resp = ureq::get(url)
        .set("User-Agent", "grimoire-css-lsp")
        .call()
        .ok()?;

    if resp.status() != 200 {
        return None;
    }

    let text = resp.into_string().ok()?;
    let json: serde_json::Value = serde_json::from_str(&text).ok()?;
    let tag = json.get("tag_name")?.as_str()?.trim();
    Some(tag.strip_prefix('v').unwrap_or(tag).to_string())
}

fn spawn_update_check(client: Client) {
    tokio::spawn(async move {
        let current = env!("CARGO_PKG_VERSION").to_string();

        let latest: Option<String> =
            (tokio::task::spawn_blocking(fetch_latest_release_version_blocking).await)
                .unwrap_or_default();

        let Some(latest) = latest else {
            return;
        };

        if !is_newer_version(&latest, &current) {
            return;
        }

        let msg = format!("A newer Grimoire CSS is available: v{latest} (you have v{current})");
        let _ = client.show_message(MessageType::INFO, msg.clone()).await;
        let _ = client.log_message(MessageType::INFO, msg).await;
    });
}

#[cfg(test)]
mod tests {
    use super::{extract_token_at_byte_offset, lsp_execute_commands};
    use crate::config::ConfigFs;
    use tempfile::tempdir;

    #[test]
    fn advertised_execute_commands_are_canonical() {
        let cmds = lsp_execute_commands();
        assert!(cmds.contains(&"grimoirecss.explain".to_string()));
        assert!(cmds.contains(&"grimoirecss.refs".to_string()));
        assert!(cmds.contains(&"grimoirecss.stats".to_string()));
        assert!(cmds.contains(&"grimoirecss.explorerIndex".to_string()));
        assert!(!cmds.contains(&"grimoirecss.refsCssVariable".to_string()));
    }

    #[test]
    fn token_extraction_includes_template_semicolon() {
        let content = r#"<div class=\"g!md3-btn;\"></div>"#;
        let offset = content.find("md3-btn").unwrap() + 2;
        let (token, _start, _len) = extract_token_at_byte_offset(content, offset).unwrap();
        assert_eq!(token, "g!md3-btn;");
    }

    #[test]
    fn dry_create_scroll_partitions_by_scroll_membership_and_preserves_prefix_semantics() {
        let dir = tempdir().unwrap();
        let cfg_dir = dir.path().join("grimoire").join("config");
        std::fs::create_dir_all(&cfg_dir).unwrap();

        // Minimal config containing a scroll with spellsByArgs.
        let config_json = r#"{
    "$schema": "https://raw.githubusercontent.com/persevie/grimoire-css/main/src/core/config/config-schema.json",
    "variables": null,
    "scrolls": [
        {
            "name": "box",
            "spells": [
                "height=var(--box-height,_100px)",
                "width=var(--box-width,_100px)"
            ],
            "spellsByArgs": {
                "1": [
                    "padding-top=$1",
                    "padding-right=$1",
                    "padding-bottom=$1",
                    "padding-left=$1"
                ],
                "2": [
                    "padding-top=$1",
                    "padding-bottom=$1",
                    "padding-left=$2",
                    "padding-right=$2"
                ]
            }
        }
    ],
    "projects": [
        {
            "projectName": "test",
            "inputPaths": ["index.html"],
            "outputDirPath": ".",
            "singleOutputFileName": "out.css"
        }
    ],
    "shared": null,
    "critical": null,
    "lock": null
}"#;

        let cfg_path = cfg_dir.join("grimoire.config.json");
        std::fs::write(&cfg_path, config_json).unwrap();
        let cfg = ConfigFs::load(dir.path()).unwrap();

        // 1) Plain scroll name stays in `spells`.
        let (extends, spells) =
            super::partition_dry_candidate_tokens_for_new_scroll(&cfg, &["box".to_string()])
                .unwrap();
        assert!(extends.is_empty());
        assert_eq!(spells, vec!["box".to_string()]);

        // 2) Scroll with args stays in `spells` (invocation preserved).
        let (extends, spells) = super::partition_dry_candidate_tokens_for_new_scroll(
            &cfg,
            &["box=10px_20px".to_string()],
        )
        .unwrap();
        assert!(extends.is_empty());
        assert_eq!(spells, vec!["box=10px_20px".to_string()]);

        // 3) Scroll with effects stays in `spells` (invocation preserved).
        let (extends, spells) = super::partition_dry_candidate_tokens_for_new_scroll(
            &cfg,
            &["hover:box=4px".to_string()],
        )
        .unwrap();
        assert!(extends.is_empty());
        assert_eq!(spells, vec!["hover:box=4px".to_string()]);
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> JsonRpcResult<InitializeResult> {
        let root = params
            .root_uri
            .and_then(|u| u.to_file_path().ok())
            .or_else(|| {
                params
                    .workspace_folders
                    .as_ref()
                    .and_then(|wf| wf.first())
                    .and_then(|wf| wf.uri.to_file_path().ok())
            })
            .or_else(|| std::env::current_dir().ok());

        if let Some(root) = root {
            self.set_root_dir(root).await;
        }

        let commands = lsp_execute_commands();

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                // Client-specific UI code actions should be implemented by the client/extension,
                // not by the LSP server.
                code_action_provider: None,
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands,
                    ..Default::default()
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "grimoire-css-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        let banner = format!(
            "Grimoire CSS LSP initialized (v{})",
            env!("CARGO_PKG_VERSION")
        );

        let _ = self
            .client
            .show_message(MessageType::INFO, banner.clone())
            .await;

        let _ = self.client.log_message(MessageType::INFO, banner).await;

        // Silent update check: only notifies if a newer version exists.
        spawn_update_check(self.client.clone());

        // Register watched-file notifications so cache invalidation works even when files change
        // outside of currently opened editors.
        // NOTE: Use simple glob patterns (no `{a,b}`) for maximum client compatibility.
        let watchers = [
            "grimoire/config/**/*.json",
            "**/*.html",
            "**/*.htm",
            "**/*.js",
            "**/*.jsx",
            "**/*.ts",
            "**/*.tsx",
            "**/*.css",
        ]
        .into_iter()
        .map(|p| FileSystemWatcher {
            glob_pattern: GlobPattern::String(p.to_string()),
            kind: None,
        })
        .collect();

        let opts = DidChangeWatchedFilesRegistrationOptions { watchers };
        let reg = Registration {
            id: "grimoirecss.watch".to_string(),
            method: "workspace/didChangeWatchedFiles".to_string(),
            register_options: Some(serde_json::to_value(opts).unwrap_or(serde_json::Value::Null)),
        };

        let _ = self
            .client
            .send_request::<RegisterCapability>(RegistrationParams {
                registrations: vec![reg],
            })
            .await;

        // Some clients (or setups) might reject dynamic registration; not fatal.
        let _ = self
            .client
            .log_message(
                MessageType::INFO,
                "Registered watched files for cache invalidation",
            )
            .await;
    }

    async fn shutdown(&self) -> JsonRpcResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let Some(path) = Self::uri_to_path(&params.text_document.uri) else {
            return;
        };

        self.update_open_document(path, params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let Some(path) = Self::uri_to_path(&params.text_document.uri) else {
            return;
        };

        // We're declaring FULL sync, so the last change should contain the full document text.
        let Some(last) = params.content_changes.last() else {
            return;
        };

        self.update_open_document(path, last.text.clone()).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        // Saving means disk content changed; cached analysis should be invalidated.
        if let Some(_path) = Self::uri_to_path(&params.text_document.uri) {
            self.invalidate_cache().await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let Some(path) = Self::uri_to_path(&params.text_document.uri) else {
            return;
        };

        self.remove_open_document(&path).await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.invalidate_cache().await;
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> JsonRpcResult<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let Some(file_path) = Self::uri_to_path(&uri) else {
            return Ok(None);
        };

        let Some(entity) = self.resolve_entity_at(&file_path, position).await? else {
            return Ok(None);
        };

        let loc = match entity {
            EntityAtCursor::Scroll(name) => self.find_scroll_definition(&name).await?,
            EntityAtCursor::GrimoireVariable(name) => {
                self.find_grimoire_variable_definition(&name).await?
            }
        };

        Ok(loc.map(GotoDefinitionResponse::Scalar))
    }

    async fn references(&self, params: ReferenceParams) -> JsonRpcResult<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let Some(file_path) = Self::uri_to_path(&uri) else {
            return Ok(None);
        };

        let Some(entity) = self.resolve_entity_at(&file_path, position).await? else {
            return Ok(None);
        };

        let root = self
            .root_dir()
            .await
            .ok_or_else(|| JsonRpcError::invalid_params("Missing workspace root"))?;

        let mut locations = Vec::new();

        match entity {
            EntityAtCursor::Scroll(scroll_name) => {
                let refs = self.cached_refs_scroll(&root, &scroll_name).await?;
                for r in refs {
                    if let Some(loc) = occurrence_to_location(&root, &r.occurrence) {
                        locations.push(loc);
                    }
                }
            }
            EntityAtCursor::GrimoireVariable(var_name) => {
                let refs = self.cached_refs_grimoire_variable(&root, &var_name).await?;
                for r in refs {
                    if let Some(loc) = occurrence_to_location(&root, &r.occurrence) {
                        locations.push(loc);
                    }
                }
            }
        }

        Ok(Some(locations))
    }

    async fn hover(&self, params: HoverParams) -> JsonRpcResult<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let Some(file_path) = Self::uri_to_path(&uri) else {
            return Ok(None);
        };

        let Some((token, (start, len), content, offset)) =
            self.token_at_position(&file_path, position).await?
        else {
            return Ok(None);
        };

        let root = self
            .root_dir()
            .await
            .ok_or_else(|| JsonRpcError::invalid_params("Missing workspace root"))?;

        let cfg = self.load_config().await?;

        // Most specific: `$var`
        if let Some(var) = extract_dollar_identifier_at(&content, offset) {
            let md = format_grimoire_var_hover_markdown(&cfg, &var);
            if md.is_empty() {
                return Ok(None);
            }
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: md,
                }),
                range: None,
            }));
        }

        let Ok(Some(spell)) =
            Spell::new(&token, &cfg.shared_spells, &cfg.scrolls, (start, len), None)
        else {
            return Ok(None);
        };

        let md = self
            .format_spell_or_scroll_hover_markdown(&root, &cfg, &token, &spell)
            .await;
        if md.is_empty() {
            return Ok(None);
        }

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: md,
            }),
            range: None,
        }))
    }

    async fn code_action(
        &self,
        params: CodeActionParams,
    ) -> JsonRpcResult<Option<CodeActionResponse>> {
        // This server intentionally does not provide client-specific UI actions.
        // Keep hook for future portable edits/refactors if needed.
        let _ = params;
        Ok(None)
    }

    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> JsonRpcResult<Option<serde_json::Value>> {
        let root = self
            .root_dir()
            .await
            .ok_or_else(|| JsonRpcError::invalid_params("Missing workspace root"))?;

        let args: &[serde_json::Value] = &params.arguments;

        match params.command.as_str() {
            "grimoirecss.explorerIndex" => {
                let cfg = Analyzer::load_config(&root).map_err(to_jsonrpc)?;

                let mut scrolls: Vec<String> = cfg
                    .scrolls
                    .as_ref()
                    .map(|m| {
                        let mut ks: Vec<String> = m.keys().cloned().collect();
                        ks.sort();
                        ks
                    })
                    .unwrap_or_default();

                scrolls.sort();

                let vars = self.cached_list_grimoire_variables(&root).await?;

                // Functions
                let mut functions: Vec<serde_json::Value> = Vec::new();
                functions.push(serde_json::json!({
                    "name": "mrs",
                    "group": "Sizing",
                    "usage": "mrs(min_max_[min_vw_max_vw])"
                }));
                functions.push(serde_json::json!({
                    "name": "mfs",
                    "group": "Sizing",
                    "usage": "mfs(min_max_[min_vw_max_vw])"
                }));

                for (name, usage) in crate::core::list_spell_color_functions() {
                    functions.push(serde_json::json!({
                        "name": name,
                        "group": "Color",
                        "usage": usage,
                    }));
                }

                functions.sort_by(|a, b| {
                    let ag = a.get("group").and_then(|v| v.as_str()).unwrap_or("");
                    let an = a.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    let bg = b.get("group").and_then(|v| v.as_str()).unwrap_or("");
                    let bn = b.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    (ag, an).cmp(&(bg, bn))
                });

                // Animations
                let mut animations: Vec<serde_json::Value> = Vec::new();
                for name in crate::core::list_builtin_animation_names() {
                    animations.push(serde_json::json!({
                        "name": name,
                        "kind": "built-in",
                        "source_path": serde_json::Value::Null,
                    }));
                }

                let mut custom_names: Vec<String> = cfg.custom_animations.keys().cloned().collect();
                custom_names.sort();
                for name in custom_names {
                    let source_path = root
                        .join("grimoire")
                        .join("animations")
                        .join(format!("{name}.css"))
                        .to_string_lossy()
                        .to_string();

                    animations.push(serde_json::json!({
                        "name": name,
                        "kind": "custom",
                        "source_path": source_path,
                    }));
                }

                animations.sort_by(|a, b| {
                    let ak = a.get("kind").and_then(|v| v.as_str()).unwrap_or("");
                    let an = a.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    let bk = b.get("kind").and_then(|v| v.as_str()).unwrap_or("");
                    let bn = b.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    (ak, an).cmp(&(bk, bn))
                });

                return Ok(Some(serde_json::json!({
                    "scrolls": scrolls,
                    "variables": vars,
                    "functions": functions,
                    "animations": animations,
                })));
            }
            "grimoirecss.documentSpells" => {
                // Args: { uri: string }
                let Some(obj) = args.first().and_then(|v| v.as_object()) else {
                    return Err(JsonRpcError::invalid_params(
                        "documentSpells expects first argument: { uri }",
                    ));
                };

                let uri_str = obj.get("uri").and_then(|v| v.as_str()).unwrap_or("");
                if uri_str.is_empty() {
                    return Err(JsonRpcError::invalid_params(
                        "documentSpells expects { uri: string }",
                    ));
                }

                let uri = Url::parse(uri_str).map_err(|_| {
                    JsonRpcError::invalid_params("documentSpells expects a valid file URI")
                })?;
                let Some(file_path) = Self::uri_to_path(&uri) else {
                    return Err(JsonRpcError::invalid_params(
                        "documentSpells expects a file:// URI",
                    ));
                };

                let content = self
                    .read_document_text(&file_path)
                    .await
                    .map_err(to_jsonrpc)?;
                let cfg = self.load_config().await?;

                let parser = crate::core::parser::parser_base::Parser::new();
                let mut candidates: Vec<(String, (usize, usize))> = Vec::new();
                // Keep duplicates: editor highlighting needs *all* occurrences.
                parser
                    .collect_candidates_all(&content, &mut candidates)
                    .map_err(to_jsonrpc)?;

                let mut ranges: Vec<Range> = Vec::new();

                for (token, (start, len)) in candidates {
                    let parsed =
                        Spell::new(&token, &cfg.shared_spells, &cfg.scrolls, (start, len), None);

                    let Ok(Some(_spell)) = parsed else {
                        continue;
                    };

                    let Some(start_pos) = offset_to_position_utf16(&content, start) else {
                        continue;
                    };
                    let Some(end_pos) = offset_to_position_utf16(&content, start + len) else {
                        continue;
                    };

                    ranges.push(Range {
                        start: start_pos,
                        end: end_pos,
                    });
                }

                return Ok(Some(serde_json::json!(ranges)));
            }
            "grimoirecss.entityDetails" => {
                // Args: { kind: "scroll"|"var"|"function"|"animation", name: string }
                let Some(obj) = args.first().and_then(|v| v.as_object()) else {
                    return Err(JsonRpcError::invalid_params(
                        "entityDetails expects first argument: { kind, name }",
                    ));
                };

                let kind = obj.get("kind").and_then(|v| v.as_str()).unwrap_or("");
                let raw_name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("");
                if kind.is_empty() || raw_name.is_empty() {
                    return Err(JsonRpcError::invalid_params(
                        "entityDetails expects { kind: 'scroll'|'var'|'function'|'animation', name }",
                    ));
                }

                let summary = Analyzer::config_summary(&root).map_err(to_jsonrpc)?;
                let cfg = self.load_config().await?;
                let mut project_inputs: Vec<PathBuf> = Vec::new();
                for p in &summary.projects {
                    for input in &p.input_paths {
                        let resolved = if Path::new(input).is_absolute() {
                            PathBuf::from(input)
                        } else {
                            root.join(input)
                        };
                        project_inputs.push(resolved);
                    }
                }

                let config_dir = root.join("grimoire").join("config");

                let is_under_any_input = |file: &Path| -> bool {
                    project_inputs
                        .iter()
                        .any(|p| file == p || file.starts_with(p))
                };

                let mut project_locs: Vec<Location> = Vec::new();
                let mut config_locs: Vec<Location> = Vec::new();
                let mut definitions: Vec<Location> = Vec::new();

                let mut css: Option<String> = None;
                let mut value: Option<String> = None;
                let mut usage: Option<String> = None;
                let mut example: Option<String> = None;
                let mut source_path: Option<String> = None;
                let mut animation_kind: Option<String> = None;
                let mut group: Option<String> = None;

                match kind {
                    "scroll" => {
                        let name = raw_name;

                        definitions.extend(self.find_scroll_definitions(name).await?);

                        // References (raw) -> Locations, split into project vs config.
                        let refs = self
                            .cached_refs_scroll(&root, name)
                            .await
                            .unwrap_or_default();
                        for r in refs {
                            let file_path = root.join(&r.occurrence.file);
                            let loc = occurrence_to_location(&root, &r.occurrence);
                            let Some(loc) = loc else { continue };

                            if file_path.starts_with(&config_dir) {
                                config_locs.push(loc);
                            } else if is_under_any_input(&file_path) {
                                project_locs.push(loc);
                            }
                        }

                        css = Analyzer::explain_class_token(&root, name)
                            .ok()
                            .map(|r| r.css);
                    }
                    "var" => {
                        let name = raw_name.trim_start_matches('$');

                        definitions.extend(self.find_grimoire_variable_definitions(name).await?);

                        let vars = self
                            .cached_list_grimoire_variables(&root)
                            .await
                            .unwrap_or_default();
                        value = vars
                            .iter()
                            .find(|v| v.name == name)
                            .map(|v| v.value.clone());

                        let refs = self
                            .cached_refs_grimoire_variable(&root, name)
                            .await
                            .unwrap_or_default();
                        for r in refs {
                            let file_path = root.join(&r.occurrence.file);
                            let loc = occurrence_to_location(&root, &r.occurrence);
                            let Some(loc) = loc else { continue };

                            if file_path.starts_with(&config_dir) {
                                config_locs.push(loc);
                            } else if is_under_any_input(&file_path) {
                                project_locs.push(loc);
                            }
                        }
                    }
                    "function" => {
                        let name = raw_name;

                        // Usage / grouping.
                        match name {
                            "mrs" => {
                                usage = Some("mrs(min_max_[min_vw_max_vw])".to_string());
                                example =
                                    Some("g!font-size=mrs(14px_16px_380px_800px);".to_string());
                                group = Some("fluid".to_string());
                            }
                            "mfs" => {
                                usage = Some("mfs(min_max_[min_vw_max_vw])".to_string());
                                group = Some("fluid".to_string());
                            }
                            _ => {
                                for (func, u) in crate::core::list_spell_color_functions() {
                                    if func == name {
                                        usage = Some(u.to_string());
                                        group = Some("color".to_string());
                                        break;
                                    }
                                }
                            }
                        }

                        // References: find occurrences in project files and in scroll config spell strings.
                        // We treat the whole token as the reference location.
                        let needle = format!("{name}(");
                        let parser = crate::core::parser::parser_base::Parser::new();

                        let mut files = std::collections::HashSet::<PathBuf>::new();
                        for input in &project_inputs {
                            if input.exists() && input.is_dir() {
                                let mut pat = input.to_string_lossy().to_string();
                                if !pat.ends_with('/') {
                                    pat.push('/');
                                }
                                pat.push_str("**/*");
                                if let Ok(entries) = glob::glob(&pat) {
                                    for p in entries.flatten() {
                                        if p.is_file() {
                                            files.insert(p);
                                        }
                                    }
                                }
                            } else {
                                let pat = input.to_string_lossy().to_string();
                                if let Ok(entries) = glob::glob(&pat) {
                                    for p in entries.flatten() {
                                        if p.is_file() {
                                            files.insert(p);
                                        }
                                    }
                                }
                            }
                        }

                        let mut file_list: Vec<PathBuf> = files.into_iter().collect();
                        file_list.sort();

                        for file_path in &file_list {
                            let Ok(content) = std::fs::read_to_string(file_path) else {
                                continue;
                            };

                            let mut candidates: Vec<(String, (usize, usize))> = Vec::new();
                            if parser
                                .collect_candidates_all(&content, &mut candidates)
                                .is_err()
                            {
                                continue;
                            }

                            for (token, (start, len)) in candidates {
                                let parsed = Spell::new(
                                    &token,
                                    &cfg.shared_spells,
                                    &cfg.scrolls,
                                    (start, len),
                                    None,
                                );

                                let Ok(Some(spell)) = parsed else {
                                    continue;
                                };

                                let expanded: Vec<&str> =
                                    if let Some(scroll_spells) = &spell.scroll_spells {
                                        scroll_spells.iter().map(|s| s.raw_spell.as_str()).collect()
                                    } else {
                                        vec![spell.raw_spell.as_str()]
                                    };

                                if !expanded.iter().any(|s| s.contains(&needle)) {
                                    continue;
                                }

                                let rel = file_path.strip_prefix(&root).unwrap_or(file_path);
                                let occ = crate::analyzer::TokenOccurrence {
                                    token,
                                    file: rel.to_string_lossy().to_string(),
                                    byte_offset: start,
                                    byte_len: len,
                                    line: 1,
                                    column: 1,
                                };
                                if let Some(loc) = occurrence_to_location(&root, &occ) {
                                    project_locs.push(loc);
                                }
                            }
                        }

                        // Config JSON spell strings.
                        let mut cfg_files: Vec<PathBuf> = Vec::new();
                        let main = root
                            .join("grimoire")
                            .join("config")
                            .join("grimoire.config.json");
                        if main.is_file() {
                            cfg_files.push(main);
                        }
                        let pattern = root
                            .join("grimoire")
                            .join("config")
                            .join("grimoire.*.scrolls.json")
                            .to_string_lossy()
                            .to_string();
                        if let Ok(entries) = glob::glob(&pattern) {
                            for p in entries.flatten() {
                                if p.is_file() {
                                    cfg_files.push(p);
                                }
                            }
                        }
                        cfg_files.sort();
                        cfg_files.dedup();

                        for file_path in cfg_files {
                            let Ok(content) = std::fs::read_to_string(&file_path) else {
                                continue;
                            };
                            let json: serde_json::Value = match serde_json::from_str(&content) {
                                Ok(v) => v,
                                Err(_) => continue,
                            };

                            let mut search_from: usize = 0;
                            let Some(scrolls) = json.get("scrolls").and_then(|v| v.as_array())
                            else {
                                continue;
                            };

                            let push_match = |raw_spell: &str,
                                              search_from: &mut usize,
                                              out: &mut Vec<Location>|
                             -> Result<(), crate::GrimoireCssError> {
                                if !raw_spell.contains(&needle) {
                                    return Ok(());
                                }
                                let json_string = serde_json::to_string(raw_spell).map_err(|e| {
                                    crate::GrimoireCssError::InvalidInput(format!(
                                        "Failed to encode JSON string for spell: {e}"
                                    ))
                                })?;
                                let mut found = None;
                                if *search_from < content.len()
                                    && let Some(rel) = content[*search_from..].find(&json_string)
                                {
                                    found = Some(*search_from + rel);
                                }
                                if found.is_none() {
                                    found = content.find(&json_string);
                                }
                                let Some(byte_offset) = found else {
                                    return Ok(());
                                };
                                *search_from = byte_offset + json_string.len();
                                let rel = file_path.strip_prefix(&root).unwrap_or(&file_path);
                                let occ = crate::analyzer::TokenOccurrence {
                                    token: raw_spell.to_string(),
                                    file: rel.to_string_lossy().to_string(),
                                    byte_offset,
                                    byte_len: json_string.len(),
                                    line: 1,
                                    column: 1,
                                };
                                if let Some(loc) = occurrence_to_location(&root, &occ) {
                                    out.push(loc);
                                }
                                Ok(())
                            };

                            for scroll in scrolls {
                                if let Some(spells) =
                                    scroll.get("spells").and_then(|v| v.as_array())
                                {
                                    for s in spells.iter().filter_map(|v| v.as_str()) {
                                        let _ = push_match(s, &mut search_from, &mut config_locs);
                                    }
                                }

                                if let Some(obj) =
                                    scroll.get("spellsByArgs").and_then(|v| v.as_object())
                                {
                                    for (_k, arr) in obj {
                                        let Some(spells) = arr.as_array() else {
                                            continue;
                                        };
                                        for s in spells.iter().filter_map(|v| v.as_str()) {
                                            let _ =
                                                push_match(s, &mut search_from, &mut config_locs);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "animation" => {
                        let name = raw_name;
                        group = Some("animation".to_string());

                        // Usage shown in Details UI.
                        usage = Some(format!("anim-dur=1s anim-i-c=infinite anim-n={name}"));

                        let needle_a = format!("anim-n={name}");
                        let needle_b = format!("g-anim={name}");

                        // Built-in first
                        if let Some(anim_css) = crate::core::get_builtin_animation_css(name) {
                            animation_kind = Some("built-in".to_string());
                            css = Some(anim_css);
                        } else {
                            // Custom from config
                            if let Some(anim_css) = cfg.custom_animations.get(name) {
                                animation_kind = Some("custom".to_string());
                                css = Some(anim_css.clone());
                                source_path = Some(
                                    root.join("grimoire")
                                        .join("animations")
                                        .join(format!("{name}.css"))
                                        .to_string_lossy()
                                        .to_string(),
                                );

                                // Definition points to the source file.
                                if let Some(p) = source_path
                                    .as_ref()
                                    .and_then(|s| PathBuf::from(s).canonicalize().ok())
                                    && let Ok(uri) = Url::from_file_path(&p)
                                {
                                    definitions.push(Location {
                                        uri,
                                        range: Range {
                                            start: Position {
                                                line: 0,
                                                character: 0,
                                            },
                                            end: Position {
                                                line: 0,
                                                character: 0,
                                            },
                                        },
                                    });
                                }
                            }
                        }

                        // References in project + config spell strings.
                        let parser = crate::core::parser::parser_base::Parser::new();

                        let mut files = std::collections::HashSet::<PathBuf>::new();
                        for input in &project_inputs {
                            if input.exists() && input.is_dir() {
                                let mut pat = input.to_string_lossy().to_string();
                                if !pat.ends_with('/') {
                                    pat.push('/');
                                }
                                pat.push_str("**/*");
                                if let Ok(entries) = glob::glob(&pat) {
                                    for p in entries.flatten() {
                                        if p.is_file() {
                                            files.insert(p);
                                        }
                                    }
                                }
                            } else {
                                let pat = input.to_string_lossy().to_string();
                                if let Ok(entries) = glob::glob(&pat) {
                                    for p in entries.flatten() {
                                        if p.is_file() {
                                            files.insert(p);
                                        }
                                    }
                                }
                            }
                        }

                        let mut file_list: Vec<PathBuf> = files.into_iter().collect();
                        file_list.sort();

                        for file_path in &file_list {
                            let Ok(content) = std::fs::read_to_string(file_path) else {
                                continue;
                            };

                            let mut candidates: Vec<(String, (usize, usize))> = Vec::new();
                            if parser
                                .collect_candidates_all(&content, &mut candidates)
                                .is_err()
                            {
                                continue;
                            }

                            for (token, (start, len)) in candidates {
                                let parsed = Spell::new(
                                    &token,
                                    &cfg.shared_spells,
                                    &cfg.scrolls,
                                    (start, len),
                                    None,
                                );

                                let Ok(Some(spell)) = parsed else {
                                    continue;
                                };

                                let expanded: Vec<&str> =
                                    if let Some(scroll_spells) = &spell.scroll_spells {
                                        scroll_spells.iter().map(|s| s.raw_spell.as_str()).collect()
                                    } else {
                                        vec![spell.raw_spell.as_str()]
                                    };

                                if !expanded
                                    .iter()
                                    .any(|s| s.contains(&needle_a) || s.contains(&needle_b))
                                {
                                    continue;
                                }

                                let rel = file_path.strip_prefix(&root).unwrap_or(file_path);
                                let occ = crate::analyzer::TokenOccurrence {
                                    token,
                                    file: rel.to_string_lossy().to_string(),
                                    byte_offset: start,
                                    byte_len: len,
                                    line: 1,
                                    column: 1,
                                };
                                if let Some(loc) = occurrence_to_location(&root, &occ) {
                                    project_locs.push(loc);
                                }
                            }
                        }

                        let mut cfg_files: Vec<PathBuf> = Vec::new();
                        let main = root
                            .join("grimoire")
                            .join("config")
                            .join("grimoire.config.json");
                        if main.is_file() {
                            cfg_files.push(main);
                        }
                        let pattern = root
                            .join("grimoire")
                            .join("config")
                            .join("grimoire.*.scrolls.json")
                            .to_string_lossy()
                            .to_string();
                        if let Ok(entries) = glob::glob(&pattern) {
                            for p in entries.flatten() {
                                if p.is_file() {
                                    cfg_files.push(p);
                                }
                            }
                        }
                        cfg_files.sort();
                        cfg_files.dedup();

                        for file_path in cfg_files {
                            let Ok(content) = std::fs::read_to_string(&file_path) else {
                                continue;
                            };
                            let json: serde_json::Value = match serde_json::from_str(&content) {
                                Ok(v) => v,
                                Err(_) => continue,
                            };

                            let mut search_from: usize = 0;
                            let Some(scrolls) = json.get("scrolls").and_then(|v| v.as_array())
                            else {
                                continue;
                            };

                            let push_match = |raw_spell: &str,
                                              search_from: &mut usize,
                                              out: &mut Vec<Location>|
                             -> Result<(), crate::GrimoireCssError> {
                                if !(raw_spell.contains(&needle_a) || raw_spell.contains(&needle_b)) {
                                    return Ok(());
                                }
                                let json_string = serde_json::to_string(raw_spell).map_err(|e| {
                                    crate::GrimoireCssError::InvalidInput(format!(
                                        "Failed to encode JSON string for spell: {e}"
                                    ))
                                })?;
                                let mut found = None;
                                if *search_from < content.len()
                                    && let Some(rel) = content[*search_from..].find(&json_string)
                                {
                                    found = Some(*search_from + rel);
                                }
                                if found.is_none() {
                                    found = content.find(&json_string);
                                }
                                let Some(byte_offset) = found else {
                                    return Ok(());
                                };
                                *search_from = byte_offset + json_string.len();
                                let rel = file_path.strip_prefix(&root).unwrap_or(&file_path);
                                let occ = crate::analyzer::TokenOccurrence {
                                    token: raw_spell.to_string(),
                                    file: rel.to_string_lossy().to_string(),
                                    byte_offset,
                                    byte_len: json_string.len(),
                                    line: 1,
                                    column: 1,
                                };
                                if let Some(loc) = occurrence_to_location(&root, &occ) {
                                    out.push(loc);
                                }
                                Ok(())
                            };

                            for scroll in scrolls {
                                if let Some(spells) =
                                    scroll.get("spells").and_then(|v| v.as_array())
                                {
                                    for s in spells.iter().filter_map(|v| v.as_str()) {
                                        let _ = push_match(s, &mut search_from, &mut config_locs);
                                    }
                                }

                                if let Some(obj) =
                                    scroll.get("spellsByArgs").and_then(|v| v.as_object())
                                {
                                    for (_k, arr) in obj {
                                        let Some(spells) = arr.as_array() else {
                                            continue;
                                        };
                                        for s in spells.iter().filter_map(|v| v.as_str()) {
                                            let _ =
                                                push_match(s, &mut search_from, &mut config_locs);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        return Err(JsonRpcError::invalid_params(
                            "entityDetails kind must be 'scroll', 'var', 'function' or 'animation'",
                        ));
                    }
                }

                let defs_count = definitions.len();
                let defs_conflict = defs_count > 1;

                Ok(Some(serde_json::json!({
                    "kind": kind,
                    "name": raw_name.trim_start_matches('$'),
                    "definitions": definitions,
                    "definition_conflict": defs_conflict,
                    "refs": {
                        "project": project_locs,
                        "config": config_locs,
                    },
                    "usage_count": {
                        "project": project_locs.len(),
                        "config": config_locs.len(),
                        "total": project_locs.len() + config_locs.len(),
                    },
                    "css": css,
                    "value": value,
                    "usage": usage,
                    "example": example,
                    "source_path": source_path,
                    "animation_kind": animation_kind,
                    "group": group,
                })))
            }

            "grimoirecss.listVars" => {
                let silent = args.first().and_then(|v| v.as_bool()).unwrap_or(false);
                let vars = self.cached_list_grimoire_variables(&root).await?;

                if !silent {
                    let _ = self
                        .client
                        .log_message(
                            MessageType::INFO,
                            format!(
                                "Grimoire variables ({}):\n{}",
                                vars.len(),
                                vars.iter()
                                    .map(|v| format!("${} = {}", v.name, v.value))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            ),
                        )
                        .await;

                    let _ = self
                        .client
                        .show_message(
                            MessageType::INFO,
                            format!(
                                "GrimoireCSS: {} variable(s) (see Output → 'Grimoire CSS Server (...)')",
                                vars.len()
                            ),
                        )
                        .await;
                }

                Ok(Some(serde_json::to_value(vars).map_err(to_jsonrpc)?))
            }

            "grimoirecss.explain" => {
                let token = args.first().and_then(|v| v.as_str()).unwrap_or("");
                let silent = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

                if token.is_empty() {
                    return Err(JsonRpcError::invalid_params(
                        "explain expects first argument: token",
                    ));
                }

                let res = Analyzer::explain_class_token(&root, token).map_err(to_jsonrpc)?;

                let expanded = if res.expanded_spells.is_empty() {
                    "(none)".to_string()
                } else {
                    res.expanded_spells.join("\n")
                };

                if !silent {
                    let _ = self
                        .client
                        .log_message(
                            MessageType::INFO,
                            format!(
                                "Explain token: `{token}`\nExpanded spells:\n{expanded}\n\nCSS:\n{}",
                                res.css
                            ),
                        )
                        .await;

                    let _ = self
                        .client
                        .show_message(
                            MessageType::INFO,
                            "GrimoireCSS: explain done (see Output → 'Grimoire CSS Server (...)')"
                                .to_string(),
                        )
                        .await;
                }

                Ok(Some(serde_json::to_value(res).map_err(to_jsonrpc)?))
            }

            "grimoirecss.refs" => {
                let query = args.first().and_then(|v| v.as_str()).unwrap_or("");
                let silent = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

                if query.is_empty() {
                    return Err(JsonRpcError::invalid_params(
                        "refs expects first argument: query",
                    ));
                }

                let cfg = Analyzer::load_config(&root).map_err(to_jsonrpc)?;
                let mut results: Vec<serde_json::Value> = Vec::new();

                let is_dollar = query.starts_with('$');
                let var_name = query.trim_start_matches('$');

                let known_var = cfg
                    .variables
                    .as_ref()
                    .is_some_and(|vars| vars.iter().any(|(k, _)| k == var_name));
                let known_scroll = cfg.scrolls.as_ref().is_some_and(|m| m.contains_key(query));

                if is_dollar || known_var {
                    let refs = self
                        .cached_refs_grimoire_variable(&root, var_name)
                        .await
                        .unwrap_or_default();
                    if !refs.is_empty() {
                        results.push(serde_json::json!({
                            "kind": "var",
                            "name": var_name,
                            "refs": refs
                        }));
                    }
                }

                if !is_dollar && known_scroll {
                    let refs = self
                        .cached_refs_scroll(&root, query)
                        .await
                        .unwrap_or_default();
                    if !refs.is_empty() {
                        results.push(serde_json::json!({
                            "kind": "scroll",
                            "name": query,
                            "refs": refs
                        }));
                    }
                }

                if results.is_empty() && !is_dollar {
                    let refs = self
                        .cached_refs_spell(&root, query)
                        .await
                        .unwrap_or_default();
                    if !refs.is_empty() {
                        results.push(serde_json::json!({
                            "kind": "spell",
                            "name": query,
                            "refs": refs
                        }));
                    }
                }

                if !silent {
                    let _ = self
                        .client
                        .log_message(
                            MessageType::INFO,
                            format!("Refs for '{query}': {} result kind(s)", results.len()),
                        )
                        .await;
                }

                Ok(Some(
                    serde_json::json!({"query": query, "results": results}),
                ))
            }

            "grimoirecss.stats" => {
                // Prefer object args: { group?, top?, token?, silent? }
                let mut group: String = String::new();
                let mut top: usize = 30;
                let mut token: Option<String> = None;
                let mut silent: bool = false;

                if let Some(obj) = args.first().and_then(|v| v.as_object()) {
                    if let Some(g) = obj.get("group").and_then(|v| v.as_str()) {
                        group = g.to_string();
                    }
                    if let Some(t) = obj.get("top").and_then(|v| v.as_u64()) {
                        top = t as usize;
                    }
                    if let Some(tok) = obj.get("token").and_then(|v| v.as_str()) {
                        token = Some(tok.to_string());
                    }
                    if let Some(s) = obj.get("silent").and_then(|v| v.as_bool()) {
                        silent = s;
                    }
                } else {
                    if let Some(first) = args.first().and_then(|v| v.as_str()) {
                        // Back-compat and convenience:
                        // - if first arg is a known group, treat it as `group`
                        // - otherwise treat it as `token`
                        match first {
                            "spells" | "scrolls" | "vars" => group = first.to_string(),
                            _ => token = Some(first.to_string()),
                        }
                    }
                    if let Some(t) = args.get(1).and_then(|v| v.as_u64()) {
                        top = t as usize;
                    }
                    // Only read explicit token from arg #2 if arg #0 was a group.
                    if token.is_none()
                        && let Some(tok) = args.get(2).and_then(|v| v.as_str())
                    {
                        token = Some(tok.to_string());
                    }
                    if let Some(s) = args.get(3).and_then(|v| v.as_bool()) {
                        silent = s;
                    }
                }

                let cfg = Analyzer::load_config(&root).map_err(to_jsonrpc)?;
                let idx = Analyzer::index(&root, top).map_err(to_jsonrpc)?;

                let compute_spells = group.is_empty() || group == "spells";
                let compute_scrolls = group.is_empty() || group == "scrolls";
                let compute_vars = group.is_empty() || group == "vars";

                let mut out = serde_json::Map::new();
                out.insert("top".to_string(), serde_json::json!(top));

                if let Some(tok) = token {
                    let raw_var = tok.trim_start_matches('$');
                    let known_var = cfg
                        .variables
                        .as_ref()
                        .is_some_and(|vars| vars.iter().any(|(k, _)| k == raw_var));

                    if tok.starts_with('$') || known_var {
                        let refs = self
                            .cached_refs_grimoire_variable(&root, raw_var)
                            .await
                            .unwrap_or_default();
                        out.insert(
                            "token".to_string(),
                            serde_json::json!({"kind": "var", "name": raw_var, "count": refs.len()}),
                        );
                        return Ok(Some(serde_json::Value::Object(out)));
                    }

                    let known_scroll = cfg.scrolls.as_ref().is_some_and(|m| m.contains_key(&tok));
                    if known_scroll {
                        let count = idx
                            .scroll_references
                            .iter()
                            .filter(|r| r.scroll == tok)
                            .count();
                        out.insert(
                            "token".to_string(),
                            serde_json::json!({"kind": "scroll", "name": tok, "count": count}),
                        );
                        return Ok(Some(serde_json::Value::Object(out)));
                    }

                    let count = Analyzer::spell_count(&root, &tok).map_err(to_jsonrpc)?;
                    if count > 0 {
                        out.insert(
                            "token".to_string(),
                            serde_json::json!({"kind": "spell", "name": tok, "count": count}),
                        );
                        return Ok(Some(serde_json::Value::Object(out)));
                    }

                    out.insert(
                        "token".to_string(),
                        serde_json::json!({
                            "error": "Unknown token",
                            "hint": "Provide a scroll name, $var name, or a spell"
                        }),
                    );
                    return Ok(Some(serde_json::Value::Object(out)));
                }

                if compute_spells {
                    out.insert(
                        "spells".to_string(),
                        serde_json::json!(idx.top_expanded_spells),
                    );
                }

                if compute_scrolls {
                    let mut counts: std::collections::HashMap<String, u64> =
                        std::collections::HashMap::new();
                    for r in &idx.scroll_references {
                        *counts.entry(r.scroll.clone()).or_insert(0) += 1;
                    }
                    let mut items: Vec<crate::analyzer::SpellFrequency> = counts
                        .into_iter()
                        .map(|(spell, count)| crate::analyzer::SpellFrequency { spell, count })
                        .collect();
                    items.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.spell.cmp(&b.spell)));
                    items.truncate(top);
                    out.insert("scrolls".to_string(), serde_json::json!(items));
                }

                if compute_vars {
                    let mut vars: Vec<String> = cfg
                        .variables
                        .as_ref()
                        .map(|pairs| pairs.iter().map(|(k, _)| k.clone()).collect())
                        .unwrap_or_default();
                    vars.sort();
                    vars.dedup();

                    let mut items: Vec<crate::analyzer::SpellFrequency> = Vec::new();
                    for v in vars {
                        let refs = self
                            .cached_refs_grimoire_variable(&root, &v)
                            .await
                            .unwrap_or_default();
                        items.push(crate::analyzer::SpellFrequency {
                            spell: v,
                            count: refs.len() as u64,
                        });
                    }
                    items.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.spell.cmp(&b.spell)));
                    items.truncate(top);
                    out.insert("vars".to_string(), serde_json::json!(items));
                }

                if !silent {
                    let _ = self
                        .client
                        .log_message(MessageType::INFO, "Stats computed".to_string())
                        .await;
                }

                Ok(Some(serde_json::Value::Object(out)))
            }

            "grimoirecss.dryCandidates" => {
                let min_support = args
                    .first()
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .unwrap_or(2);
                let min_items = args
                    .get(1)
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .unwrap_or(2);
                let silent = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);

                let res =
                    Analyzer::dry_candidates(&root, min_support, min_items).map_err(to_jsonrpc)?;

                let mut msg = String::new();
                msg.push_str(&format!(
                    "DRY candidates (min_support={min_support}, min_items={min_items})\nFiles scanned: {}\nClass occurrences: {}\n\n",
                    res.files_scanned, res.class_occurrences
                ));

                for c in res.candidates.iter().take(30) {
                    msg.push_str(&format!(
                        "support={} tokens={}\n",
                        c.support,
                        c.tokens.join(" ")
                    ));
                    for occ in c.occurrences.iter().take(10) {
                        msg.push_str(&format!("  - {}:{}:{}\n", occ.file, occ.line, occ.column));
                    }
                    msg.push('\n');
                }

                if !silent {
                    let _ = self.client.log_message(MessageType::INFO, msg).await;
                    let _ = self
                        .client
                        .show_message(
                            MessageType::INFO,
                            "GrimoireCSS: DRY candidates ready (see Output → 'Grimoire CSS Server (...)')"
                                .to_string(),
                        )
                        .await;
                }

                Ok(Some(serde_json::to_value(res).map_err(to_jsonrpc)?))
            }

            "grimoirecss.dryCreateScroll" => {
                // Args: { scrollName: string, tokens: string[] }
                let Some(obj) = args.first().and_then(|v| v.as_object()) else {
                    return Err(JsonRpcError::invalid_params(
                        "dryCreateScroll expects first argument: { scrollName, tokens }",
                    ));
                };

                let scroll_name = obj
                    .get("scrollName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim();
                let tokens_val = obj.get("tokens");
                let tokens_arr = tokens_val
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut candidate_tokens: Vec<String> = tokens_arr
                    .into_iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();

                if scroll_name.is_empty() || candidate_tokens.is_empty() {
                    return Err(JsonRpcError::invalid_params(
                        "dryCreateScroll expects non-empty { scrollName, tokens[] }",
                    ));
                }

                // Conservative validation: scroll names match existing grammar.
                if !scroll_name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
                {
                    return Err(JsonRpcError::invalid_params(
                        "dryCreateScroll.scrollName must be [A-Za-z0-9_.-]",
                    ));
                }

                candidate_tokens.sort();
                candidate_tokens.dedup();

                let summary = Analyzer::config_summary(&root).map_err(to_jsonrpc)?;

                // Load current config (before applying edits) so we can expand candidate tokens
                // into actual property spells for the new scroll definition.
                let cfg = self.load_config().await?;

                let (extends_scrolls, expanded_scroll_spells) =
                    partition_dry_candidate_tokens_for_new_scroll(&cfg, &candidate_tokens)?;

                let config_path = if Path::new(&summary.config_path).is_absolute() {
                    PathBuf::from(&summary.config_path)
                } else {
                    root.join(&summary.config_path)
                };

                // Update config JSON: append scroll to main config.
                let config_content = std::fs::read_to_string(&config_path).map_err(to_jsonrpc)?;
                let mut json: serde_json::Value =
                    serde_json::from_str(&config_content).map_err(to_jsonrpc)?;

                // Ensure scrolls array exists.
                if json.get("scrolls").is_none() || json.get("scrolls").is_some_and(|v| v.is_null())
                {
                    json["scrolls"] = serde_json::Value::Array(Vec::new());
                }

                let scrolls = json
                    .get_mut("scrolls")
                    .and_then(|v| v.as_array_mut())
                    .ok_or_else(|| {
                        JsonRpcError::invalid_params("config.scrolls must be an array")
                    })?;

                if scrolls.iter().any(|s| {
                    s.get("name")
                        .and_then(|v| v.as_str())
                        .is_some_and(|n| n == scroll_name)
                }) {
                    return Err(JsonRpcError::invalid_params(format!(
                        "Scroll '{scroll_name}' already exists"
                    )));
                }

                let mut new_scroll = serde_json::json!({
                    "name": scroll_name,
                    "spells": expanded_scroll_spells,
                });

                if !extends_scrolls.is_empty() {
                    new_scroll["extends"] = serde_json::Value::Array(
                        extends_scrolls
                            .into_iter()
                            .map(serde_json::Value::String)
                            .collect(),
                    );
                }

                scrolls.push(new_scroll);

                let new_config = serde_json::to_string_pretty(&json)
                    .map(|s| format!("{s}\n"))
                    .map_err(to_jsonrpc)?;

                let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();

                // Whole-file replace for config (keeps implementation simple and deterministic).
                let uri = Url::from_file_path(&config_path).map_err(|_| {
                    to_jsonrpc(format!(
                        "Failed to create file URI for config path: {}",
                        config_path.display()
                    ))
                })?;
                let end_pos = offset_to_position_utf16(&config_content, config_content.len())
                    .unwrap_or(Position {
                        line: 0,
                        character: 0,
                    });
                changes.entry(uri).or_default().push(TextEdit {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: end_pos,
                    },
                    new_text: new_config,
                });

                // Replace occurrences in project files: for each regular class group that contains
                // all candidate tokens, replace one token with the new scroll and delete the rest.
                let parser = crate::core::parser::Parser::new();

                // Collect all input files from config summary.
                let mut project_inputs: Vec<PathBuf> = Vec::new();
                for p in &summary.projects {
                    for input in &p.input_paths {
                        let resolved = if Path::new(input).is_absolute() {
                            PathBuf::from(input)
                        } else {
                            root.join(input)
                        };
                        project_inputs.push(resolved);
                    }
                }

                let mut files = std::collections::HashSet::<PathBuf>::new();
                for input in &project_inputs {
                    if input.exists() && input.is_dir() {
                        let mut pat = input.to_string_lossy().to_string();
                        if !pat.ends_with('/') {
                            pat.push('/');
                        }
                        pat.push_str("**/*");
                        if let Ok(entries) = glob::glob(&pat) {
                            for p in entries.flatten() {
                                if p.is_file() {
                                    files.insert(p);
                                }
                            }
                        }
                    } else {
                        let pat = input.to_string_lossy().to_string();
                        if let Ok(entries) = glob::glob(&pat) {
                            for p in entries.flatten() {
                                if p.is_file() {
                                    files.insert(p);
                                }
                            }
                        }
                    }
                }

                let mut file_list: Vec<PathBuf> = files.into_iter().collect();
                file_list.sort();

                // Helper subset check: both sorted.
                let is_subset_sorted = |needles: &[String], haystack: &[String]| -> bool {
                    let mut i = 0usize;
                    let mut j = 0usize;
                    while i < needles.len() && j < haystack.len() {
                        match needles[i].cmp(&haystack[j]) {
                            std::cmp::Ordering::Less => return false,
                            std::cmp::Ordering::Greater => j += 1,
                            std::cmp::Ordering::Equal => {
                                i += 1;
                                j += 1;
                            }
                        }
                    }
                    i == needles.len()
                };

                for file_path in &file_list {
                    let Ok(content) = std::fs::read_to_string(file_path) else {
                        continue;
                    };

                    let mut groups: Vec<crate::core::parser::RegularClassGroup> = Vec::new();
                    if parser
                        .collect_regular_class_groups(&content, &mut groups)
                        .is_err()
                    {
                        continue;
                    }

                    let mut edits_for_file: Vec<TextEdit> = Vec::new();

                    for g in groups {
                        let mut spell_like: Vec<String> = Vec::new();
                        for (t, span) in g.tokens {
                            if t.is_empty() {
                                continue;
                            }
                            let parsed =
                                Spell::new(&t, &cfg.shared_spells, &cfg.scrolls, span, None);
                            if let Ok(Some(_)) = parsed {
                                spell_like.push(t);
                            }
                        }

                        if spell_like.is_empty() {
                            continue;
                        }

                        let mut norm: Vec<String> = spell_like;
                        norm.sort();
                        norm.dedup();

                        if !is_subset_sorted(&candidate_tokens, &norm) {
                            continue;
                        }

                        // Rewrite the entire attribute value to avoid leaving extra whitespace.
                        let (value_start, value_len) = g.value_span;
                        let value_end = value_start + value_len;
                        if value_end > content.len() {
                            continue;
                        }

                        let mut inserted = false;
                        let mut rebuilt: Vec<String> = Vec::new();

                        // Use the original token order (including non-spell tokens) but remove
                        // candidate tokens, replacing them with a single scroll invocation.
                        //
                        // `toks` only contains spell-like tokens, but we still want to preserve
                        // any other class tokens in the attribute value. We can derive the full
                        // token list from the value slice.
                        let original_value = &content[value_start..value_end];
                        for part in original_value.split_whitespace() {
                            if part.is_empty() {
                                continue;
                            }

                            if candidate_tokens.binary_search(&part.to_string()).is_ok() {
                                if !inserted {
                                    rebuilt.push(scroll_name.to_string());
                                    inserted = true;
                                }
                                continue;
                            }

                            rebuilt.push(part.to_string());
                        }

                        if !inserted {
                            continue;
                        }

                        let new_value = rebuilt.join(" ");
                        let Some(start_pos) = offset_to_position_utf16(&content, value_start)
                        else {
                            continue;
                        };
                        let Some(end_pos) = offset_to_position_utf16(&content, value_end) else {
                            continue;
                        };

                        edits_for_file.push(TextEdit {
                            range: Range {
                                start: start_pos,
                                end: end_pos,
                            },
                            new_text: new_value,
                        });
                    }

                    if !edits_for_file.is_empty()
                        && let Ok(uri) = Url::from_file_path(file_path)
                    {
                        changes.entry(uri).or_default().extend(edits_for_file);
                    }
                }

                let edit = WorkspaceEdit {
                    changes: Some(changes),
                    document_changes: None,
                    change_annotations: None,
                };

                Ok(Some(serde_json::to_value(edit).map_err(to_jsonrpc)?))
            }
            "grimoirecss.listScrolls" => {
                let silent = args.first().and_then(|v| v.as_bool()).unwrap_or(false);
                let cfg = Analyzer::load_config(&root).map_err(to_jsonrpc)?;
                let mut items: Vec<serde_json::Value> = Vec::new();

                if let Some(scrolls) = &cfg.scrolls {
                    let mut names: Vec<String> = scrolls.keys().cloned().collect();
                    names.sort();

                    for name in names {
                        let def = scrolls.get(&name);
                        let spells_len = def.map(|d| d.spells.len()).unwrap_or(0);
                        let spells_preview: Vec<String> =
                            def.map(|d| d.spells.clone()).unwrap_or_default();

                        let overloads: Vec<String> = def
                            .and_then(|d| d.spells_by_args.as_ref())
                            .map(|m| {
                                let mut ks: Vec<String> = m.keys().cloned().collect();
                                ks.sort();
                                ks
                            })
                            .unwrap_or_default();

                        let overload_spells_preview = def
                            .and_then(|d| d.spells_by_args.as_ref())
                            .map(|m| {
                                let mut ks: Vec<String> = m.keys().cloned().collect();
                                ks.sort();

                                let mut obj = serde_json::Map::new();
                                for k in ks {
                                    let v = m.get(&k).cloned().unwrap_or_default();
                                    obj.insert(
                                        k,
                                        serde_json::Value::Array(
                                            v.into_iter().map(serde_json::Value::String).collect(),
                                        ),
                                    );
                                }
                                serde_json::Value::Object(obj)
                            })
                            .unwrap_or(serde_json::Value::Null);

                        let loc = self.find_scroll_definition(&name).await.ok().flatten();
                        let loc_json = loc.as_ref().map(|l| {
                            serde_json::json!({
                                "uri": l.uri.to_string(),
                                "line": (l.range.start.line + 1),
                                "character": (l.range.start.character + 1)
                            })
                        });

                        items.push(serde_json::json!({
                            "name": name,
                            "spells": spells_len,
                            "spellsPreview": spells_preview,
                            "overloads": overloads,
                            "overloadSpellsPreview": overload_spells_preview,
                            "location": loc_json
                        }));
                    }
                }

                if !silent {
                    let preview = items
                        .iter()
                        .map(|v| {
                            let name = v
                                .get("name")
                                .and_then(|x| x.as_str())
                                .unwrap_or("(unknown)");
                            let spells = v.get("spellsPreview");
                            let spells_s = spells
                                .and_then(|x| x.as_array())
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|x| x.as_str())
                                        .collect::<Vec<_>>()
                                        .join(" ")
                                })
                                .unwrap_or_default();

                            let overloads = v
                                .get("overloads")
                                .and_then(|x| x.as_array())
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|x| x.as_str())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                })
                                .unwrap_or_default();

                            if overloads.is_empty() {
                                format!("- {name}\n  spells: {spells_s}")
                            } else {
                                format!("- {name}\n  spells: {spells_s}\n  overloads: {overloads}")
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n");

                    let _ = self
                        .client
                        .log_message(
                            MessageType::INFO,
                            format!("Scrolls ({}):\n{}", items.len(), preview),
                        )
                        .await;

                    let _ = self
                        .client
                        .show_message(
                            MessageType::INFO,
                            "GrimoireCSS: scroll list ready (see Output → 'Grimoire CSS Server (...)')"
                                .to_string(),
                        )
                        .await;
                }

                Ok(Some(serde_json::json!({"scrolls": items})))
            }
            "grimoirecss.index" => {
                let top = args
                    .first()
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .unwrap_or(50);

                let res = Analyzer::index(&root, top).map_err(to_jsonrpc)?;

                let top_spells = res
                    .top_expanded_spells
                    .iter()
                    .map(|s| format!("{} × {}", s.spell, s.count))
                    .collect::<Vec<_>>()
                    .join("\n");

                let _ = self
                    .client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "Index workspace\nFiles scanned: {}\nToken occurrences: {}\nScroll refs: {}\n\nTop {top} expanded spells:\n{top_spells}",
                            res.files_scanned,
                            res.token_occurrences,
                            res.scroll_references.len(),
                        ),
                    )
                    .await;

                let _ = self
                    .client
                    .show_message(
                        MessageType::INFO,
                        format!(
                            "GrimoireCSS: index done ({} files, {} tokens). See Output for details.",
                            res.files_scanned,
                            res.token_occurrences,
                        ),
                    )
                    .await;
                Ok(Some(serde_json::to_value(res).map_err(to_jsonrpc)?))
            }
            "grimoirecss.lint" => {
                let silent = args.first().and_then(|v| v.as_bool()).unwrap_or(false);
                let res = self.cached_lint(&root).await?;

                let lines = res
                    .errors
                    .iter()
                    .map(|m| format!("error {}: {}", m.code, m.message))
                    .chain(
                        res.warnings
                            .iter()
                            .map(|m| format!("warning {}: {}", m.code, m.message)),
                    )
                    .chain(
                        res.notes
                            .iter()
                            .map(|m| format!("note {}: {}", m.code, m.message)),
                    )
                    .collect::<Vec<_>>()
                    .join("\n");

                if !silent {
                    let _ = self
                        .client
                        .log_message(
                            MessageType::INFO,
                            format!(
                                "Lint workspace\nErrors: {}\nWarnings: {}\nNotes: {}\n\n{}",
                                res.errors.len(),
                                res.warnings.len(),
                                res.notes.len(),
                                lines
                            ),
                        )
                        .await;

                    let _ = self
                        .client
                        .show_message(
                            MessageType::INFO,
                            format!(
                                "GrimoireCSS: lint done ({} errors, {} warnings). See Output for details.",
                                res.errors.len(),
                                res.warnings.len()
                            ),
                        )
                        .await;
                }
                Ok(Some(serde_json::to_value(res).map_err(to_jsonrpc)?))
            }
            "grimoirecss.configSummary" => {
                let silent = args.first().and_then(|v| v.as_bool()).unwrap_or(false);
                let res = Analyzer::config_summary(&root).map_err(to_jsonrpc)?;

                if !silent {
                    let _ = self
                        .client
                        .log_message(
                            MessageType::INFO,
                            format!(
                                "Config summary\nprojects={} scrolls={} variables={} shared_spells={} custom_animations={} css_custom_properties={}\nconfig_path={}",
                                res.projects.len(),
                                res.scrolls.len(),
                                res.variables.len(),
                                res.shared_spells.len(),
                                res.custom_animations.len(),
                                res.css_custom_properties.len(),
                                res.config_path,
                            ),
                        )
                        .await;
                }

                Ok(Some(serde_json::to_value(res).map_err(to_jsonrpc)?))
            }
            _ => Err(JsonRpcError::method_not_found()),
        }
    }
}

impl Backend {
    async fn format_spell_or_scroll_hover_markdown(
        &self,
        root: &Path,
        _cfg: &crate::config::ConfigFs,
        token: &str,
        spell: &Spell,
    ) -> String {
        let _token_inline = format!("`{}`", token);

        if let Some(scroll_spells) = &spell.scroll_spells
            && !spell.component().is_empty()
        {
            let _scroll_name = spell.component();
            let _args_count = if spell.component_target().is_empty() {
                0
            } else {
                spell.component_target().split('_').count()
            };

            let mut out = String::new();

            if let Some(css) = self.try_explain_css_for_token(root, token).await {
                out.push_str("```css\n");
                out.push_str(&css);
                if !css.ends_with('\n') {
                    out.push('\n');
                }
                out.push_str("```\n\n");
            } else {
                let decls = scroll_spells
                    .iter()
                    .flat_map(spell_to_css_declarations)
                    .collect::<Vec<String>>();
                if !decls.is_empty() {
                    out.push_str("```css\n");
                    for d in decls {
                        out.push_str(&d);
                        out.push('\n');
                    }
                    out.push_str("```\n\n");
                }
            }

            return out;
        }

        // Template spell.
        if let Some(_expanded) = &spell.scroll_spells {
            let mut out = String::new();

            if let Some(css) = self.try_explain_css_for_token(root, token).await {
                out.push_str("```css\n");
                out.push_str(&css);
                if !css.ends_with('\n') {
                    out.push('\n');
                }
                out.push_str("```\n\n");
            }

            return out;
        }

        // Plain spell.
        let mut out = String::new();

        // If this token is a real class token, we can compile and show actual CSS.
        if let Some(css) = self.try_explain_css_for_token(root, token).await {
            out.push_str("```css\n");
            out.push_str(&css);
            if !css.ends_with('\n') {
                out.push('\n');
            }
            out.push_str("```\n\n");
        } else {
            let decls = spell_to_css_declarations(spell);
            if !decls.is_empty() {
                out.push_str("```css\n");
                for d in decls {
                    out.push_str(&d);
                    out.push('\n');
                }
                out.push_str("```\n\n");
            }
        }

        out
    }

    async fn try_explain_css_for_token(&self, root: &Path, token: &str) -> Option<String> {
        let explained = Analyzer::explain_class_token(root, token).ok()?;
        Some(explained.css)
    }
}

fn spell_to_css_declarations(spell: &Spell) -> Vec<String> {
    let component = spell.component();
    let target = spell.component_target();
    if component.is_empty() || target.is_empty() {
        return vec![];
    }

    // Prefer mapped full property name (handles abbreviations).
    let prop = crate::component::get_css_property(component).unwrap_or(component);
    vec![format!("{prop}: {target};")]
}

fn format_grimoire_var_hover_markdown(cfg: &crate::config::ConfigFs, name: &str) -> String {
    let Some(vars) = &cfg.variables else {
        return String::new();
    };

    let Some((_, value)) = vars.iter().find(|(k, _)| k == name) else {
        return String::new();
    };

    let mut out = String::new();

    out.push_str(value);
    out
}

fn extract_token_at_byte_offset(
    content: &str,
    byte_offset: usize,
) -> Option<(String, usize, usize)> {
    if byte_offset >= content.len() {
        return None;
    }

    let is_token_char = |ch: char| {
        !(ch.is_whitespace()
            || matches!(
                ch,
                '"' | '\''
                    | '`'
                    | '<'
                    | '>'
                    | '{'
                    | '}'
                    | '('
                    | ')'
                    | '['
                    | ']'
                    | ','
                    | ';'
                    | '+'
                    | '*'
                    | '&'
                    | '|'
                    | '?'
                    | '\n'
                    | '\r'
                    | '\t'
            ))
    };

    // Find the char boundary for the given byte offset.
    let mut start = byte_offset;
    while start > 0 {
        let prev = content[..start].chars().next_back()?;
        let prev_len = prev.len_utf8();
        let prev_start = start.saturating_sub(prev_len);
        if prev_start >= start {
            break;
        }
        if !is_token_char(prev) {
            break;
        }
        start = prev_start;
    }

    let mut end = byte_offset;
    // Ensure we include the char at `byte_offset`.
    if let Some(ch) = content[byte_offset..].chars().next() {
        end = byte_offset + ch.len_utf8();
    }

    while end < content.len() {
        let ch = content[end..].chars().next()?;
        if !is_token_char(ch) {
            break;
        }
        end += ch.len_utf8();
    }

    if start >= end {
        return None;
    }

    // Template tokens look like `g!...;`. Our generic token scan treats `;` as a delimiter, so
    // when the cursor is inside a template we would otherwise extract `g!...` (missing the `;`)
    // and `Spell::check_for_template` wouldn't trigger.
    if content[start..end].starts_with("g!")
        && end < content.len()
        && content.as_bytes().get(end) == Some(&b';')
    {
        end += 1; // safe: ';' is ASCII (1 byte)
    }

    let token = content[start..end].to_string();
    Some((token, start, end - start))
}

fn extract_json_string_literal_at_byte_offset(
    content: &str,
    byte_offset: usize,
) -> Option<(String, usize, usize)> {
    if byte_offset >= content.len() {
        return None;
    }

    let bytes = content.as_bytes();

    // Find opening quote to the left (unescaped).
    let mut start_quote: Option<usize> = None;
    let mut i = byte_offset;
    while i > 0 {
        i -= 1;
        if bytes[i] == b'"' {
            let mut backslashes = 0usize;
            let mut j = i;
            while j > 0 {
                j -= 1;
                if bytes[j] == b'\\' {
                    backslashes += 1;
                } else {
                    break;
                }
            }
            if backslashes.is_multiple_of(2) {
                start_quote = Some(i);
                break;
            }
        }
    }

    let start_quote = start_quote?;

    // Find closing quote to the right (unescaped).
    let mut end_quote: Option<usize> = None;
    let mut k = start_quote + 1;
    while k < bytes.len() {
        if bytes[k] == b'"' {
            let mut backslashes = 0usize;
            let mut j = k;
            while j > start_quote {
                j -= 1;
                if bytes[j] == b'\\' {
                    backslashes += 1;
                } else {
                    break;
                }
                if j == 0 {
                    break;
                }
            }
            if backslashes.is_multiple_of(2) {
                end_quote = Some(k);
                break;
            }
        }
        k += 1;
    }

    let end_quote = end_quote?;

    // Ensure the cursor is actually inside the string.
    if byte_offset <= start_quote || byte_offset >= end_quote {
        return None;
    }

    let literal = &content[start_quote..=end_quote];
    let decoded: String = serde_json::from_str(literal).ok()?;

    // Return range excluding quotes.
    let start = start_quote + 1;
    let len = end_quote.saturating_sub(start_quote + 1);
    Some((decoded, start, len))
}

fn extract_dollar_identifier_at(content: &str, byte_offset: usize) -> Option<String> {
    extract_prefixed_identifier_at(content, byte_offset, '$')
        .map(|s| s.trim_start_matches('$').to_string())
}

fn extract_prefixed_identifier_at(
    content: &str,
    byte_offset: usize,
    prefix_char: char,
) -> Option<String> {
    if byte_offset >= content.len() {
        return None;
    }

    let is_ident = |ch: char| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-';

    // Walk left to find start of ident or prefix.
    let mut start = byte_offset;
    while start > 0 {
        let prev = content[..start].chars().next_back()?;
        if is_ident(prev) || prev == prefix_char {
            start = start.saturating_sub(prev.len_utf8());
            continue;
        }
        break;
    }

    // Walk right to end of ident.
    let mut end = byte_offset;
    if let Some(ch) = content[byte_offset..].chars().next() {
        end = byte_offset + ch.len_utf8();
    }
    while end < content.len() {
        let ch = content[end..].chars().next()?;
        if is_ident(ch) {
            end += ch.len_utf8();
        } else {
            break;
        }
    }

    let slice = &content[start..end];
    // Find the nearest prefix within slice that starts an identifier.
    if let Some(pos) = slice.rfind(prefix_char) {
        let s = &slice[pos..];
        if s.len() >= 2 {
            return Some(s.to_string());
        }
    }

    None
}

fn occurrence_to_location(root: &Path, occ: &crate::analyzer::TokenOccurrence) -> Option<Location> {
    let abs = root.join(&occ.file);
    let uri = Url::from_file_path(&abs).ok()?;
    let content = std::fs::read_to_string(&abs).ok()?;

    let start = occ.byte_offset;
    let end = occ.byte_offset.saturating_add(occ.byte_len);

    let start_pos = offset_to_position_utf16(&content, start)?;
    let end_pos = offset_to_position_utf16(&content, end)?;

    Some(Location {
        uri,
        range: Range {
            start: start_pos,
            end: end_pos,
        },
    })
}

fn to_jsonrpc<E: std::fmt::Display>(e: E) -> JsonRpcError {
    let message = e.to_string();
    let mut err = JsonRpcError::internal_error();
    err.message = message.clone().into();
    err.data = Some(serde_json::json!({"error": message}));
    err
}

fn partition_dry_candidate_tokens_for_new_scroll(
    cfg: &crate::config::ConfigFs,
    candidate_tokens: &[String],
) -> Result<(Vec<String>, Vec<String>), JsonRpcError> {
    // We always emit extracted content into `spells`.
    //
    // Rationale:
    // - `extends` supports only plain scroll names (no args / no prefixes), which makes extraction
    //   logic surprising when a cluster mixes `box` and `hover:box=4px`.
    // - With nested scroll invocations supported in scroll spell lists, keeping invocations in
    //   `spells` is both simpler and safer.
    let mut spells: Vec<String> = Vec::new();
    let mut seen_spells: std::collections::HashSet<String> = std::collections::HashSet::new();

    for t in candidate_tokens {
        let parsed = Spell::new(t, &cfg.shared_spells, &cfg.scrolls, (0, 0), None);
        let Ok(Some(spell)) = parsed else {
            continue;
        };

        // Use the original token for output stability (keeps prefixes/args as the user wrote them).
        let out = if spell.with_template {
            // DRY candidates currently come from regular class groups, but keep this robust.
            spell.raw_spell.clone()
        } else {
            t.clone()
        };

        if seen_spells.insert(out.clone()) {
            spells.push(out);
        }
    }

    if spells.is_empty() {
        return Err(JsonRpcError::invalid_params(
            "dryCreateScroll: candidate tokens produced no spells",
        ));
    }

    Ok((Vec::new(), spells))
}

fn find_scroll_name_value_in_scrolls_array(
    content: &str,
    scroll_name: &str,
) -> Option<(usize, usize)> {
    let (_, (arr_start, arr_end)) = find_field_value_array_range(content, "scrolls")?;

    // Iterate top-level objects within the array.
    let mut i = arr_start;
    while i < arr_end {
        // Find next object start.
        let bytes = content.as_bytes();
        while i < arr_end {
            let b = bytes[i];
            if b == b'{' {
                break;
            }
            i += 1;
        }
        if i >= arr_end {
            break;
        }

        let obj_start = i;
        let obj_end = find_matching_brace(content, obj_start)?;

        if let Some(found) = find_object_member_value_string_in_range(
            content,
            obj_start,
            obj_end,
            "name",
            scroll_name,
        ) {
            return Some(found);
        }

        i = obj_end;
    }

    None
}

fn find_object_member_value_string_in_range(
    content: &str,
    obj_start: usize,
    obj_end: usize,
    member_key: &str,
    wanted_value: &str,
) -> Option<(usize, usize)> {
    let mut i = obj_start;
    while i < obj_end {
        let (_key_start, key_end, key_value) = next_json_string(content, i, obj_end)?;

        if key_value == member_key {
            let mut j = key_end;
            skip_ws(content, &mut j, obj_end);
            if j >= obj_end || content.as_bytes().get(j) != Some(&b':') {
                i = key_end;
                continue;
            }
            j += 1;
            skip_ws(content, &mut j, obj_end);

            let (val_start, val_end, val_value) = next_json_string(content, j, obj_end)?;
            if val_value == wanted_value {
                // return range of the quoted value string
                return Some((val_start, val_end));
            }

            i = val_end;
            continue;
        }

        i = key_end;
    }

    None
}

fn find_object_member_key_in_named_object(
    content: &str,
    object_field_name: &str,
    wanted_key: &str,
) -> Option<(usize, usize)> {
    let (_, (obj_start, obj_end)) = find_field_value_object_range(content, object_field_name)?;
    find_object_member_key_in_range(content, obj_start, obj_end, wanted_key)
}

fn find_object_member_key_in_range(
    content: &str,
    obj_start: usize,
    obj_end: usize,
    wanted_key: &str,
) -> Option<(usize, usize)> {
    let wanted = serde_json::to_string(wanted_key).ok()?;
    let mut i = obj_start;
    while i < obj_end {
        let Some((s_start, s_end, s_value)) = next_json_string(content, i, obj_end) else {
            break;
        };
        // Check if this string equals the wanted key AND is used as an object member key:
        //   "key" : <value>
        if s_value == wanted_key {
            let mut j = s_end;
            skip_ws(content, &mut j, obj_end);
            if j < obj_end && content.as_bytes().get(j) == Some(&b':') {
                // Return range covering the quoted key.
                // Use the already-found JSON string boundaries, not a substring search.
                // (s_start..s_end) includes the quotes.
                let _ = wanted; // keep in sync if we ever change encoding strategy
                return Some((s_start, s_end));
            }
        }
        i = s_end;
    }
    None
}

fn find_field_value_object_range(
    content: &str,
    field_name: &str,
) -> Option<(usize, (usize, usize))> {
    find_field_value_object_range_from(content, field_name, 0)
}

fn find_field_value_object_range_from(
    content: &str,
    field_name: &str,
    start_from: usize,
) -> Option<(usize, (usize, usize))> {
    let bytes = content.as_bytes();

    let mut i = start_from;
    while i < content.len() {
        // Find next string.
        let (s_start, s_end, s_value) = next_json_string(content, i, content.len())?;
        i = s_end;

        if s_value != field_name {
            continue;
        }

        // Ensure it's a member key: "field" : { ... }
        let mut j = s_end;
        skip_ws(content, &mut j, content.len());
        if bytes.get(j) != Some(&b':') {
            continue;
        }
        j += 1;
        skip_ws(content, &mut j, content.len());

        if bytes.get(j) != Some(&b'{') {
            continue;
        }

        let obj_start = j;
        let obj_end = find_matching_brace(content, obj_start)?;
        return Some((s_start, (obj_start, obj_end)));
    }
    None
}

fn find_field_value_array_range(
    content: &str,
    field_name: &str,
) -> Option<(usize, (usize, usize))> {
    find_field_value_array_range_from(content, field_name, 0)
}

fn find_field_value_array_range_from(
    content: &str,
    field_name: &str,
    start_from: usize,
) -> Option<(usize, (usize, usize))> {
    let bytes = content.as_bytes();

    let mut i = start_from;
    while i < content.len() {
        let (s_start, s_end, s_value) = next_json_string(content, i, content.len())?;
        i = s_end;

        if s_value != field_name {
            continue;
        }

        let mut j = s_end;
        skip_ws(content, &mut j, content.len());
        if bytes.get(j) != Some(&b':') {
            continue;
        }
        j += 1;
        skip_ws(content, &mut j, content.len());

        if bytes.get(j) != Some(&b'[') {
            continue;
        }

        let arr_start = j;
        let arr_end = find_matching_bracket(content, arr_start)?;
        return Some((s_start, (arr_start, arr_end)));
    }
    None
}

fn skip_ws(content: &str, i: &mut usize, end: usize) {
    while *i < end {
        let b = content.as_bytes()[*i];
        if b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' {
            *i += 1;
        } else {
            break;
        }
    }
}

fn next_json_string(content: &str, mut i: usize, end: usize) -> Option<(usize, usize, String)> {
    let bytes = content.as_bytes();
    // Walk until next quote, skipping over non-strings and over full strings.
    while i < end {
        if bytes[i] == b'"' {
            let start = i;
            i += 1;
            let mut out = String::new();
            while i < end {
                match bytes[i] {
                    b'"' => {
                        let finish = i + 1;
                        return Some((start, finish, out));
                    }
                    b'\\' => {
                        i += 1;
                        if i >= end {
                            return None;
                        }
                        // Minimal escape handling: enough to keep scanning stable.
                        let esc = bytes[i];
                        match esc {
                            b'"' => out.push('"'),
                            b'\\' => out.push('\\'),
                            b'/' => out.push('/'),
                            b'b' => out.push('\u{0008}'),
                            b'f' => out.push('\u{000C}'),
                            b'n' => out.push('\n'),
                            b'r' => out.push('\r'),
                            b't' => out.push('\t'),
                            b'u' => {
                                // Skip 4 hex digits; don't decode fully (we just need scanning).
                                // If malformed, bail.
                                if i + 4 >= end {
                                    return None;
                                }
                                i += 4;
                            }
                            _ => {}
                        }
                        i += 1;
                    }
                    b => {
                        out.push(b as char);
                        i += 1;
                    }
                }
            }
            return None;
        }
        i += 1;
    }
    None
}

fn find_matching_brace(content: &str, open_brace_offset: usize) -> Option<usize> {
    let bytes = content.as_bytes();
    if bytes.get(open_brace_offset) != Some(&b'{') {
        return None;
    }

    let mut depth: i64 = 0;
    let mut i = open_brace_offset;
    let mut in_string = false;
    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match b {
            b'"' => {
                in_string = true;
            }
            b'{' => {
                depth += 1;
            }
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }

        i += 1;
    }

    None
}

fn find_matching_bracket(content: &str, open_bracket_offset: usize) -> Option<usize> {
    let bytes = content.as_bytes();
    if bytes.get(open_bracket_offset) != Some(&b'[') {
        return None;
    }

    let mut depth: i64 = 0;
    let mut i = open_bracket_offset;
    let mut in_string = false;
    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match b {
            b'"' => {
                in_string = true;
            }
            b'[' => {
                depth += 1;
            }
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }

        i += 1;
    }

    None
}

fn position_to_offset_utf16(content: &str, pos: Position) -> Option<usize> {
    let mut line: u32 = 0;
    let mut line_start: usize = 0;

    for (i, ch) in content.char_indices() {
        if line == pos.line {
            // We are on the target line; compute byte offset for UTF-16 character index.
            let line_str = &content[line_start..];
            let line_end = line_str
                .find('\n')
                .map(|d| line_start + d)
                .unwrap_or(content.len());
            return utf16_col_to_offset(&content[line_start..line_end], pos.character)
                .map(|in_line| line_start + in_line);
        }

        if ch == '\n' {
            line += 1;
            line_start = i + ch.len_utf8();
        }
    }

    if line == pos.line {
        let line_end = content.len();
        return utf16_col_to_offset(&content[line_start..line_end], pos.character)
            .map(|in_line| line_start + in_line);
    }

    None
}

fn utf16_col_to_offset(line: &str, col_utf16: u32) -> Option<usize> {
    let mut col: u32 = 0;
    for (byte_idx, ch) in line.char_indices() {
        if col == col_utf16 {
            return Some(byte_idx);
        }
        col += ch.len_utf16() as u32;
        if col > col_utf16 {
            return Some(byte_idx + ch.len_utf8());
        }
    }

    if col == col_utf16 {
        Some(line.len())
    } else {
        None
    }
}

fn offset_to_position_utf16(content: &str, byte_offset: usize) -> Option<Position> {
    if byte_offset > content.len() {
        return None;
    }

    let mut line: u32 = 0;
    let mut line_start: usize = 0;

    for (i, ch) in content.char_indices() {
        if i >= byte_offset {
            let col_utf16 = utf16_len(&content[line_start..byte_offset]);
            return Some(Position {
                line,
                character: col_utf16,
            });
        }

        if ch == '\n' {
            line += 1;
            line_start = i + ch.len_utf8();
        }
    }

    if byte_offset == content.len() {
        let col_utf16 = utf16_len(&content[line_start..byte_offset]);
        return Some(Position {
            line,
            character: col_utf16,
        });
    }

    None
}

fn utf16_len(s: &str) -> u32 {
    s.chars().map(|c| c.len_utf16() as u32).sum()
}

pub async fn serve_stdio() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        state: Arc::new(RwLock::new(State::default())),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
