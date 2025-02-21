use std::collections::HashMap;

/// Configuration for processing CSS in memory
#[derive(Debug, Clone)]
pub struct ConfigInMemory {
    /// Projects to process
    pub projects: Vec<ConfigInMemoryEntry>,
    /// Optional CSS variables for value substitution
    pub variables: Option<Vec<(String, String)>>,
    /// Optional scroll definitions
    pub scrolls: Option<HashMap<String, Vec<String>>>,
    /// Map of custom animation names to their CSS definitions
    pub custom_animations: HashMap<String, String>,
    /// Optional browserslist configuration content
    pub browserslist_content: Option<String>,
}

/// Project entry for in-memory CSS processing
#[derive(Debug, Clone)]
pub struct ConfigInMemoryEntry {
    /// Name identifier for the project
    pub name: String,
    /// Content to process for CSS classes
    pub content: Vec<String>,
}
