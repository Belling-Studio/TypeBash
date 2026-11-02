# TypeBash

<img width="280" height="282" alt="download" src="https://github.com/user-attachments/assets/c77f611e-50ac-4598-8894-43f390be479a" />

![License](https://github.com/Belling-Studio/TypeBash/blob/main/LICENSE) ![Built with Rust](https://rust-lang.org/)

A Rust-powered static type checker and compiler that brings strict typing and safety features to Bash scripts [INDEX].

TypeBash transforms vulnerable, error-prone shell scripts into safe, robust, and highly predictable production code using strict compiler diagnostics heavily inspired by `rustc` [INDEX].

---

## ⚡ Key Features

* 🦀 **Rust-like Diagnostics:** Human-readable compiler errors with strict codes (`E100`-`E400`), exact line highlights, and helpful hints [INDEX].
* 🛡️ **DevOps Shield:** Automatically verifies whether invoked system utilities (like `docker`, `git`, `curl`) exist in your host `PATH` during compilation [INDEX].
* 📐 **Strict Typing & Scoping:** Full support for `string`, `number`, `boolean`, generic arrays (`Array<T>`), and an intelligent path-validation type (`Path`) [INDEX].
* 🔒 **Secure Bash Output:** Automatically generates production-ready `.sh` files configured with `set -euo pipefail` and a native runtime error handler (`trap ERR`) [INDEX].

---

## 💻 Code Example

Write clean, typed, and structured code in `script.tb`:

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

### 📦 Optimized Output (`script.sh`)

Your code is automatically compiled into professional shell script formatting [INDEX]:
* All local/global scopes are handled correctly with `local` variables [INDEX].
* Normal variables are forced into `readonly` state to prevent unexpected tampering [INDEX].
* Automatic conversion of variables into shell-compliant `UPPERCASE` [INDEX].

---

## 🚀 Quick Start

1. **Clone the repository and build the release binary:**
   ```bash
   cargo build --release
   ```
2. **Run compilation checks and generate code:**
   ```bash
   ./target/release/typebash build your_script.tb
   ```

## 📜 License
Distributed under the **MIT License**. See `LICENSE` for more information [INDEX].
