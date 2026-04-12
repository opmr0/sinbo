use std::{
    env, fs,
    io::{self, IsTerminal, Read},
    path::PathBuf,
    process::Command,
};

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use colored::Colorize;
use dialoguer::Confirm;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

mod encryption;
mod storage;
mod transfer;
mod var;

use crate::storage::SnippetMeta;
use storage::Storage;

#[derive(Parser)]
#[clap(version, name = "sinbo", about = "A CLI snippet manager")]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[command(about = "Print or copy a snippet", alias = "g")]
    Get {
        name: String,
        #[arg(short, long, help = "Copy to clipboard instead of printing")]
        copy: bool,
        #[arg(long, value_parser = parse_key_val, help = "Substitute placeholders (key=value)")]
        args: Vec<(String, String)>,
    },
    #[command(about = "Add a new snippet", alias = "a")]
    Add {
        name: String,
        #[arg(long, short, num_args = 1, help = "Read content from a file")]
        file_name: Option<String>,
        #[arg(short, long, num_args = 1.., help = "Assign tags to the snippet")]
        tags: Option<Vec<String>>,
        #[arg(
            short,
            long,
            num_args = 1,
            help = "File extension for editor syntax highlighting"
        )]
        ext: Option<String>,
        #[arg(long, help = "Encrypt the snippet with a password")]
        encrypt: bool,
        #[arg(long, short, num_args = 1, help = "Short description for the snippet")]
        description: Option<String>,
    },
    #[command(about = "List all snippets", alias = "l")]
    List {
        #[arg(short, long, num_args = 1.., help = "Filter snippets by tag")]
        tags: Option<Vec<String>>,
        #[arg(short, long, help = "Show snippet content")]
        show: bool,
    },
    #[command(about = "Remove a snippet", alias = "r")]
    Remove { name: String },
    #[command(about = "Edit an existing snippet", alias = "e")]
    Edit {
        name: String,
        #[arg(short, long, num_args = 1.., help = "Replace the snippet's tags")]
        tags: Option<Vec<String>>,
        #[arg(long, short, num_args = 1, help = "Update the snippet's description")]
        description: Option<String>,
    },
    #[command(about = "Search snippets by name or content", alias = "s")]
    Search {
        #[arg(short, long, num_args = 1.., help = "Narrow search to specific tags")]
        tags: Option<Vec<String>>,
        query: String,
    },
    #[command(about = "Encrypt an existing snippet")]
    Encrypt { name: String },
    #[command(about = "Decrypt a snippet permanently")]
    Decrypt { name: String },
    #[command(about = "Export a snippet to a .sinbo.json file")]
    Export {
        name: String,
        #[arg(
            short,
            long,
            num_args = 1,
            help = "Directory to export into (default: current dir)"
        )]
        path: Option<PathBuf>,
    },
    #[command(about = "Import a snippet from a .sinbo.json file")]
    Import { path: std::path::PathBuf },
}

fn confirm() {
    let confirmation = Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap();
    if !confirmation {
        std::process::exit(0)
    }
}

