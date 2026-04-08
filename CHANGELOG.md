# Changelog

All notable changes to sinbo will be documented here.

---

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
