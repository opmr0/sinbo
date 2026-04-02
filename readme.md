<img src="./assets/logo_no_bg.png" alt="logo" width="100"/>

<br>

# sinbo

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
sinbo add rust-test -e rs        # opens your editor, saves as a Rust snippet
sinbo get rust-test              # prints the snippet
sinbo get rust-test --copy       # copies to clipboard
sinbo search "hello"             # fuzzy search across all snippets
```

---

## Setup

sinbo opens your `$EDITOR` when adding or editing snippets. Set it in your shell config:

```bash
export EDITOR="vim"                    # Vim
export EDITOR="nvim"                   # Neovim
export EDITOR="nano"                   # Nano
export EDITOR="hx"                     # Helix
export EDITOR="code --wait"            # VSCode (--wait is required)
export EDITOR="idea --wait"            # IntelliJ
export EDITOR="pycharm --wait"         # PyCharm
export EDITOR="subl --wait"            # Sublime Text
```

The temp file uses the `--ext` flag for syntax highlighting. `sinbo add my-snippet -e rs` opens the editor with a `.rs` file so your editor formats the language syntax correctly.

---

## Commands

### `sinbo add <name>`

Add a new snippet. Opens your `$EDITOR` if no input is piped.

```bash
sinbo add rust-test -e rs                  # open editor with .rs extension
sinbo add center-div -f style.css          # read from file
sinbo add docker-run -t docker infra       # add with tags
echo "hello world" | sinbo add greeting   # read from stdin
```

| Flag          | Short | Description                            |
| ------------- | ----- | -------------------------------------- |
| `--ext`       | `-e`  | File extension for syntax highlighting |
| `--file-name` | `-f`  | Read content from a file               |
| `--tags`      | `-t`  | Tag the snippet                        |

---

### `sinbo get <name>`

Print or copy a snippet.

```bash
sinbo get rust-test          # print to stdout
sinbo get rust-test --copy   # copy to clipboard
```

| Flag     | Short | Description                   |
| -------- | ----- | ----------------------------- |
| `--copy` | `-c`  | Copy to clipboard             |

---

### `sinbo list`

List all saved snippets.

```bash
sinbo list              # list all
sinbo list -t docker    # filter by tag
sinbo list -s           # show snippet content
```

| Flag     | Short | Description            |
| -------- | ----- | ---------------------- |
| `--tags` | `-t`  | Filter by tags         |
| `--show` | `-s`  | Show snippet content   |

---

### `sinbo search <query>`

Fuzzy search across snippet names and content. Results are ranked by relevance. Optionally scope the search to a tag.

```bash
sinbo search "docker run"          # search all snippets
sinbo search "deploy" -t infra     # search within a tag
```

| Flag     | Short | Description                        |
| -------- | ----- | ---------------------------------- |
| `--tags` | `-t`  | Scope search to snippets with tag  |

Matching content lines are shown inline under each result.

---

### `sinbo edit <name>`

Edit an existing snippet in your `$EDITOR`. Preserves the extension for syntax highlighting.

```bash
sinbo edit rust-test
sinbo edit rust-test -t rust testing   # update tags while editing
```

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
sinbo list -t infra            # shows both
sinbo search "deploy" -t infra # search within infra tag
```

---

## How It Works

- Snippets are stored as plain files in your system config directory
- Each snippet has two files: `{name}.code` for content and `{name}.meta.json` for tags and metadata
- On Linux/macOS: `~/.config/sinbo/snippets/`
- On Windows: `%APPDATA%\sinbo\snippets\`
- Files are plain text, you can grep, copy, or back them up directly

---

## License

MIT - [LICENSE](LICENSE)