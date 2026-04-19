<table border="0" cellspacing="0" cellpadding="10">
<tr>
<td><img src="./assets/logo_no_bg.png" alt="sinbo logo" width="100"/></td>
<td><h1>sinbo</h1><p>Store code once, retrieve it anywhere.</p></td>
</tr>
</table>

[![Crates.io](https://img.shields.io/crates/v/sinbo)](https://crates.io/crates/sinbo)
[![Downloads](https://img.shields.io/crates/d/sinbo)](https://crates.io/crates/sinbo)
[![License](https://img.shields.io/crates/l/sinbo)](LICENSE)
[![Build](https://github.com/opmr0/sinbo/actions/workflows/release.yml/badge.svg)](https://github.com/opmr0/sinbo/actions)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org)

**sinbo is a CLI snippet manager. Store code once, retrieve it anywhere.**

## ToC

<details>
<summary>Click to expand</summary>

- [Demo](#demo)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Setup](#setup)
- [Commands](#commands)
  - [add](#sinbo-add-n)
  - [get](#sinbo-get-n)
  - [copy](#sinbo-copy-n)
  - [list](#sinbo-list)
  - [search](#sinbo-search-query)
  - [edit](#sinbo-edit-n)
  - [rename](#sinbo-rename-old-new)
  - [remove](#sinbo-remove-n)
  - [encrypt](#sinbo-encrypt-n)
  - [decrypt](#sinbo-decrypt-n)
- [Tags](#tags)
- [Encryption](#encryption)
- [Variables](#variables)
- [Export / Import](#export--import)
- [Piping](#piping)
- [Shell Completions](#shell-completions)
- [Editor Integration](#editor-integration)
- [How It Works](#how-it-works)
- [License](#license)

</details>

---

## Demo

![demo](assets/demo.gif)

---

## Installation

**macOS / Linux**

```bash
curl -sSf https://raw.githubusercontent.com/opmr0/sinbo/main/install.sh | sh
```

**Windows**

```powershell
iwr https://raw.githubusercontent.com/opmr0/sinbo/main/install.ps1 -UseBasicParsing | iex
```

**Via cargo**

```bash
cargo install sinbo
```

---

## Quick Start

```bash
sinbo add rust-test -e rs        # open editor, save as a Rust snippet
sinbo get rust-test              # print the snippet
sinbo copy rust-test             # copy to clipboard
sinbo list                       # list all snippets
sinbo search "hello"             # fuzzy search across all snippets
```

---

## Setup

sinbo opens your `$EDITOR` when adding or editing snippets. Set it in your shell config:

```bash
export EDITOR="vim"           # Vim
export EDITOR="nvim"          # Neovim
export EDITOR="nano"          # Nano
export EDITOR="hx"            # Helix
export EDITOR="code --wait"   # VSCode   (--wait is required)
export EDITOR="idea --wait"   # IntelliJ (--wait is required)
export EDITOR="subl --wait"   # Sublime  (--wait is required)
```

> The `--ext` flag sets the temp file extension so your editor applies the right syntax highlighting. `sinbo add my-snippet -e rs` opens a `.rs` file.

---

## Commands

### `sinbo add <n>`

Add a new snippet. Opens `$EDITOR` if no input is piped.

```bash
sinbo add rust-test -e rs              # open editor with .rs syntax
sinbo add center-div -f style.css      # read from file
sinbo add docker-run -t docker infra   # add with tags
sinbo add api-key --encrypt            # encrypt with a password
echo "hello world" | sinbo add greeting   # read from stdin
```

| Flag            | Short | Description                            |
| --------------- | ----- | -------------------------------------- |
| `--ext`         | `-e`  | File extension for syntax highlighting |
| `--file-name`   | `-f`  | Read content from a file               |
| `--tags`        | `-t`  | Tag the snippet                        |
| `--description` | `-d`  | Add a description to the snippet       |
| `--encrypt`     |       | Encrypt the snippet with a password    |

---

### `sinbo get <n>`

Print a snippet to stdout. Prompts for a password if the snippet is encrypted.

```bash
sinbo get rust-test                            # print to stdout
sinbo get docker-run -a port=8080 name=myapp   # substitute placeholders
sinbo get api-key                              # prompts for password if encrypted
```

| Flag     | Short | Description                           |
| -------- | ----- | ------------------------------------- |
| `--args` | `-a`  | Substitute placeholders (`key=value`) |
| `--copy` | `-c`  | Copy to clipboard                     |

---

### `sinbo copy <n>`

Copy a snippet to clipboard. Prompts for a password if the snippet is encrypted.

```bash
sinbo copy rust-test                            # copy to clipboard
sinbo copy docker-run -a port=8080 name=myapp   # substitute then copy
```

| Flag     | Short | Description                           |
| -------- | ----- | ------------------------------------- |
| `--args` | `-a`  | Substitute placeholders (`key=value`) |

---

### `sinbo list`

List all saved snippets. Encrypted snippets are shown with a `Locked` indicator.

```bash
sinbo list              # list all
sinbo list -t docker    # filter by tag
sinbo list -s           # show full content
sinbo list -p           # preview first 25 characters
```

| Flag     | Short | Description                            |
| -------- | ----- | -------------------------------------- |
| `--tags` | `-t`  | Filter by tags                         |
| `--show` | `-s`  | Show full snippet content              |
| `--peek` | `-p`  | Preview first 30 characters of content |

---

### `sinbo search <query>`

Fuzzy search across snippet names and content. Encrypted snippet content is not searched.

```bash
sinbo search "docker run"          # search all snippets
sinbo search "deploy" -t infra     # search within a tag
```

| Flag     | Short | Description                       |
| -------- | ----- | --------------------------------- |
| `--tags` | `-t`  | Scope search to snippets with tag |

---

### `sinbo edit <n>`

Edit an existing snippet in `$EDITOR`. Preserves extension for syntax highlighting. Encrypted snippets cannot be edited.

```bash
sinbo edit rust-test
sinbo edit rust-test -t rust testing   # update tags while editing
```

| Flag            | Short | Description        |
| --------------- | ----- | ------------------ |
| `--tags`        | `-t`  | Update tags        |
| `--description` | `-d`  | Update description |

---

### `sinbo rename <old> <new>`

Rename an existing snippet.

```bash
sinbo rename old-name new-name
```

---

### `sinbo remove <n>`

Delete a snippet. Prompts for confirmation.

```bash
sinbo remove rust-test
```

---

### `sinbo encrypt <n>`

Encrypt an existing plaintext snippet. Prompts for a password twice. The plaintext file is removed after encryption.

```bash
sinbo encrypt api-key
```

---

### `sinbo decrypt <n>`

Permanently decrypt an encrypted snippet back to plaintext. Prompts for the password.

```bash
sinbo decrypt api-key
```

---

## Tags

Tags let you group and filter snippets:

```bash
sinbo add nginx-conf -t infra server
sinbo add k8s-deploy -t infra k8s
sinbo list -t infra             # shows both
sinbo search "deploy" -t infra  # search within infra tag
```

---

## Encryption

Sensitive snippets like API keys and tokens can be encrypted at rest.

```bash
sinbo add github-token --encrypt   # new encrypted snippet
sinbo encrypt github-token         # encrypt an existing snippet
sinbo decrypt github-token         # permanently decrypt
sinbo get github-token             # prompts for password
```

Encryption uses AES-256-GCM with Argon2id key derivation. The plaintext never touches disk, only the `.enc` file is stored. Encrypted snippets appear in `list` and `search` normally, but their content is never shown or searched.

> [!WARNING]
> sinbo encryption protects against casual filesystem access. It is not a substitute for a dedicated secrets manager like Bitwarden for high-value credentials.

> [!WARNING]
> `.enc` files are binary. Do not open or edit them in a text editor, any modification will corrupt the file permanently.

---

## Variables

Snippets can contain placeholders using the `SINBO:name:` syntax.

```bash
# snippet content:
docker run -p SINBO:port: -it SINBO:name:

# usage:
sinbo get docker-run -a port=8080 name=myapp
# output: docker run -p 8080 -it myapp
```

Fallback values are supported, if no `-a` value is provided the fallback is used:

```bash
# snippet content:
docker run -p SINBO:port:8080: -it SINBO:name:myapp:

# usage without args:
sinbo get docker-run
# output: docker run -p 8080 -it myapp
```

---

## Export / Import

Snippets can be exported to `.sinbo.json` files and imported back.

```bash
sinbo export docker-run                          # export to current directory
sinbo export docker-run -p ~/backups             # export to a specific directory
sinbo import ~/backups/docker-run.sinbo.json     # import from file
```

Encrypted snippets cannot be exported, decrypt them first. If a name conflict is detected on import or export, sinbo will prompt you to overwrite or rename.

---

## Piping

Since `sinbo get` prints to stdout, snippets compose naturally with other tools:

```bash
sinbo get deploy-script | sh                             # run a shell snippet
sinbo get query | psql mydb                              # pipe into psql
sinbo get nginx-conf | sudo tee /etc/nginx/nginx.conf    # write to a file
sinbo get docker-run -a port=8080 | sh                   # substitute then run
```

---

## Shell Completions

**bash**

```bash
echo 'eval "$(sinbo completions bash)"' >> ~/.bashrc && source ~/.bashrc
```

**zsh**

```bash
echo 'eval "$(sinbo completions zsh)"' >> ~/.zshrc && source ~/.zshrc
```

**fish**

```bash
sinbo completions fish > ~/.config/fish/completions/sinbo.fish
```

**powershell**

```powershell
Add-Content $PROFILE "`nsinbo completions powershell | Invoke-Expression"
```

---

## Editor Integration

sinbo-lsp provides inline snippet completions in any editor. Type `sinbo:` to get a completion list of all your saved snippets, selecting one inserts the full content.

See [sinbo-lsp/README.md](sinbo-lsp/README.md) for installation and editor setup.

---

## How It Works

Snippets are stored as plain files in your system config directory:

- **Linux/macOS:** `~/.config/sinbo/snippets/`
- **Windows:** `%APPDATA%\sinbo\snippets\`

| File               | Contents                       |
| ------------------ | ------------------------------ |
| `{name}.code`      | Plaintext snippet content      |
| `{name}.enc`       | Encrypted snippet content      |
| `{name}.meta.json` | Tags, extension, and timestamp |

Plain `.code` files are grep-able, copyable, and easy to back up directly.

---

## License

MIT - [LICENSE](LICENSE)