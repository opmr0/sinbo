use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SnippetMeta {
    pub tags: Vec<String>,
    pub lang: Option<String>,
    pub created_at: String,
}

pub struct Snippet {
    pub name: String,
    pub content: String,
    pub meta: SnippetMeta,
}

pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let base_path = dirs::config_dir()
            .expect("could not find config dir")
            .join("sinbo")
            .join("snippets");

        fs::create_dir_all(&base_path)
            .expect("could not create snippets directory");

        Self { base_path }
    }

    pub fn exists(&self, name: &str) -> bool {
        self.base_path.join(format!("{}.code", name)).exists()
    }

    pub fn save(&self, name: &str, content: &str, meta: SnippetMeta) -> Result<(), String> {
        if self.exists(name) {
            return Err(format!("snippet '{}' already exists", name));
        }

        fs::write(
            self.base_path.join(format!("{}.code", name)),
            content
        ).map_err(|e| e.to_string())?;

        let meta_json = serde_json::to_string_pretty(&meta)
            .map_err(|e| e.to_string())?;

        fs::write(
            self.base_path.join(format!("{}.meta.json", name)),
            meta_json
        ).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<Snippet, String> {
        if !self.exists(name) {
            return Err(format!("snippet '{}' not found", name));
        }

        let content = fs::read_to_string(
            self.base_path.join(format!("{}.code", name))
        ).map_err(|e| e.to_string())?;

        let meta_str = fs::read_to_string(
            self.base_path.join(format!("{}.meta.json", name))
        ).map_err(|e| e.to_string())?;

        let meta: SnippetMeta = serde_json::from_str(&meta_str)
            .map_err(|e| e.to_string())?;

        Ok(Snippet { name: name.to_string(), content, meta })
    }

    pub fn remove(&self, name: &str) -> Result<(), String> {
        if !self.exists(name) {
            return Err(format!("snippet '{}' not found", name));
        }

        fs::remove_file(self.base_path.join(format!("{}.code", name)))
            .map_err(|e| e.to_string())?;

        fs::remove_file(self.base_path.join(format!("{}.meta.json", name)))
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn list(&self, tag_filter: Option<&Vec<String>>) -> Result<Vec<Snippet>, String> {
        let mut snippets = vec![];

        for entry in fs::read_dir(&self.base_path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("code") {
                continue;
            }

            let name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            let snippet = self.get(&name)?;

            if let Some(filter) = tag_filter {
                if !filter.iter().any(|t| snippet.meta.tags.contains(t)) {
                    continue;
                }
            }

            snippets.push(snippet);
        }

        Ok(snippets)
    }
}