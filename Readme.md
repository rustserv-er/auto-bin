# auto-bin

`cargo auto-bin` 🦀 — a Cargo subcommand that automatically syncs all `src/bin/*.rs` files with `[[bin]]` entries in `Cargo.toml`.

No more manual editing of `Cargo.toml` when you add new binaries. Just run `cargo auto-bin` and your project stays in sync.  

---

## ✨ Features
- Scans `src/bin/*.rs` for binaries
- Adds missing `[[bin]]` entries to `Cargo.toml`
- Idempotent → safe to run multiple times
- `--status` command to preview changes
- Works with standard Cargo projects (workspace support planned)

---

## 📦 Installation
You need Rust and Cargo installed.

```bash
cargo install auto-bin
