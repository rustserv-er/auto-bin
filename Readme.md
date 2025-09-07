# auto-bin 🚀  

[![Crates.io](https://img.shields.io/crates/v/auto-bin.svg)](https://crates.io/crates/auto-bin)  
[![Docs](https://docs.rs/auto-bin/badge.svg)](https://docs.rs/auto-bin)  

**auto-bin** is a Cargo subcommand that boosts Rust productivity by **automating the setup of `src/bin` binaries**.  
No more manually editing `Cargo.toml` every time you add a new binary!  

---

## ✨ Features
- 🔍 Detects all `.rs` files inside `src/bin/`  
- ⚡ Automatically syncs them into `Cargo.toml` as bin targets  
- 📦 Simple CLI interface (`init`, `status`)  
- 🚀 Saves time and reduces errors in multi-bin projects  

---

## 📦 Installation
```bash
cargo install auto-bin
```

---

## 🛠️ Usage

```bash
# Initialize auto-bin for your Rust project
cargo-auto-bin --init

# Check current configuration
cargo-auto-bin --status
```

---

## 🔮 Example
Suppose you have:
```
src/bin/foo.rs
src/bin/bar.rs
```

After running:
```bash
cargo-auto-bin --init
```

Your `Cargo.toml` will automatically get updated with:
```toml
[[bin]]
name = "foo"
path = "src/bin/foo.rs"

[[bin]]
name = "bar"
path = "src/bin/bar.rs"
```

---

## 📂 Project Goals
- Provide a smooth developer experience for Rust multi-bin projects  
- Eliminate boilerplate Cargo setup  
- Keep everything minimal, fast, and idiomatic  

---

## 🤝 Contributing
PRs, issues, and ideas are always welcome!  

---

## 📜 License
This project is licensed under the **MIT License**.  
See [LICENSE](LICENSE) for details.  
