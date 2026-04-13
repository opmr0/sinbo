# Changelog

All notable changes to sinbo will be documented here.

---

## 1.5.0 - 2026-04-13

### Added
- `rename <old> <new>` command to rename snippets
- `--peek` / `-p` flag on `list` to preview first 30 characters of snippet content
- `-a` short flag for `--args` on `get`
- `copy <snippet>` / `c <snippet>` command to copy a snippet
- add `human_panic`

## 1.4.0 - 2026-04-12

### Added
- Shell completions for bash, zsh, fish, and powershell (`sinbo completions <shell>`)
- Dynamic snippet name completion on TAB for `get`, `remove`, `edit`, `encrypt`, `decrypt`, `export`
- Hidden `list-names` command for shell completion scripts
- Unit tests for `var.rs`, `storage.rs`, `encryption.rs`, and `main.rs`

## 1.3.0 - 2026-04-12

### Added
- Variable substitution system with `SINBO:name:` placeholder syntax
- `--args key=value` flag on `get` for placeholder substitution
- `export` command — export snippets to `.sinbo.json` files
- `import` command — import snippets from `.sinbo.json` files
- Conflict resolution prompt on import/export name collision

## 1.2.1 - 2026-04-11

### Added
- Description field to snippet metadata

### Fixed
- Suppressed false-positive `RUSTSEC-2026-0097` advisory for `rand 0.8.5` (unsoundness does not affect sinbo's usage)

### CI
- Added `cargo audit` check to the pipeline

## 1.2.0 - 2026-04-10

### Added

- `sinbo encrypt <name>` — encrypt an existing plaintext snippet
- `sinbo decrypt <name>` — permanently decrypt an encrypted snippet

### Changed

- Bumped Argon2id memory cost from 19MB to 32MB and time cost from 2 to 3

## 1.2.0-beta - 2026-04-08

### Added

- **Snippet encryption** — `sinbo add <name> --encrypt` prompts for a password and stores the snippet as an encrypted `.enc` file. `sinbo get` detects encrypted snippets automatically and prompts for the password.
- Encrypted snippets are listed normally with a `Locked` indicator. `sinbo list --show` displays `[encrypted]` instead of the content.
- `sinbo search` skips the content of encrypted snippets — only the name is matched.
- `sinbo remove` now correctly deletes `.enc` files alongside metadata.

### Changed

- `storage::exists()` now checks for both `.code` and `.enc` files — previously encrypted snippets were invisible to duplicate checks.
- `sinbo edit` on an encrypted snippet now returns a clear error instead of silently operating on empty content.
- Editor undo corruption: if the editor is closed without saving, or the resulting content is identical to what was written initially (e.g. full undo in vim), sinbo now detects the unchanged state and aborts instead of saving empty or garbage content.
- `storage::save_meta()` extracted as a public method for use by encrypted add.
- `storage::Snippet` now carries an `encrypted: bool` field.

---

## 1.1.0 - 2026-04-01

### Added

- `sinbo search <query>` command — fuzzy search across snippet names and content
- Results are ranked by relevance score, with matching content lines shown inline
- `--tags` / `-t` flag on `search` to scope results to a specific tag

---

## 1.0.0 - Initial release

### Added

- `sinbo add` — add snippets via editor, stdin, or file
- `sinbo get` — print or copy a snippet to clipboard
- `sinbo list` — list all snippets, with tag filtering and `--show` for inline content
- `sinbo edit` — edit an existing snippet in `$EDITOR`, with optional tag update
- `sinbo remove` — delete a snippet
- `--ext` flag for syntax-highlighted temp files in editor
- Tag support across add, edit, and list
- Plain-file storage under system config directory
- Cross-platform: Linux, macOS, Windows
