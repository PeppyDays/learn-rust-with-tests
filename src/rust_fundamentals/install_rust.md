# Install Rust

## Install rustup and Toolchains

The official installation instructions for Rust are available [here](https://www.rust-lang.org/tools/install). After installing `rustup`, make sure to update it to the latest version and nightly toolchains:

```bash
rustup update stable
rustup toolchain install nightly
```

## Refactoring and Tooling

### Refactoring

A big emphasis of this book is the importance of refactoring. Your tools can help you do bigger refactoring with confidence. You should be familiar enough with your editor to perform the following with a simple key combination:

- Extract/Inline variable
  - Taking magic values and giving them a name lets you simplify your code quickly
- Extract method/function
  - It is vital to be able to take a section of code and extract functions/methods
- Rename symbols
  - You should be able to rename symbols across files confidently
- Format code
  - Rust has a formatter called [rustfmt](https://github.com/rust-lang/rustfmt) that will format your code to a guided style
  - Your editor should run this on every file saved
- Lint code
  - Rust has a linter called [clippy](https://github.com/rust-lang/rust-clippy) that will help you find common mistakes in your code
  - Your editor should run this on every file saved
- Run tests
  - You should be able to do any of the above and then quickly re-run your tests to ensure your refactoring hasn't broken anything

In addition, to help you work with your code, you should be able to:

- View function signature
  - You should never be unsure how to call a function in Rust
  - Your IDE should describe a function in terms of its documentation, its parameters and what it returns
- View function definition
  - If it's still unclear what a function does, you should be able to jump to the source code and try and figure it out yourself
- Find usages of a symbol
  - Understanding a function's context can help you make decisions when refactoring

Mastering your tools will help you concentrate on the code and reduce context switching.

### Tooling

There are two options available for Rust IDE: [rust-analyzer](https://rust-analyzer.github.io/) and [RustRover](https://www.jetbrains.com/rust/). You can choose one of them based on your preference. Both of them are great tools for Rust development. I'm using Neovim with rust-analyzer, but I also have a RustRover installed. I use it for some of the more advanced refactoring features and debugging. You can use either of them, or both, depending on your needs.

#### rust-analyzer

rust-analyzer is an implementation of the [language server protocol](https://microsoft.github.io/language-server-protocol/) for Rust. Any editors that support the language server protocol can use rust-analyzer. The most popular editors are:

- [Visual Studio Code](https://code.visualstudio.com/)
- [Neovim](https://neovim.io/)
- [Helix](https://helix-editor.com/)

You can find all the detailed configuration for rust-analyzer [here](https://rust-analyzer.github.io/book/configuration.html), and this is personal rust-analyzer configuration in Neovim:

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

RustRover is a JetBrains IDE for Rust. It is a great option if you are already familiar with JetBrains IDEs. At the time of writing, it is free for non-commercial use. Also, it supports great features for refactoring, such as extractions, moving and renaming code, and more.

## Wrapping up

At this point, you should have Rust installed, an editor available, and some basic tooling in place. Rust has a very large ecosystem of third-party products. If you find some useful components, see [Awesome Rust](https://github.com/rust-unofficial/awesome-rust).
