// cli/src/lsp.rs

use async_trait::async_trait;
use tower_lsp::{LspService, Server};
use tower_lsp::lsp_types::*;
use std::error::Error;

// ✅ Assuming core is a separate crate with a parser module
use core::parser::Parser as EdlParser;

pub async fn start_lsp_async() -> Result<(), Box<dyn Error + Send + Sync>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}

// Optional wrapper for sync context; used by main
pub async fn start_lsp() -> Result<(), Box<dyn Error + Send + Sync>> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(start_lsp_async())
}

#[derive(Clone)]
struct Backend {
    client: tower_lsp::Client,
}

#[async_trait]
impl tower_lsp::LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult, tower_lsp::jsonrpc::Error> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "EDL Language Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<(), tower_lsp::jsonrpc::Error> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let code = params.text_document.text;
        let mut parser = EdlParser::new(&code);

        match parser.parse() {
            Ok(_) => {
                self.client
                    .log_message(MessageType::INFO, "Parsed successfully")
                    .await;
            }
            Err(err) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Parse error: {:?}", err))
                    .await;
            }
        }
    }
}