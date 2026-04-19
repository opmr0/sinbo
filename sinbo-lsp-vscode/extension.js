const { LanguageClient, TransportKind } = require("vscode-languageclient/node");
const vscode = require("vscode");

let client;

function activate(context) {
    const serverOptions = {
        run: {
            command: "sinbo-lsp",
            transport: TransportKind.stdio,
        },
        debug: {
            command: "sinbo-lsp",
            transport: TransportKind.stdio,
        },
    };

    const clientOptions = {
        documentSelector: [{ scheme: "file", pattern: "**/*" }],
        outputChannelName: "sinbo-lsp",
    };

    client = new LanguageClient(
        "sinbo-lsp",
        "sinbo",
        serverOptions,
        clientOptions
    );

    client.start().catch((err) => {
        vscode.window.showErrorMessage(
            `sinbo-lsp failed to start: ${err.message}\n\nMake sure sinbo-lsp is installed and in your PATH.`
        );
    });

    context.subscriptions.push(
        vscode.commands.registerCommand("sinbo.restart", async () => {
            await client.stop();
            await client.start();
            vscode.window.showInformationMessage("sinbo-lsp restarted");
        })
    );
}

function deactivate() {
    if (client) return client.stop();
}

module.exports = { activate, deactivate };