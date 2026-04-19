# sinbo-lsp

LSP server for [sinbo](https://github.com/opmr0/sinbo). Type `sinbo:` in any file to get snippet completions.

## Installation

**macOS / Linux**
```bash
curl -sSf https://raw.githubusercontent.com/opmr0/sinbo/main/install_lsp.sh | sh
```

**Windows**
```powershell
iwr https://raw.githubusercontent.com/opmr0/sinbo/main/install_lsp.ps1 -UseBasicParsing | iex
```

## Editor Setup

### VS Code

Download the latest `.vsix` from [releases](https://github.com/opmr0/sinbo/releases/latest) then install it:

```bash
code --install-extension sinbo-lsp-0.1.0.vsix
```

Or via the UI: `Extensions > ... > Install from VSIX`

### Emacs

Using `lsp-mode`. Add to your config:

```elisp
(require 'lsp-mode)

(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(fundamental-mode . "plaintext"))
  (lsp-register-client
   (make-lsp-client
    :new-connection (lsp-stdio-connection "sinbo-lsp")
    :major-modes '(fundamental-mode text-mode prog-mode)
    :server-id 'sinbo-lsp)))

(add-hook 'find-file-hook #'lsp)
```

---

### IntelliJ IDEA / CLion / other JetBrains IDEs

Install the [LSP4IJ](https://plugins.jetbrains.com/plugin/23257-lsp4ij) plugin, then add a new LSP server:

1. Go to `Settings > Languages & Frameworks > LSP4IJ > New Language Server`
2. Set command to `sinbo-lsp`
3. Set file associations to `*` or specific extensions you want
4. Apply and restart

---

### Sublime Text

Install the [LSP](https://packagecontrol.io/packages/LSP) package via Package Control, then add to your LSP settings (`Preferences > Package Settings > LSP > Settings`):

```json
{
  "clients": {
    "sinbo-lsp": {
      "enabled": true,
      "command": ["sinbo-lsp"],
      "selector": "source, text"
    }
  }
}
```

---

### Vim

Install [vim-lsp](https://github.com/prabirshrestha/vim-lsp), then add to your `.vimrc`:

```vim
if executable('sinbo-lsp')
  au User lsp_setup call lsp#register_server({
    \ 'name': 'sinbo-lsp',
    \ 'cmd': {server_info->['sinbo-lsp']},
    \ 'allowlist': ['*'],
    \ })
endif
```

---

### Neovim

Add to your config (`~/.config/nvim/init.lua`):

```lua
vim.api.nvim_create_autocmd("FileType", {
    pattern = "*",
    callback = function()
        vim.lsp.start({
            name = "sinbo-lsp",
            cmd = { "sinbo-lsp" },
            root_dir = vim.fn.getcwd(),
        })
        vim.bo.omnifunc = "v:lua.vim.lsp.omnifunc"
    end,
})
```

---

### Helix

Add to `~/.config/helix/languages.toml`:

```toml
[language-server.sinbo-lsp]
command = "sinbo-lsp"

[[language]]
name = "markdown"
language-servers = ["sinbo-lsp"]
```

Add more `[[language]]` blocks for other file types as needed.

---

### Zed

Add to `~/.config/zed/settings.json`:

```json
{
  "lsp": {
    "sinbo-lsp": {
      "binary": {
        "path": "sinbo-lsp"
      }
    }
  }
}
```

---

### Other Editors

Any editor with LSP support can use sinbo-lsp. Point your editor's LSP client at the `sinbo-lsp` binary using stdio transport. Check your editor's LSP configuration documentation for the exact steps.

## Usage

Type `sinbo:` anywhere in your editor, a completion list of your saved snippets appears. Select one to insert the full snippet content. Encrypted snippets show as `[encrypted]` and are not inserted.

