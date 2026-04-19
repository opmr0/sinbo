use dashmap::DashMap;
use dirs::config_dir;
use rayon::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

const TRIGGER: &str = "sinbo:";

struct Backend {
    client: Client,
    documents: DashMap<String, String>,
}

impl Backend {
    fn snippets_dir() -> PathBuf {
        config_dir()
            .unwrap_or_default()
            .join("sinbo")
            .join("snippets")
    }

    fn is_encrypted(name: &str) -> bool {
        Self::snippets_dir().join(format!("{}.enc", name)).exists()
    }

    fn get_content(name: &str) -> String {
        if Self::is_encrypted(name) {
            return "[encrypted]".to_string();
        }
        Command::new("sinbo")
            .args(["get", name])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default()
            .trim()
            .to_string()
    }

    fn list_names() -> Vec<String> {
        Command::new("sinbo")
            .arg("list-names")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect()
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![":".to_string()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "sinbo-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "sinbo-lsp ready")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.documents.insert(
            params.text_document.uri.to_string(),
            params.text_document.text,
        );
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.documents
                .insert(params.text_document.uri.to_string(), change.text);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri.to_string());
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let triggered = params
            .context
            .as_ref()
            .and_then(|c| c.trigger_character.as_deref())
            == Some(":");

        if !triggered {
            return Ok(None);
        }

        let uri = params.text_document_position.text_document.uri.to_string();
        let line_num = params.text_document_position.position.line as usize;
        let col = params.text_document_position.position.character as usize;

        let doc = match self.documents.get(&uri) {
            Some(d) => d.clone(),
            None => return Ok(None),
        };

        let line = match doc.lines().nth(line_num) {
            Some(l) => l.to_string(),
            None => return Ok(None),
        };

        let before = &line[..col.min(line.len())];
        if !before.ends_with(TRIGGER) {
            return Ok(None);
        }

        let names = Self::list_names();
        if names.is_empty() {
            return Ok(None);
        }

        let start_char = (col - TRIGGER.len()) as u32;
        let end_char = col as u32;

        let items: Vec<CompletionItem> = names
            .par_iter()
            .map(|name| {
                let content = Self::get_content(name);
                let encrypted = content == "[encrypted]";
                CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::SNIPPET),
                    detail: Some(if encrypted {
                        "sinbo snippet (encrypted)".to_string()
                    } else {
                        "sinbo snippet".to_string()
                    }),
                    filter_text: Some(format!("sinbo:{}", name)),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: start_char,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: end_char,
                            },
                        },
                        new_text: content,
                    })),
                    ..Default::default()
                }
            })
            .collect();

        Ok(Some(CompletionResponse::Array(items)))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: DashMap::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
