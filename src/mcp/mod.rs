use crate::{
    analyzer::Analyzer, build_with_options, component, init, shorten,
    transmutator::TransmuteOptions,
};
use serde::Serialize;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

const PROTOCOL_VERSION: &str = "2025-11-25";

pub struct McpServer {
    root: PathBuf,
}

impl McpServer {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn dispatch(&self, request: Value) -> Option<Value> {
        let Some(request) = request.as_object() else {
            return Some(error_response(Value::Null, -32600, "Invalid Request"));
        };
        let id = request.get("id").cloned();
        let response_id = id.clone().unwrap_or(Value::Null);
        let valid_id = id
            .as_ref()
            .is_none_or(|id| id.is_null() || id.is_string() || id.is_number());
        let Some(method) = request.get("method").and_then(Value::as_str) else {
            return Some(error_response(response_id, -32600, "Invalid Request"));
        };
        if request.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
            return Some(error_response(response_id, -32600, "Invalid Request"));
        }
        if !valid_id {
            return Some(error_response(Value::Null, -32600, "Invalid Request"));
        }
        let params = request.get("params").cloned().unwrap_or_else(|| json!({}));
        let result = match method {
            "initialize" => Ok(json!({
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {"resources": {}, "tools": {}},
                "serverInfo": {"name": "grimoire-css", "version": env!("CARGO_PKG_VERSION")},
                "instructions":"Use Grimoire CSS resources and tools instead of guessing. Before presenting Grimoire CSS code, call grimoire_validate_spells for every proposed spell. Use grimoire_transmute_css for CSS migration and grimoire_import_css only for an explicit project import. After creating or changing config or project files, call grimoire_validate_config and then grimoire_check_project. Do not claim completion unless the relevant validation result has valid=true."
            })),
            "ping" => Ok(json!({})),
            "resources/list" => Ok(resources()),
            "resources/read" => self.read_resource(&params),
            "tools/list" => Ok(json!({"tools": tools()})),
            "tools/call" => self.call_tool(&params),
            _ => return id.map(|id| error_response(id, -32601, "Method not found")),
        };
        let id = id?;
        Some(match result {
            Ok(result) => json!({"jsonrpc":"2.0","id":id,"result":result}),
            Err(message) => error_response(id, -32602, &message),
        })
    }

    fn read_resource(&self, params: &Value) -> Result<Value, String> {
        match string_arg(params, "uri")? {
            "grimoire://primer" => Ok(json!({"contents":[{
                "uri":"grimoire://primer",
                "mimeType":"text/markdown",
                "text": include_str!("primer.md")
            }]})),
            "grimoire://components" => {
                let text = serde_json::to_string_pretty(&component::get_all_components_map())
                    .map_err(|error| error.to_string())?;
                Ok(json!({"contents":[{
                    "uri":"grimoire://components",
                    "mimeType":"application/json",
                    "text":text
                }]}))
            }
            "grimoire://config-schema" => Ok(json!({"contents":[{
                "uri":"grimoire://config-schema",
                "mimeType":"application/schema+json",
                "text":include_str!("../core/config/config-schema.json")
            }]})),
            "grimoire://documentation" => Ok(json!({"contents":[{
                "uri":"grimoire://documentation",
                "mimeType":"text/markdown",
                "text":include_str!("../../README.md")
            }]})),
            _ => Err("Unknown Grimoire CSS resource".to_string()),
        }
    }

    fn call_tool(&self, params: &Value) -> Result<Value, String> {
        crate::buffer::discard_messages();
        let result = self.call_tool_inner(params);
        crate::buffer::discard_messages();
        result
    }

    fn call_tool_inner(&self, params: &Value) -> Result<Value, String> {
        let name = string_arg(params, "name")?;
        let arguments = params
            .get("arguments")
            .cloned()
            .unwrap_or_else(|| json!({}));
        match name {
            "grimoire_explain" => {
                exact_keys(&arguments, &["token"])?;
                domain_result(Analyzer::explain_class_token(
                    &self.root,
                    string_arg(&arguments, "token")?,
                ))
            }
            "grimoire_config_summary" => {
                exact_keys(&arguments, &[])?;
                domain_result(Analyzer::config_summary(&self.root))
            }
            "grimoire_index" => {
                exact_keys(&arguments, &["top"])?;
                domain_result(Analyzer::index(
                    &self.root,
                    usize_arg(&arguments, "top", 30)?,
                ))
            }
            "grimoire_lint" => {
                exact_keys(&arguments, &[])?;
                domain_result(Analyzer::lint(&self.root))
            }
            "grimoire_dry" => {
                exact_keys(&arguments, &["min_support", "min_items"])?;
                domain_result(Analyzer::dry_candidates(
                    &self.root,
                    usize_arg(&arguments, "min_support", 3)?,
                    usize_arg(&arguments, "min_items", 2)?,
                ))
            }
            "grimoire_list_variables" => {
                exact_keys(&arguments, &[])?;
                domain_result(Analyzer::list_grimoire_variables(&self.root))
            }
            "grimoire_list_scrolls" => {
                exact_keys(&arguments, &[])?;
                domain_result(Analyzer::config_summary(&self.root).map(|summary| summary.scrolls))
            }
            "grimoire_refs" => {
                exact_keys(&arguments, &["kind", "query"])?;
                let query = string_arg(&arguments, "query")?;
                match string_arg(&arguments, "kind")? {
                    "spell" => domain_result(Analyzer::refs_spell(&self.root, query)),
                    "scroll" => domain_result(Analyzer::refs_scroll(&self.root, query)),
                    "variable" => domain_result(Analyzer::refs_grimoire_variable(
                        &self.root,
                        query.trim_start_matches('$'),
                    )),
                    _ => Err("'kind' must be spell, scroll, or variable".to_string()),
                }
            }
            "grimoire_stats_spells" => {
                exact_keys(&arguments, &["top"])?;
                domain_result(Analyzer::stats_spells(
                    &self.root,
                    usize_arg(&arguments, "top", 30)?,
                ))
            }
            "grimoire_stats" => {
                exact_keys(&arguments, &["group", "token", "top"])?;
                domain_result(Analyzer::stats(
                    &self.root,
                    optional_string_arg(&arguments, "group")?,
                    optional_string_arg(&arguments, "token")?,
                    usize_arg(&arguments, "top", 30)?,
                ))
            }
            "grimoire_refs_auto" => {
                exact_keys(&arguments, &["query"])?;
                domain_result(Analyzer::refs(&self.root, string_arg(&arguments, "query")?))
            }
            "grimoire_validate_config" => {
                exact_keys(&arguments, &[])?;
                domain_result(Analyzer::validate_config(&self.root))
            }
            "grimoire_validate_spells" => {
                exact_keys(&arguments, &["tokens"])?;
                domain_result(Analyzer::validate_spells(
                    &self.root,
                    &string_array_arg(&arguments, "tokens")?,
                ))
            }
            "grimoire_check_project" => {
                exact_keys(&arguments, &[])?;
                domain_result(Analyzer::check_project(&self.root))
            }
            "grimoire_transmute_css" => {
                exact_keys(&arguments, &["content", "with_oneliner"])?;
                domain_result(Analyzer::transmute_and_validate(
                    &self.root,
                    string_arg(&arguments, "content")?,
                    TransmuteOptions {
                        with_oneliner: bool_arg(&arguments, "with_oneliner", false)?,
                    },
                ))
            }
            "grimoire_import_css" => {
                exact_keys(
                    &arguments,
                    &[
                        "content",
                        "paths",
                        "import_name",
                        "with_oneliner",
                        "replace",
                    ],
                )?;
                let content = optional_string_arg(&arguments, "content")?;
                let paths = optional_string_array_arg(&arguments, "paths")?;
                domain_result(Analyzer::import_css(
                    &self.root,
                    content,
                    paths.as_deref(),
                    string_arg(&arguments, "import_name")?,
                    TransmuteOptions {
                        with_oneliner: bool_arg(&arguments, "with_oneliner", false)?,
                    },
                    bool_arg(&arguments, "replace", false)?,
                ))
            }
            "grimoire_init" => {
                exact_keys(&arguments, &[])?;
                domain_result(init(&self.root))
            }
            "grimoire_build" => {
                exact_keys(&arguments, &["force_version_update"])?;
                domain_result(build_with_options(
                    &self.root,
                    bool_arg(&arguments, "force_version_update", false)?,
                ))
            }
            "grimoire_shorten" => {
                exact_keys(&arguments, &[])?;
                domain_result(shorten(&self.root))
            }
            _ => Err("Unknown Grimoire CSS tool".to_string()),
        }
    }
}

