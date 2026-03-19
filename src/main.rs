use std::{
    env, fs,
    io::{self, Read},
    process::Command,
};

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};

mod storage;
use atty;

use storage::Storage;

use crate::storage::SnippetMeta;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Get {
        name: String,
        #[arg(short, long)]
        copy: bool,
    },
    Add {
        name: String,
        #[arg(long, short, num_args = 1)]
        file_name: Option<String>,
        #[arg(short, long, num_args = 1..)]
        tags: Option<Vec<String>>,
    },
    List {
        #[arg(short, long, num_args = 1..)]
        tags: Option<Vec<String>>,
    },
    Remove {
        name: String,
    },
    Edit {
        name: String,
        #[arg(short, long, num_args = 1..)]
        tags: Option<Vec<String>>,
    },
}

#[allow(unused)]
fn main() -> Result<()> {
    let args = Cli::parse();
    let storage = Storage::new();

    match args.action {
        Action::Get { name, copy } => {}
        Action::Add {
            name,
            file_name,
            tags,
        } => {
            let mut content = String::new();

            if storage.exists(&name) {
                return Err(anyhow!("snippet '{}' already exists", name));
            }

            if let Some(file_name) = file_name {
                content = fs::read_to_string(file_name)?;
            } else if atty::is(atty::Stream::Stdin) {
                let editor = env::var("EDITOR").unwrap_or("nano".to_string());
                let tmp = env::temp_dir().join("sinbo_snippet.tmp");

                #[cfg(windows)]
                Command::new("cmd")
                    .arg("/c")
                    .args([&editor, tmp.to_str().unwrap()])
                    .status()
                    .context("failed to open editor")?;

                #[cfg(not(windows))]
                Command::new(editor)
                    .arg(&tmp)
                    .status()
                    .context("failed to open editor")?;

                content = fs::read_to_string(&tmp).context("failed to read temp file")?;
                fs::remove_file(&tmp).ok();
            } else {
                io::stdin()
                    .read_to_string(&mut content)
                    .context("failed to read stdin")?;
            }

            let meta = SnippetMeta {
                modified_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                tags: tags.unwrap_or_default(),
            };

            storage.save(&name, &content, meta)?;
        }
        Action::List { tags } => {
            let snippets = storage.list(tags.as_ref())?;
            for s in snippets {
                println!("{} --- {:?}", s.name, s.meta.tags);
                println!("{}", s.content);
            }
        }
        Action::Remove { name } => {
            storage.remove(&name)?;
            println!("removed '{}'", name);
        }
        Action::Edit { name, tags } => {
            let mut content = String::new();

            let snippet = storage.get(&name).context("snippet '{name}' not found")?;

            if atty::is(atty::Stream::Stdin) {
                let editor = env::var("EDITOR").unwrap_or("nano".to_string());
                let tmp = env::temp_dir().join("sinbo_snippet.tmp");
                fs::write(&tmp, snippet.content)?;

                #[cfg(windows)]
                Command::new("cmd")
                    .arg("/c")
                    .args([&editor, tmp.to_str().unwrap()])
                    .status()
                    .context("failed to open editor")?;

                #[cfg(not(windows))]
                Command::new(editor)
                    .arg(&tmp)
                    .status()
                    .context("failed to open editor")?;

                content = fs::read_to_string(&tmp).context("failed to read temp file")?;
                fs::remove_file(&tmp).ok();
            } else {
                io::stdin()
                    .read_to_string(&mut content)
                    .context("failed to read stdin")?;
            }

            let meta = SnippetMeta {
                modified_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                tags: tags.unwrap_or_default(),
            };

            storage.save(&name, &content, meta)?;
        }
    }
    Ok(())
}
