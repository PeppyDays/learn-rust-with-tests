# Install Rust

## Install rustup and Toolchains

The official Rust installation guide is available [here](https://www.rust-lang.org/tools/install). After installing `rustup`, update to the latest stable version and install the nightly toolchain:

```bash
rustup update stable
rustup toolchain install nightly
```

## Refactoring and Tooling

### Refactoring

This book places a strong emphasis on refactoring. Your tools should enable you to perform large-scale refactoring with confidence. Master these essential operations with simple keyboard shortcuts:

- **Extract/Inline variable**
  - Transform magic values into named constants to simplify your code
- **Extract method/function**
  - Essential for breaking complex code into manageable functions
- **Rename symbols**
  - Confidently rename identifiers across your entire codebase
- **Format code**
  - [rustfmt](https://github.com/rust-lang/rustfmt) automatically formats your code according to Rust style guidelines
  - Configure your editor to run this on save
- **Lint code**
  - [clippy](https://github.com/rust-lang/rust-clippy) catches common mistakes and suggests improvements
  - Configure your editor to run this on save
- **Run tests**
  - Quickly verify that your refactoring hasn't broken functionality

Additionally, your development environment should support:

- **View function signature**
  - Never guess how to call a function
  - Your IDE should display documentation, parameters, and return types
- **View function definition**
  - Jump to source code when you need deeper understanding
- **Find usages of a symbol**
  - Understand how code is used before refactoring

Mastering these tools helps you focus on writing code instead of fighting your environment.

### Tooling

Two excellent IDE options are available for Rust development: [rust-analyzer](https://rust-analyzer.github.io/) and [RustRover](https://www.jetbrains.com/rust/). Choose based on your preferences - both provide comprehensive Rust support. I primarily use Neovim with rust-analyzer, but also keep RustRover installed for advanced refactoring and debugging tasks.

#### rust-analyzer

rust-analyzer implements the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) for Rust, making it compatible with any LSP-supporting editor. Popular choices include:

- [Visual Studio Code](https://code.visualstudio.com/)
- [Neovim](https://neovim.io/)
- [Helix](https://helix-editor.com/)

Find detailed configuration options [here](https://rust-analyzer.github.io/book/configuration.html). Here's my personal rust-analyzer configuration for Neovim:

```json
cargo = {
  allFeatures = true,
  loadOutDirsFromCheck = true,
  buildScripts = {
    enable = true,
  },
},
check = {
  command = "clippy",
  extraArgs = { "--no-deps" },
},
checkOnSave = true,
diagnostics = {
  enable = true,
},
procMacro = {
  enable = true,
  ignored = {
    ["async-trait"] = { "async_trait" },
    ["napi-derive"] = { "napi" },
    ["async-recursion"] = { "async_recursion" },
  },
},
imports = {
  preferPrelude = true,
  granularity = {
    group = "item",
    enforce = true,
  },
  prefix = "crate",
},
```

#### RustRover

RustRover is JetBrains' dedicated Rust IDE, perfect if you're already comfortable with their ecosystem. It's currently free for non-commercial use and excels at advanced refactoring operations like code extraction, moving modules, and intelligent renaming.

## Wrapping Up

You should now have Rust installed, a configured editor, and essential tooling ready. The Rust ecosystem offers many excellent third-party tools - explore [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) to discover useful additions to your workflow.
