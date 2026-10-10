# 🦀 Rust File Organizer

> Fast and safe command-line file organizer built with Rust.

![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)
![CLI](https://img.shields.io/badge/interface-CLI-black)
![License](https://img.shields.io/badge/license-MIT-green)

Point the program at a directory and it sorts files into categories such as Images, Documents, Audio, Video, Archives and Code.

## ✨ Features

- Extension-based categorization
- Dry-run mode before moving anything
- Collision-safe filenames
- Ignores directories
- Clear movement summary
- Standard-library-only implementation

## 🚀 Run

Preview changes first:

```bash
cargo run -- ~/Downloads --dry-run
```

Organize for real:

```bash
cargo run -- ~/Downloads
```

## 📁 Categories

Images, Documents, Audio, Video, Archives, Code and Other.

## 🛡️ Safety

The program never deletes files. If a destination filename already exists, it generates a new numbered name instead of overwriting it.

## 🧠 What it demonstrates

Rust filesystem APIs, enums, pattern matching, error propagation, path handling and defensive file operations.

## 📄 License

MIT.

## 🆕 Recent changes

### 2026-10-09

- `--only` now validates category names and prints the supported options when the value is invalid.

### 2026-10-08

- The final summary now shows the total size of files organized or previewed.

### 2026-10-07

- Added `--only <category>` to organize just one category while leaving other files untouched.

### 2026-10-06

- The CLI now shows usage for missing folders instead of panicking.

### 2026-10-05

- The summary now reports how many hidden files were skipped when `--include-hidden` is not used.

### 2026-10-04

- The final summary now breaks down organized files by category.

### Previous update

- Hidden files are now skipped by default, with a new `--include-hidden` option when you want to organize them too.
