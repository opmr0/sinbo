# sinbo — VS Code Extension

Snippet completions for [sinbo](https://github.com/opmr0/sinbo).

Type `sinbo:` anywhere in your editor to get a completion list of all your saved snippets. Selecting one replaces `sinbo:` with the full snippet content.

## Requirements

Both `sinbo` and `sinbo-lsp` must be installed and available in your PATH.

## Usage

1. Open any file
2. Type `sinbo:` 
3. A completion list of your snippets appears
4. Select one to insert the snippet content

## Commands

- `sinbo: Restart LSP Server` — restart the LSP server if completions stop working

## Troubleshooting

If completions are not showing:

1. Make sure `sinbo-lsp` is in your PATH: `sinbo-lsp --version`
2. Check the output panel: `View > Output > sinbo-lsp`
3. Run `sinbo: Restart LSP Server` from the command palette
4. If still broken, reload the window: `Developer: Reload Window`