fn resources() -> Value {
    json!({"resources":[
        {"uri":"grimoire://primer","name":"Grimoire CSS primer","mimeType":"text/markdown"},
        {"uri":"grimoire://components","name":"Grimoire CSS components","mimeType":"application/json"},
        {"uri":"grimoire://config-schema","name":"Grimoire CSS config schema","mimeType":"application/schema+json"},
        {"uri":"grimoire://documentation","name":"Grimoire CSS documentation","mimeType":"text/markdown"}
    ]})
}

fn tools() -> Vec<Value> {
    vec![
        tool(
            "grimoire_explain",
            "Validate and explain a spell or scroll using the Grimoire CSS engine",
            object_schema(json!({"token":{"type":"string","minLength":1}}), &["token"]),
            true,
        ),
        tool(
            "grimoire_config_summary",
            "Read the existing Grimoire CSS configuration summary",
            object_schema(json!({}), &[]),
            true,
        ),
        tool(
            "grimoire_index",
            "Index Grimoire CSS tokens using the existing analyzer",
            object_schema(json!({"top":{"type":"integer","minimum":1}}), &[]),
            true,
        ),
        tool(
            "grimoire_lint",
            "Lint the project using the existing analyzer",
            object_schema(json!({}), &[]),
            true,
        ),
        tool(
            "grimoire_dry",
            "Find repeated token groups using the existing fi dry analysis",
            object_schema(
                json!({"min_support":{"type":"integer","minimum":1},"min_items":{"type":"integer","minimum":1}}),
                &[],
            ),
            true,
        ),
        tool(
            "grimoire_list_variables",
            "List configured Grimoire variables",
            object_schema(json!({}), &[]),
            true,
        ),
        tool(
            "grimoire_list_scrolls",
            "List configured Grimoire scrolls",
            object_schema(json!({}), &[]),
            true,
        ),
        tool(
            "grimoire_refs",
            "Find exact project references through the existing analyzer",
            object_schema(
                json!({
                    "kind":{"type":"string","enum":["spell","scroll","variable"]},
                    "query":{"type":"string","minLength":1}
                }),
                &["kind", "query"],
            ),
            true,
        ),
        tool(
            "grimoire_stats_spells",
            "Return the most frequent expanded spells from the existing analyzer",
            object_schema(json!({"top":{"type":"integer","minimum":1}}), &[]),
            true,
        ),
        tool(
            "grimoire_stats",
            "Return the existing fi statistics for spells, scrolls, variables, or one token",
            object_schema(
                json!({
                    "group":{"type":"string","enum":["all","spells","scrolls","vars"]},
                    "token":{"type":"string","minLength":1},
                    "top":{"type":"integer","minimum":1}
                }),
                &[],
            ),
            true,
        ),
        tool(
            "grimoire_refs_auto",
            "Resolve a reference query with the existing fi spell, scroll, and variable rules",
            object_schema(json!({"query":{"type":"string","minLength":1}}), &["query"]),
            true,
        ),
        tool(
            "grimoire_validate_config",
            "Validate the main and external project configs against the official schema and real engine loader",
            object_schema(json!({}), &[]),
            true,
        ),
        tool(
            "grimoire_validate_spells",
            "Validate proposed spells and scrolls through the real parser and CSS generator",
            object_schema(
                json!({
                    "tokens":{
                        "type":"array",
                        "minItems":1,
                        "maxItems":256,
                        "items":{"type":"string","minLength":1}
                    }
                }),
                &["tokens"],
            ),
            true,
        ),
        tool(
            "grimoire_check_project",
            "Run config validation, spell indexing, lint, and the real Grimoire CSS build",
            object_schema(json!({}), &[]),
            false,
        ),
        tool(
            "grimoire_transmute_css",
            "Convert inline CSS to typed Grimoire scrolls and validate every generated spell",
            object_schema(
                json!({
                    "content":{"type":"string","minLength":1},
                    "with_oneliner":{"type":"boolean","default":false}
                }),
                &["content"],
            ),
            true,
        ),
        tool(
            "grimoire_import_css",
            "Convert CSS into an external scroll file, verify the project, and roll back on failure",
            json!({
                "type":"object",
                "properties":{
                    "content":{"type":"string","minLength":1},
                    "paths":{
                        "type":"array",
                        "minItems":1,
                        "maxItems":256,
                        "items":{"type":"string","minLength":1}
                    },
                    "import_name":{
                        "type":"string",
                        "minLength":1,
                        "pattern":"^[A-Za-z0-9_-]+$"
                    },
                    "with_oneliner":{"type":"boolean","default":false},
                    "replace":{"type":"boolean","default":false}
                },
                "required":["import_name"],
                "oneOf":[{"required":["content"]},{"required":["paths"]}],
                "additionalProperties":false
            }),
            false,
        ),
        tool(
            "grimoire_init",
            "Initialize the project with the existing Grimoire CSS init API",
            object_schema(json!({}), &[]),
            false,
        ),
        tool(
            "grimoire_build",
            "Build CSS with the existing Grimoire CSS filesystem build API",
            object_schema(
                json!({"force_version_update":{"type":"boolean","default":false}}),
                &[],
            ),
            false,
        ),
        tool(
            "grimoire_shorten",
            "Rewrite configured project files with the existing Grimoire CSS shorten API",
            object_schema(json!({}), &[]),
            false,
        ),
    ]
}

