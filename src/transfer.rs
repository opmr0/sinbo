use anyhow::{Context, Result, anyhow};
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::{
    now_secs,
    storage::{Snippet, SnippetMeta},
};

#[derive(Serialize, Deserialize)]
struct ExportedSnippet {
    name: String,
    description: Option<String>,
    content: String,
    tags: Option<Vec<String>>,
    extension: Option<String>,
}

pub fn export(snippet: &Snippet, path_to: Option<PathBuf>) -> Result<()> {
    if snippet.encrypted {
        return Err(anyhow!(
            "cannot export an encrypted snippet, decrypt it first"
        ));
    }

    let mut exported = ExportedSnippet {
        name: snippet.name.clone(),
        description: snippet.meta.description.clone(),
        tags: Some(snippet.meta.tags.clone()),
        content: snippet.content.clone(),
        extension: snippet.meta.ext.clone(),
    };

    let base = path_to.clone().unwrap_or_default();
    let mut f_path = base.join(format!("{}.sinbo.json", exported.name));

    if f_path.exists()
        && let Some(new_name) =
            prompt_options(&format!("{}.sinbo.json", exported.name), None, Some(&base))
    {
        exported.name = new_name;
        f_path = base.join(format!("{}.sinbo.json", exported.name));
    }

    let json = serde_json::to_string_pretty(&exported)
        .with_context(|| format!("failed to serialize '{}'", exported.name))?;

    fs::write(&f_path, json).with_context(|| format!("failed to write '{}'", exported.name))?;

    Ok(())
}

pub fn import(path: PathBuf, storage: crate::Storage) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("file not found: {}", path.display()));
    }

    if !path.to_string_lossy().ends_with(".sinbo.json") {
        return Err(anyhow!("'{}' is not a .sinbo.json file", path.display()));
    }

    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read '{}'", path.display()))?;

    let mut exported: ExportedSnippet = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse '{}'", path.display()))?;

    if storage.exists(&exported.name)
        && let Some(new_name) = prompt_options(&exported.name, Some(&storage), None)
    {
        exported.name = new_name;
    }

    let meta = SnippetMeta {
        description: exported.description,
        ext: exported.extension,
        tags: exported.tags.unwrap_or_default(),
        modified_at: now_secs(),
    };

    storage.save(&exported.name, &exported.content, meta)?;

    Ok(())
}

fn prompt_options(
    name: &str,
    storage: Option<&crate::Storage>,
    base: Option<&PathBuf>,
) -> Option<String> {
    let selection = Select::new()
        .with_prompt(format!("'{}' already exists", name))
        .items(["Overwrite", "Rename", "Cancel"])
        .interact()
        .unwrap();

    match selection {
        0 => None,
        1 => loop {
            let new_name: String = Input::new()
                .with_prompt("New name")
                .interact_text()
                .unwrap();

            if new_name == name {
                eprintln!("error: new name must differ from the current name");
                continue;
            }

            if let Some(s) = storage
                && s.exists(&new_name)
            {
                eprintln!("error: '{}' already exists, choose another name", new_name);
                continue;
            }

            if let Some(b) = base
                && b.join(format!("{}.sinbo.json", new_name)).exists()
            {
                eprintln!(
                    "error: '{}.sinbo.json' already exists, choose another name",
                    new_name
                );
                continue;
            }

            return Some(new_name);
        },
        2 => std::process::exit(0),
        _ => unreachable!(),
    }
}
