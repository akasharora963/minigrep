# Text Search CLI (Mini-Grep in Rust)

A simple command-line utility written in Rust to search for lines containing a query string within a file. This is a minimal version of the `grep` tool and is great for learning purposes.

---

## 📦 Features

- Search for text within a file (like `grep`)
- Case-sensitive or case-insensitive search (toggle via environment variable)
- Clean and testable architecture
- Lifetime-aware and memory efficient
- Includes unit tests

---

## 🔧 Usage

```bash
cargo run -- <query> <filename>