fn tool(name: &str, description: &str, input_schema: Value, read_only: bool) -> Value {
    json!({
        "name":name,
        "description":description,
        "inputSchema":input_schema,
        "outputSchema":{
            "type":"object",
            "properties":{"data":{},"error":{"type":"string"}},
            "oneOf":[{"required":["data"]},{"required":["error"]}],
            "additionalProperties":false
        },
        "annotations":{
            "readOnlyHint":read_only,
            "destructiveHint":!read_only,
            "idempotentHint":read_only,
            "openWorldHint":false
        }
    })
}

fn object_schema(properties: Value, required: &[&str]) -> Value {
    json!({
        "type":"object",
        "properties":properties,
        "required":required,
        "additionalProperties":false
    })
}

fn string_arg<'a>(value: &'a Value, name: &str) -> Result<&'a str, String> {
    value
        .get(name)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("'{name}' must be a non-empty string"))
}

fn optional_string_arg<'a>(value: &'a Value, name: &str) -> Result<Option<&'a str>, String> {
    match value.get(name) {
        None => Ok(None),
        Some(value) => value
            .as_str()
            .filter(|value| !value.is_empty())
            .map(Some)
            .ok_or_else(|| format!("'{name}' must be a non-empty string")),
    }
}

fn string_array_arg(value: &Value, name: &str) -> Result<Vec<String>, String> {
    let values = value
        .get(name)
        .and_then(Value::as_array)
        .filter(|values| !values.is_empty() && values.len() <= 256)
        .ok_or_else(|| format!("'{name}' must contain between 1 and 256 strings"))?;
    if values
        .iter()
        .any(|value| value.as_str().is_none_or(str::is_empty))
    {
        return Err(format!("'{name}' must contain only non-empty strings"));
    }
    Ok(values
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect())
}

