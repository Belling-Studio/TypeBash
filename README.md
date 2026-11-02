# TypeBash 🚀
> A Rust-powered static type checker and compiler that brings strict typing and safety features to Bash scripts.

TypeBash transforms vulnerable, error-prone shell scripts into safe, robust, and highly predictable production code using compiler diagnostics heavily inspired by Rust.

---

## ⚡ Key Features

- 🦀 **Rust-like Diagnostics:** Human-readable compiler errors with strict codes, line highlights, and helpful hints.
- 🛡️ **DevOps Shield:** Automatically verifies whether invoked system utilities (e.g., `git`, `docker`, `curl`) exist in your path during compilation.
- 📐 **Strict Typing & Scoping:** Full support for `string`, `number`, `boolean`, generic arrays (`Array<T>`), and an intelligent path-validation type (`Path`).
- 🔒 **Secure Bash Output:** Automatically generates production-ready `.sh` files configured with `set -euo pipefail` and a beautiful native runtime error handler (`trap ERR`).

---

## 💻 Code Example

Write clean and typed code in `script.tb`:

```typebash
let backupDir: Path = "./backups"
let maxAttempts: number = 3

fn notify(msg: string) {
    print(msg)
}

notify("TypeBash v0.5.0-beta: Initializing secure script execution...")

let targetFiles: Array<string> = ["config.json", "database.sql"]
for file in targetFiles {
    print(file)
}

exec "mkdir -p ./backups"
```

Compile it with a single command:
```bash
typebash build script.tb
```

And get an optimized, safe `script.sh` with automatic upper-case transformations, `readonly` variables, and runtime crash guards!

---

## 🚀 Quick Start

1. Clone the repository and build the release binary:
   ```bash
   cargo build --release
   ```
2. Run compilation checks:
   ```bash
   ./target/release/typebash build your_script.tb
   ```

## 📜 License
Distributed under the **MIT License**. See `LICENSE` for more information.
