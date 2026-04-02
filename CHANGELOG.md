# Changelog

All notable changes to sinbo will be documented here.

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