<div align=right>Table of Contents↗️</div>

<img src="./assets/logo_no_bg.png" alt="sinbo logo" width="100"/>

<br>

# sinbo

[![Crates.io](https://img.shields.io/crates/v/sinbo)](https://crates.io/crates/sinbo)
[![Downloads](https://img.shields.io/crates/d/sinbo)](https://crates.io/crates/sinbo)
[![License](https://img.shields.io/crates/l/sinbo)](LICENSE)
[![Build](https://github.com/opmr0/sinbo/actions/workflows/release.yml/badge.svg)](https://github.com/opmr0/sinbo/actions)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org)

**sinbo is a CLI snippet manager. Store code once, retrieve it anywhere.**

---

## demo

![demo](assets/demo.gif)

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

### `sinbo add <n>`

Add a new snippet. Opens `$EDITOR` if no input is piped.

```bash
sinbo add rust-test -e rs                  # open editor with .rs syntax
sinbo add center-div -f style.css          # read from file
sinbo add docker-run -t docker infra       # add with tags
sinbo add api-key --encrypt                # encrypt with a password
sinbo get docker-run --args port=8080 name=myapp   # substitute placeholders
echo "hello world" | sinbo add greeting   # read from stdin
```

| Flag            | Short | Description                            |
| --------------- | ----- | -------------------------------------- |
| `--ext`         | `-e`  | File extension for syntax highlighting |
| `--file-name`   | `-f`  | Read content from a file               |
| `--tags`        | `-t`  | Tag the snippet                        |
| `--description` | `-d`  | Add a description to the snippet       |
| `--args`        |       | Substitute placeholders (`key=value`)  |
| `--encrypt`     |       | Encrypt the snippet with a password    |

---

### `sinbo get <n>`

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

List all saved snippets. Encrypted snippets are shown with a `Locked` indicator.

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

| Flag     | Short | Description |
| -------- | ----- | ----------- |
| `--tags` | `-t`  | Update tags |

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
sinbo get docker-run --args port=8080 name=myapp
# output: docker run -p 8080 -it myapp
```

If a placeholder has no matching `--args` value, it is left as-is in the output.

---

## Export / Import

Snippets can be exported to `.sinbo.json` files and imported back.

```bash
sinbo export docker-run                      # export to current directory
sinbo import ~/backups/docker-run.sinbo.json # import from file
```

Encrypted snippets cannot be exported, decrypt them first.

If a name conflict is detected on import or export, sinbo will prompt you to overwrite or rename.

---

## Piping

Since `sinbo get` prints to stdout, snippets compose naturally with other tools:

```bash
sinbo get deploy-script | sh          # run a shell snippet
sinbo get query | psql mydb           # pipe into psql
sinbo get nginx-conf | sudo tee /etc/nginx/nginx.conf   # write to a file
sinbo get docker-run --args port=8080 | sh   # substitute then run
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

## How It Works

Snippets are stored as plain files in your system config directory:

- **Linux/macOS:** `~/.config/sinbo/snippets/`
- **Windows:** `%APPDATA%\sinbo\snippets\`

| File                | Contents                       |
| ------------------- | ------------------------------ |
| `{name}.code`       | Plaintext snippet content      |
| `{name}.enc`        | Encrypted snippet content      |
| `{name}.meta.json`  | Tags, extension, and timestamp |
| `{name}.sinbo.json` | Exported snippet file          |

Plain `.code` files are grep-able, copyable, and easy to back up directly.

---

## License

MIT - [LICENSE](LICENSE)
