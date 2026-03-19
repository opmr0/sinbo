use std::{fs};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use anyhow::{Context, Result, anyhow};

#[derive(Serialize, Deserialize)]
pub struct SnippetMeta {
    pub tags: Vec<String>,    
    pub modified_at: u64,
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

    pub fn save(&self, name: &str, content: &str, meta: SnippetMeta) -> Result<()> {

        fs::write(
            self.base_path.join(format!("{}.code", name)),
            content
        ).context("{e}")?;

        let meta_json = serde_json::to_string_pretty(&meta)
            .context("{e}")?;

        fs::write(
            self.base_path.join(format!("{}.meta.json", name)),
            meta_json
        ).context("{e}")?;

        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<Snippet> {
        if !self.exists(name) {
            return Err(anyhow!("snippet '{}' not found", name));
        }

        let content = fs::read_to_string(
            self.base_path.join(format!("{}.code", name))
        ).context("{e}")?;

        let meta_str = fs::read_to_string(
            self.base_path.join(format!("{}.meta.json", name))
        ).context("{e}")?;

        let meta: SnippetMeta = serde_json::from_str(&meta_str)
            .context("{e}")?;

        Ok(Snippet { name: name.to_string(), content, meta })
    }

    pub fn remove(&self, name: &str) -> Result<()> {
        if !self.exists(name) {
            return Err(anyhow!("snippet '{}' not found", name));
        }

        fs::remove_file(self.base_path.join(format!("{}.code", name)))
            .context("{e}")?;

        fs::remove_file(self.base_path.join(format!("{}.meta.json", name)))
            .context("{e}")?;

        Ok(())
    }

    pub fn list(&self, tag_filter: Option<&Vec<String>>) -> Result<Vec<Snippet>> {
        let mut snippets = vec![];

        for entry in fs::read_dir(&self.base_path).context("{e}")? {
            let entry = entry.context("{e}")?;
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