fn optional_string_array_arg(value: &Value, name: &str) -> Result<Option<Vec<String>>, String> {
    if value.get(name).is_none() {
        return Ok(None);
    }
    string_array_arg(value, name).map(Some)
}

fn usize_arg(value: &Value, name: &str, default: usize) -> Result<usize, String> {
    match value.get(name) {
        None => Ok(default),
        Some(value) => value
            .as_u64()
            .and_then(|value| usize::try_from(value).ok())
            .filter(|value| *value > 0)
            .ok_or_else(|| format!("'{name}' must be a positive integer")),
    }
}

fn bool_arg(value: &Value, name: &str, default: bool) -> Result<bool, String> {
    match value.get(name) {
        None => Ok(default),
        Some(value) => value
            .as_bool()
            .ok_or_else(|| format!("'{name}' must be a boolean")),
    }
}

fn exact_keys(value: &Value, allowed: &[&str]) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| "tool arguments must be an object".to_string())?;
    if let Some(name) = object.keys().find(|name| !allowed.contains(&name.as_str())) {
        return Err(format!("unknown tool argument '{name}'"));
    }
    Ok(())
}

fn domain_result<T: Serialize>(
    result: Result<T, crate::GrimoireCssError>,
) -> Result<Value, String> {
    Ok(match result {
        Ok(value) => {
            let structured = serde_json::to_value(value).map_err(|error| error.to_string())?;
            let text =
                serde_json::to_string_pretty(&structured).map_err(|error| error.to_string())?;
            json!({
                "content":[{"type":"text","text":text}],
                "structuredContent":{"data":structured},
                "isError":false
            })
        }
        Err(error) => {
            let message = error.to_string();
            json!({
                "content":[{"type":"text","text":message}],
                "structuredContent":{"error":message},
                "isError":true
            })
        }
    })
}

fn error_response(id: Value, code: i64, message: &str) -> Value {
    json!({"jsonrpc":"2.0","id":id,"error":{"code":code,"message":message}})
}
