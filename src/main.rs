use std::{
    env, fs,
    io::{self, Read},
    process::Command,
};

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use colored::Colorize;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

mod storage;

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
    },
    #[command(about = "Add a new snippet", alias = "a")]
    Add {
        name: String,
        #[arg(long, short, num_args = 1, help = "Read content from a file")]
        file_name: Option<String>,
        #[arg(short, long, num_args = 1.., help = "Tags for the snippet")]
        tags: Option<Vec<String>>,
        #[arg(
            short,
            long,
            num_args = 1,
            help = "File extension for syntax highlighting in editor"
        )]
        ext: Option<String>,
    },
    #[command(about = "List all snippets", alias = "l")]
    List {
        #[arg(short, long, num_args = 1.., help = "Filter by tags")]
        tags: Option<Vec<String>>,
        #[arg(short, long, help = "Show the snippets content")]
        show: bool,
    },
    #[command(about = "Remove a snippet", alias = "r")]
    Remove { name: String },
    #[command(about = "Edit an existing snippet", alias = "e")]
    Edit {
        name: String,
        #[arg(short, long, num_args = 1.., help = "Update tags")]
        tags: Option<Vec<String>>,
    },
    #[command(about = "Search a query in snippets", alias = "s")]
    Search {
        #[arg(short, long, num_args = 1.., help = "search in tags")]
        tags: Option<Vec<String>>,
        query: String,
    },
}

fn open_editor(initial_content: Option<&str>, ext: Option<&str>) -> Result<String> {
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
    fs::remove_file(&tmp).ok();

    Ok(content)
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let storage = Storage::new();

    match args.action {
        Action::Get { name, copy } => {
            let snippet = storage.get(&name)?;
            if copy {
                let mut clipboard = arboard::Clipboard::new()?;
                clipboard.set_text(&snippet.content)?;
                eprintln!(
                    "{} copied '{}' to clipboard",
                    "sinbo".cyan().bold(),
                    name.yellow()
                );
            } else {
                print!("{}", snippet.content);
            }
        }
        Action::Add {
            name,
            file_name,
            tags,
            ext,
        } => {
            if storage.exists(&name) {
                return Err(anyhow!("snippet '{}' already exists", name));
            }

            let content = if let Some(path) = file_name {
                fs::read_to_string(&path).with_context(|| format!("failed to read '{}'", path))?
            } else if atty::is(atty::Stream::Stdin) {
                open_editor(None, ext.as_deref())?
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
                modified_at: now_secs(),
                tags: tags.unwrap_or_default(),
                ext,
            };

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
                let tags_str = if s.meta.tags.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", s.meta.tags.join(", ").dimmed())
                };

                let ext_str = s
                    .meta
                    .ext
                    .as_deref()
                    .map(|e| format!(" .{}", e.bright_black()))
                    .unwrap_or_default();

                println!("{}{}{}", s.name.cyan().bold(), tags_str, ext_str);
                if show {
                    println!("> {}", s.content.dimmed())
                }
            }
        }
        Action::Remove { name } => {
            storage.remove(&name)?;
            eprintln!("{} removed '{}'", "sinbo".cyan().bold(), name.yellow());
        }
        Action::Edit { name, tags } => {
            let snippet = storage
                .get(&name)
                .with_context(|| format!("snippet '{}' not found", name))?;

            let content = if atty::is(atty::Stream::Stdin) {
                open_editor(Some(&snippet.content), snippet.meta.ext.as_deref())?
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
                    let content_match = s
                        .content
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
                    let ext_str = s
                        .1.meta
                        .ext
                        .as_deref()
                        .map(|e| format!(" .{}", e.bright_black()))
                        .unwrap_or_default();

                    println!("{}{}{}", s.1.name.cyan().bold(), tags_str, ext_str);

                    for line in s.1.content.lines() {
                        if line.to_lowercase().contains(&query_lower) {
                            println!("  {} {}", ">".yellow().bold(), line.dimmed());
                        }
                    }
                    println!();
                }
            }
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
