use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub path: Option<PathBuf>,
    pub content: Arc<String>,
}

impl SourceFile {
    pub fn new(path: Option<PathBuf>, name: String, content: String) -> Self {
        Self {
            name,
            path,
            content: Arc::new(content),
        }
    }
}
