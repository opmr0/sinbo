<img src="./assets/logo_no_bg.png" alt="sinbo logo" width="100"/>

<br>

# Sinbo

[![Crates.io](https://img.shields.io/crates/v/sinbo)](https://crates.io/crates/sinbo)
[![Downloads](https://img.shields.io/crates/d/sinbo)](https://crates.io/crates/sinbo)
[![License](https://img.shields.io/crates/l/sinbo)](LICENSE)
[![Build](https://github.com/opmr0/sinbo/actions/workflows/release.yml/badge.svg)](https://github.com/opmr0/sinbo/actions)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org)

**sinbo is a CLI snippet manager. Store code once, retrieve it anywhere.**

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
sinbo get rust-test --copy       # copy to clipboard
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

### `sinbo add <name>`

Add a new snippet. Opens `$EDITOR` if no input is piped.

```bash
sinbo add rust-test -e rs                  # open editor with .rs syntax
sinbo add center-div -f style.css          # read from file
sinbo add docker-run -t docker infra       # add with tags
sinbo add api-key --encrypt                # encrypt with a password
echo "hello world" | sinbo add greeting   # read from stdin
```

| Flag          | Short | Description                            |
| ------------- | ----- | -------------------------------------- |
| `--ext`       | `-e`  | File extension for syntax highlighting |
| `--file-name` | `-f`  | Read content from a file               |
| `--tags`      | `-t`  | Tag the snippet                        |
| `--encrypt`   |       | Encrypt the snippet with a password    |

---

### `sinbo get <name>`

Print or copy a snippet. Prompts for a password if the snippet is encrypted.

```bash
sinbo get rust-test          # print to stdout
sinbo get rust-test --copy   # copy to clipboard
sinbo get api-key            # prompts for password if encrypted
```

| Flag     | Short | Description       |
| -------- | ----- | ----------------- |
| `--copy` | `-c`  | Copy to clipboard |

---

### `sinbo list`

List all saved snippets. Encrypted snippets are shown with a `locked` indicator.

```bash
sinbo list              # list all
sinbo list -t docker    # filter by tag
sinbo list -s           # show content (encrypted snippets show [encrypted])
```

| Flag     | Short | Description          |
| -------- | ----- | -------------------- |
| `--tags` | `-t`  | Filter by tags       |
| `--show` | `-s`  | Show snippet content |

---

### `sinbo search <query>`

Fuzzy search across snippet names and content. Results are ranked by relevance. Encrypted snippet content is not searched.

```bash
sinbo search "docker run"          # search all snippets
sinbo search "deploy" -t infra     # search within a tag
```

| Flag     | Short | Description                       |
| -------- | ----- | --------------------------------- |
| `--tags` | `-t`  | Scope search to snippets with tag |

---

### `sinbo edit <name>`

Edit an existing snippet in `$EDITOR`. Preserves extension for syntax highlighting. Encrypted snippets cannot be edited , remove and re-add them.

```bash
sinbo edit rust-test
sinbo edit rust-test -t rust testing   # update tags while editing
```

| Flag     | Short | Description |
| -------- | ----- | ----------- |
| `--tags` | `-t`  | Update tags |

---

### `sinbo remove <name>`

Delete a snippet.

```bash
sinbo remove rust-test
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

Sensitive snippets like API keys and tokens can be encrypted with `--encrypt`:

```bash
sinbo add github-token --encrypt   # prompts for a password twice
sinbo get github-token             # prompts for the password to decrypt
```

Encryption uses AES-256-GCM with Argon2id key derivation. The plaintext never touches disk, only the encrypted `.enc` file is stored. Encrypted snippets are listed normally but their content is never shown or searched.

> Encrypted snippets cannot be edited. Remove and re-add them if you need to update the content.

> [!WARNING]
> sinbo encryption is designed to protect against casual filesystem access.
> It is not a substitute for a dedicated secrets manager like Bitwarden
> for high-value credentials.

> [!WARNING]
> `.enc` files are binary, do NOT open or edit them in a text editor.
> Any modification, including saving without changes, will corrupt the file.

---

## How It Works

Snippets are stored as plain files in your system config directory:

- **Linux/macOS:** `~/.config/sinbo/snippets/`
- **Windows:** `%APPDATA%\sinbo\snippets\`

Each snippet has up to two files:

| File               | Contents                       |
| ------------------ | ------------------------------ |
| `{name}.code`      | Plaintext snippet content      |
| `{name}.enc`       | Encrypted snippet content      |
| `{name}.meta.json` | Tags, extension, and timestamp |

Plain `.code` files are grep-able, copyable, and easy to back up directly.

---

## License

MIT - [LICENSE](LICENSE)
