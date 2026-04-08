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
    pub encrypted: bool,
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

    pub fn snippet_path(&self, name: &str) -> PathBuf {
        self.base_path.join(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.base_path.join(format!("{}.code", name)).exists()
            || self.base_path.join(format!("{}.enc", name)).exists()
    }

    pub fn is_encrypted(&self, name: &str) -> bool {
        self.base_path.join(format!("{}.enc", name)).exists()
    }

    pub fn save(&self, name: &str, content: &str, meta: SnippetMeta) -> Result<()> {
        fs::write(self.base_path.join(format!("{}.code", name)), content)
            .with_context(|| format!("failed to write content for '{}'", name))?;

        self.save_meta(name, &meta)?;

        Ok(())
    }

    pub fn save_meta(&self, name: &str, meta: &SnippetMeta) -> Result<()> {
        let meta_json = serde_json::to_string_pretty(meta)
            .with_context(|| format!("failed to serialize metadata for '{}'", name))?;

        fs::write(
            self.base_path.join(format!("{}.meta.json", name)),
            meta_json,
        )
        .with_context(|| format!("failed to write metadata for '{}'", name))?;

        Ok(())
    }

    fn read_meta(&self, name: &str) -> Result<SnippetMeta> {
        let meta_str = fs::read_to_string(self.base_path.join(format!("{}.meta.json", name)))
            .with_context(|| format!("failed to read metadata for '{}'", name))?;

        serde_json::from_str(&meta_str)
            .with_context(|| format!("failed to parse metadata for '{}'", name))
    }

    pub fn get(&self, name: &str) -> Result<Snippet> {
        if !self.exists(name) {
            return Err(anyhow!("snippet '{}' not found", name));
        }

        if self.is_encrypted(name) {
            let meta = self.read_meta(name)?;
            return Ok(Snippet {
                name: name.to_string(),
                content: String::new(),
                meta,
                encrypted: true,
            });
        }

        let content = fs::read_to_string(self.base_path.join(format!("{}.code", name)))
            .with_context(|| format!("failed to read content for '{}'", name))?;

        let meta = self.read_meta(name)?;

        Ok(Snippet {
            name: name.to_string(),
            content,
            meta,
            encrypted: false,
        })
    }

    pub fn remove(&self, name: &str) -> Result<()> {
        if !self.exists(name) {
            return Err(anyhow!("snippet '{}' not found", name));
        }

        let code_path = self.base_path.join(format!("{}.code", name));
        let enc_path = self.base_path.join(format!("{}.enc", name));

        if code_path.exists() {
            fs::remove_file(&code_path)
                .with_context(|| format!("failed to remove content for '{}'", name))?;
        }
        if enc_path.exists() {
            fs::remove_file(&enc_path)
                .with_context(|| format!("failed to remove encrypted content for '{}'", name))?;
        }

        fs::remove_file(self.base_path.join(format!("{}.meta.json", name)))
            .with_context(|| format!("failed to remove metadata for '{}'", name))?;

        Ok(())
    }

    pub fn list(&self, tag_filter: Option<&Vec<String>>) -> Result<Vec<Snippet>> {
        let mut snippets = vec![];
        let mut seen = std::collections::HashSet::new();

        for entry in fs::read_dir(&self.base_path).context("failed to read snippets directory")? {
            let entry = entry.context("failed to read directory entry")?;
            let path = entry.path();

            let ext = path.extension().and_then(|e| e.to_str());

            let name = match ext {
                Some("code") => path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string(),
                Some("enc") => path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string(),
                _ => continue,
            };

            if name.is_empty() || seen.contains(&name) {
                continue;
            }
            seen.insert(name.clone());

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
