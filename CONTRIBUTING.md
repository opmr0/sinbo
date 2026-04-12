# Contributing

Contributions are welcome, bug fixes, new features, or doc improvements.

If you're planning something large, [open an issue](https://github.com/opmr0/sinbo/issues) first so we can discuss it before you invest the time.

---

## Getting Started

```bash
git clone https://github.com/opmr0/sinbo
cd sinbo
cargo build
cargo test
```

---

## Making Changes

- Keep changes focused, one fix or feature per PR
- Match the existing code style
- If you're adding a feature, update the README too
- If you're fixing a bug, explain what caused it in the PR description

---

## Before Submitting

```bash
cargo fmt
cargo clippy  # no warnings
cargo test    # all pass
cargo audit   # no unignored advisories
```

---

## Project Structure

```
src/
  main.rs        # CLI parsing and command dispatch
  storage.rs     # Snippet storage and retrieval
  encryption.rs  # AES-256-GCM encryption and Argon2id key derivation
```

---

## Reporting Bugs

Open an issue and include:

- What you ran
- What you expected
- What actually happened
- Your OS and sinbo version (`sinbo --version`)