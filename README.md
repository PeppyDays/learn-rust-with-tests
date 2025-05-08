# Learn Rust with Tests

I'm writing Rust version of [Learn Go with Tests](https://github.com/quii/learn-go-with-tests?tab=readme-ov-file). You can check this book [here](https://peppydays.github.io/learn-rust-with-tests/).

## Setup

### Task

We use [Taskfile](https://taskfile.dev/) to standardise commands related to this project. All the details are listed in the configuration file.

```bash
brew install go-task/tap/go-task
```

### mdBook

We use [mdBook](https://github.com/rust-lang/mdBook) to generate the documentation. You can install it with:

```bash
brew install mdbook
```

### Conventional Commit

To manage commit messages, you need to install pre-commit and commitlint to enforce message format. Commit message should follow convensional commit. You can modify the configuration file to adjust the specific constraints for commit message by referencing the rules.

```bash
brew install pre-commit
brew install npm
npm install

pre-commit install
pre-commit install --hook-type commit-msg
```

## To Do

- [ ] Add English grammar and fluency check in CI
- [ ] Finish prefix first
- [ ] Finish Rust Fundamentals
