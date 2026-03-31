use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SnippetMeta {
    pub tags: Vec<String>,
    pub ext: Option<String>,
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

        fs::create_dir_all(&base_path).expect("could not create snippets directory");

        Self { base_path }
    }

    pub fn exists(&self, name: &str) -> bool {
        self.base_path.join(format!("{}.code", name)).exists()
    }

    pub fn save(&self, name: &str, content: &str, meta: SnippetMeta) -> Result<()> {
        fs::write(self.base_path.join(format!("{}.code", name)), content)
            .with_context(|| format!("failed to write content for '{}'", name))?;

        let meta_json = serde_json::to_string_pretty(&meta)
            .with_context(|| format!("failed to serialize metadata for '{}'", name))?;

        fs::write(
            self.base_path.join(format!("{}.meta.json", name)),
            meta_json,
        )
        .with_context(|| format!("failed to write metadata for '{}'", name))?;

        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<Snippet> {
        if !self.exists(name) {
            return Err(anyhow!("snippet '{}' not found", name));
        }

        let content = fs::read_to_string(self.base_path.join(format!("{}.code", name)))
            .with_context(|| format!("failed to read content for '{}'", name))?;

        let meta_str = fs::read_to_string(self.base_path.join(format!("{}.meta.json", name)))
            .with_context(|| format!("failed to read metadata for '{}'", name))?;

        let meta: SnippetMeta = serde_json::from_str(&meta_str)
            .with_context(|| format!("failed to parse metadata for '{}'", name))?;

        Ok(Snippet {
            name: name.to_string(),
            content,
            meta,
        })
    }

    pub fn remove(&self, name: &str) -> Result<()> {
        if !self.exists(name) {
            return Err(anyhow!("snippet '{}' not found", name));
        }

        fs::remove_file(self.base_path.join(format!("{}.code", name)))
            .with_context(|| format!("failed to remove content for '{}'", name))?;

        fs::remove_file(self.base_path.join(format!("{}.meta.json", name)))
            .with_context(|| format!("failed to remove metadata for '{}'", name))?;

        Ok(())
    }

    pub fn list(&self, tag_filter: Option<&Vec<String>>) -> Result<Vec<Snippet>> {
        let mut snippets = vec![];

        for entry in fs::read_dir(&self.base_path).context("failed to read snippets directory")? {
            let entry = entry.context("failed to read directory entry")?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("code") {
                continue;
            }

            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            let snippet = self.get(&name)?;

            if let Some(filter) = tag_filter
                && !filter.iter().any(|t| snippet.meta.tags.contains(t))
            {
                continue;
            }

            snippets.push(snippet);
        }

        Ok(snippets)
    }
}