fn open_editor(
    initial_content: Option<&str>,
    ext: Option<&str>,
    sensitive: bool,
) -> Result<String> {
    let editor = env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(windows) {
            "notepad".to_string()
        } else {
            "nano".to_string()
        }
    });

    let file_name = match ext {
        Some(e) => format!("sinbo_snippet.{}", e),
        None => "sinbo_snippet.tmp".to_string(),
    };

    let tmp = env::temp_dir().join(file_name);

    if let Some(content) = initial_content {
        fs::write(&tmp, content)?;
    }

    #[cfg(windows)]
    Command::new("cmd")
        .arg("/c")
        .args([&editor, tmp.to_str().unwrap()])
        .status()
        .context("failed to open editor")?;

    #[cfg(not(windows))]
    Command::new(&editor)
        .arg(&tmp)
        .status()
        .context("failed to open editor")?;

    let content = fs::read_to_string(&tmp).context("failed to read temp file")?;

    if sensitive {
        encryption::secure_delete(&tmp).ok();
    } else {
        fs::remove_file(&tmp).ok();
    }

    Ok(content)
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let storage = Storage::new();

    match args.action {
        Action::Get { name, copy, args } => {
            let snippet = storage.get(&name)?;

            let mut content = if snippet.encrypted {
                let password = encryption::prompt_password("Password: ")?;
                let enc_path = storage.snippet_path(&name).with_extension("enc");
                let bytes = encryption::read_encrypted(&enc_path, password.as_bytes())
                    .map_err(|e| anyhow!("{}", e))?;
                String::from_utf8(bytes).context("decrypted content is not valid utf-8")?
            } else {
                snippet.content
            };

            if !args.is_empty() {
                let map: std::collections::HashMap<String, String> = args.into_iter().collect();
                content = var::substitute(&content, &map)?;
            }

            if copy {
                let mut clipboard = arboard::Clipboard::new()?;
                clipboard.set_text(&content)?;
                eprintln!(
                    "{} copied '{}' to clipboard",
                    "sinbo".cyan().bold(),
                    name.yellow()
                );
            } else {
                print!("{}", content);
            }
        }
        Action::Add {
            name,
            file_name,
            tags,
            ext,
            encrypt,
            description,
        } => {
            if storage.exists(&name) {
                return Err(anyhow!("snippet '{}' already exists", name));
            }

            let content = if let Some(path) = file_name {
                fs::read_to_string(&path).with_context(|| format!("failed to read '{}'", path))?
            } else if std::io::stdin().is_terminal() {
                open_editor(None, ext.as_deref(), encrypt)?
            } else {
                let mut buf = String::new();
                io::stdin()
                    .read_to_string(&mut buf)
                    .context("failed to read stdin")?;
                buf
            };

            if content.trim().is_empty() {
                return Err(anyhow!("snippet content is empty"));
            }

            let meta = SnippetMeta {
                description,
                modified_at: now_secs(),
                tags: tags.unwrap_or_default(),
                ext,
            };

            if encrypt && (!meta.tags.is_empty() || meta.description.is_some()) {
                eprintln!(
                    "{} tags and description are stored in plaintext, avoid sensitive names",
                    "Warning:".yellow().bold()
                );
            }

            if encrypt {
                let password = encryption::prompt_password_confirmed()?;
                let enc_path = storage.snippet_path(&name).with_extension("enc");
                encryption::write_encrypted(&enc_path, content.as_bytes(), password.as_bytes())
                    .map_err(|e| anyhow!("{}", e))?;
                storage.save_meta(&name, &meta)?;
                eprintln!(
                    "{} saved '{}' {}",
                    "sinbo".cyan().bold(),
                    name.yellow(),
                    "(encrypted)".dimmed()
                );
                return Ok(());
            }

            storage.save(&name, &content, meta)?;
            eprintln!("{} saved '{}'", "sinbo".cyan().bold(), name.yellow());
        }
        Action::List { tags, show } => {
            let snippets = storage.list(tags.as_ref())?;

            if snippets.is_empty() {
                eprintln!("{} no snippets found", "sinbo".cyan().bold());
                return Ok(());
            }

            eprintln!("{} {} snippets\n", "sinbo".cyan().bold(), snippets.len());
            for s in &snippets {
                let ext_raw = s
                    .meta
                    .ext
                    .as_deref()
                    .map(|e| format!(" .{e}"))
                    .unwrap_or_default();

                let ext_str = s
                    .meta
                    .ext
                    .as_deref()
                    .map(|e| format!(" .{}", e.bright_black()))
                    .unwrap_or_default();

                let desc_str = s.meta.description.clone().unwrap_or_default();

                let tags_raw = if s.meta.tags.is_empty() {
                    String::new()
                } else {
                    format!("[{}]", s.meta.tags.join(", "))
                };

                let tags_str = if s.meta.tags.is_empty() {
                    String::new()
                } else {
                    format!("[{}]", s.meta.tags.join(", ").dimmed())
                };

                let name_ext_raw = format!("{}{}", s.name, ext_raw);
                let pad = 20usize.saturating_sub(name_ext_raw.len());

                let tags_pad = 30usize.saturating_sub(tags_raw.len());

                let end = if s.encrypted {
                    format!(
                        "{} --- {}",
                        "Locked".yellow().dimmed(),
                        desc_str.bright_black().italic()
                    )
                } else {
                    desc_str.bright_black().italic().to_string()
                };

                println!(
                    "{}{}{}{}{}{}",
                    s.name.cyan().bold(),
                    ext_str,
                    " ".repeat(pad),
                    tags_str,
                    " ".repeat(tags_pad),
                    end
                );

                if show {
                    if s.encrypted {
                        println!("> {}", "[encrypted]".dimmed());
                    } else {
                        println!("> {}", s.content.dimmed());
                    }
                }
            }
        }
        Action::Remove { name } => {
            eprintln!("This command will remove '{}'", name);
            confirm();
            storage.remove(&name)?;
            eprintln!("{} removed '{}'", "sinbo".cyan().bold(), name.yellow());
        }
        Action::Edit {
            name,
            tags,
            description,
        } => {
            let snippet = storage
                .get(&name)
                .with_context(|| format!("snippet '{}' not found", name))?;

            if snippet.encrypted {
                return Err(anyhow!(
                    "cannot edit encrypted snippet '{}', remove and re-add it",
                    name
                ));
            }

            let content = if std::io::stdin().is_terminal() {
                open_editor(Some(&snippet.content), snippet.meta.ext.as_deref(), false)?
            } else {
                let mut buf = String::new();
                io::stdin()
                    .read_to_string(&mut buf)
                    .context("failed to read stdin")?;
                buf
            };

            if content.trim().is_empty() {
                return Err(anyhow!("snippet content is empty"));
            }

            let meta = SnippetMeta {
                description,
                modified_at: now_secs(),
                tags: tags.unwrap_or(snippet.meta.tags),
                ext: snippet.meta.ext,
            };

            storage.save(&name, &content, meta)?;
            eprintln!("{} updated '{}'", "sinbo".cyan().bold(), name.yellow());
        }
        Action::Search { query, tags } => {
            let all_snippets = storage.list(tags.as_ref())?;
            let matcher = SkimMatcherV2::default().ignore_case();
            let query_lower = query.to_lowercase();

            let mut results: Vec<(i64, &_)> = all_snippets
                .iter()
                .filter_map(|s| {
                    let name_score = matcher.fuzzy_match(&s.name, &query);
                    let content_match = !s.encrypted
                        && s.content
                            .lines()
                            .any(|l| l.to_lowercase().contains(&query_lower));
                    match (name_score, content_match) {
                        (Some(score), _) => Some((score, s)),
                        (None, true) => Some((0, s)),
                        _ => None,
                    }
                })
                .collect();

            results.sort_by(|a, b| b.0.cmp(&a.0));

            if results.is_empty() {
                eprintln!("{} no matches found", "sinbo".cyan().bold());
            } else {
                eprintln!("{} {} results\n", "sinbo".cyan().bold(), results.len());
                for s in results {
                    let tags_str = if s.1.meta.tags.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", s.1.meta.tags.join(", ").dimmed())
                    };
                    let ext_str =
                        s.1.meta
                            .ext
                            .as_deref()
                            .map(|e| format!(" .{}", e.bright_black()))
                            .unwrap_or_default();

                    println!("{}{}{}", s.1.name.cyan().bold(), tags_str, ext_str);

                    if s.1.encrypted {
                        println!("  {} {}", ">".yellow().bold(), "[encrypted]".dimmed());
                    } else {
                        for line in s.1.content.lines() {
                            if line.to_lowercase().contains(&query_lower) {
                                println!("  {} {}", ">".yellow().bold(), line.dimmed());
                            }
                        }
                    }
                    println!();
                }
            }
        }
        Action::Encrypt { name } => {
            let snippet = storage
                .get(&name)
                .with_context(|| format!("snippet '{}' not found", name))?;

            eprintln!("This command will encrypt '{}'", name);
            confirm();

            let enc_path = storage.snippet_path(&name).with_extension("enc");

            if enc_path.exists() {
                return Err(anyhow!("snippet '{}' is already encrypted", name));
            }

            let password = encryption::prompt_password_confirmed()?;
            encryption::write_encrypted(&enc_path, snippet.content.as_bytes(), password.as_bytes())
                .map_err(|e| anyhow!("{}", e))?;

            fs::remove_file(storage.snippet_path(&name).with_extension("code"))
                .with_context(|| format!("failed to remove plaintext for '{}'", name))?;

            eprintln!("{} encrypted '{}'", "sinbo".cyan().bold(), name.yellow());
        }
        Action::Decrypt { name } => {
            let enc_path = storage.snippet_path(&name).with_extension("enc");

            if !enc_path.exists() {
                return Err(anyhow!("snippet '{}' is not encrypted", name));
            }

            eprintln!("This command will decrypt '{}'", name);
            confirm();

            let password = encryption::prompt_password("Password: ")?;
            let bytes = encryption::read_encrypted(&enc_path, password.as_bytes())
                .map_err(|e| anyhow!("{}", e))?;
            let content =
                String::from_utf8(bytes).context("decrypted content is not valid utf-8")?;

            let meta = storage.get(&name)?.meta;
            storage
                .save(&name, &content, meta)
                .with_context(|| format!("failed to save plaintext for '{}'", name))?;
            encryption::secure_delete(&enc_path).map_err(|e| anyhow!("{}", e))?;

            eprintln!("{} decrypted '{}'", "sinbo".cyan().bold(), name.yellow());
        }
        Action::Export { name, path } => {
            let snippet = storage.get(&name)?;
            transfer::export(&snippet, path)?;
        }
        Action::Import { path } => {
            transfer::import(path, storage)?;
        }
    }

    Ok(())
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    s.split_once('=')
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .ok_or_else(|| format!("invalid key=value pair: '{}'", s))
}
