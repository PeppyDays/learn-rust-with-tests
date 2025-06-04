# Hello, World

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/hello).

## How to Start

It's traditional for your first program in a new language to be [Hello, World](https://en.m.wikipedia.org/wiki/%22Hello,_World!%22_program). Let's start by creating a new Rust project. Run this command in your preferred directory:

```bash
cargo new hello
```

Navigate to `hello/src` and create `main.rs` (if it doesn't exist) and `lib.rs`. We'll add our code to these files. The `lib.rs` contains your core logic, while `main.rs` handles program execution.

Copy this code into `lib.rs`:

```rust
pub fn greet() {
    println!("Hello, World!")
}
```

Copy this code into `main.rs`:

```rust,ignore
use hello::greet;

fn main() {
    greet();
}
```

Run it with `cargo run`. You should see:

```bash
> cargo run
   Compiling hello v0.1.0 (/Users/arine/../learn-rust-with-tests/examples/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/hello`
Hello, World!
```

## How It Works

The `lib.rs` file contains your program's core logic. The `greet` function prints "Hello, World!". In Rust, functions are private by default, so we use the `pub` keyword to make it public for use in other files like `main.rs`.

The `main.rs` file runs the program. It imports the `greet` function using `use hello::greet;`. Where does `hello` come from? When we ran `cargo new hello`, it set the library package name to `hello` in `Cargo.toml`. The function then prints the greeting to the console.

## How to Test

### Refactoring

How do you test this? It's good practice to separate domain code from external dependencies. The `println!` macro is a side effect that prints to stdout, while the string we send is our domain logic.

Let's separate these concerns for easier testing:

```rust
pub fn greet() -> String {
    String::from("Hello, World!")
}
```

```rust,ignore
fn main() {
    println!("{}", greet());
}
```

We've modified our function to return a `String` instead of printing directly. This makes it testable.

### The First Test

Add test cases for our `greet` function in `lib.rs`:

```rust
#[cfg(test)]
mod specs_for_greet {
    use super::greet;

    #[test]
    fn sut_returns_hello_world_correctly() {
        // Act
        let actual = greet();

        // Assert
        let expected = "Hello, World!";
        assert_eq!(expected, actual);
    }
}
```

Test it with `cargo test`:

```bash
> cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/hello-b9ee1e14d7090257)

running 1 test
test specs_for_greet::sut_returns_hello_world_correctly ... ok
```

Try deliberately breaking the test by changing the expected value to something else, like "Hello, stranger".

Notice how you don't need to choose between testing frameworks or figure out installation. Everything you need is built into the language with consistent syntax.

### Writing Unit Tests

Writing tests follows simple rules:
- Tests go in modules marked with `#[cfg(test)]`
- Test functions must be marked with `#[test]`

If you're new to Rust, you might be surprised to see tests in the same file as the code. This is common practice in Rust for unit tests. The `#[cfg(test)]` attribute tells the compiler to only compile this code during `cargo test`, excluding it from the final binary. Learn more in [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-03-test-organization.html).

The test module name `specs_for_greet` indicates specifications for the `greet` function. The test function name `sut_returns_hello_world_correctly` uses `sut` for [system under test](http://xunitpatterns.com/SUT.html). There are many naming conventions, but consistency matters most. I prefer `specs_for_<name of sut>` for modules and `sut_<description of specification>` for test functions. This makes finding tests for specific functionality easy.

Inside test functions, follow the triple-A pattern:
- **Arrange**: Prepare test data or dependencies, set required system state
- **Act**: Call the system under test and capture the actual result
- **Assert**: Verify the actual result matches expectations

The `assert_eq!` macro compares expected and actual values. When they differ, the test fails with an error message.

### Hello, YOU

Now that we have a test, we can iterate safely on our software.

In the previous example, we wrote the test after the code to demonstrate testing and function declaration. From now on, we'll write tests first.

Our next requirement: let users specify the greeting recipient.

Let's capture this requirement in a test. This is basic test-driven development, ensuring our test actually tests what we want. Writing tests retrospectively risks tests that pass even when code doesn't work as intended.

Replace our existing test in the `specs_for_greet` module:

```rust
#[test]
fn sut_returns_hello_with_given_name_correctly() {
    // Act
    let actual = greet("Chris");

    // Assert
    let expected = "Hello, Chris!";
    assert_eq!(expected, actual);
}
```

Run `cargo test` - you should get a compilation error:

```bash
> cargo test
   Compiling hello v0.1.0 (/Users/arine/../learn-rust-with-tests/examples/hello)
error[E0061]: this function takes 0 arguments but 1 argument was supplied
  --> src/lib.rs:16:22
   |
16 |         let actual = greet("Chris");
   |                      ^^^^^ ------- unexpected argument of type `&'static str`
   |
note: function defined here
  --> src/lib.rs:5:8
   |
5  | pub fn greet() -> String {
   |        ^^^^^
help: remove the extra argument
   |
16 -         let actual = greet("Chris");
16 +         let actual = greet();
   |

For more information about this error, try `rustc --explain E0061`.
error: could not compile `hello` (lib test) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

In statically typed languages like Rust, listen to the compiler. It understands how your code should work together.

The compiler tells us exactly what to do: change our `greet` function to accept an argument.

Edit the `greet` function to accept a `&str` argument:

```rust
pub fn greet(name: &str) -> String {
    String::from("Hello, World!")
}
```

Running tests now fails to compile because `main.rs` doesn't pass an argument. Add "world" to make it compile:

```rust,ignore
fn main() {
    println!("{}", greet("world"));
}
```

Now when you run tests, you should see:

```plain
assertion `left == right` failed
  left: "Hello, Chris!"
 right: "Hello, World"
```

We have a compiling program that doesn't meet our requirements. Let's make the test pass by using the name argument:

```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

Tests should now pass. As part of the TDD cycle, we should refactor.

### Note on Source Control

If you're using source control (which you should!), commit your code now. We have working software backed by tests.

Don't push to main yet since we plan to refactor. It's good to commit at this point - if refactoring goes wrong, you can always return to the working version.

There's not much to refactor here, but we can introduce constants.

### Constants

Constants are defined like this:

```rust
const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";
```

Now we can refactor our code:

```rust
const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";

pub fn greet(name: &str) -> String {
    format!("{}{}!", GREETING_PREFIX_FOR_ENGLISH, name)
}
```

After refactoring, re-run tests to ensure nothing broke. Consider creating constants to capture value meanings and sometimes aid performance.

### Hello, World .. Again

The next requirement: when called with an empty string, default to printing "Hello, World" rather than "Hello, ".

Start with a new failing test:

```rust
#[test]
fn sut_returns_hello_world_if_empty_name_is_given() {
    // Act
    let actual = greet("");

    // Assert
    let expected = "Hello, World!";
    assert_eq!(expected, actual);
}
```

With a failing test, let's fix the code using an `if` statement:

```rust
const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";

pub fn greet(name: &str) -> String {
    if name == "" {
        return format!("{}World!", GREETING_PREFIX_FOR_ENGLISH);
    }
    format!("{}{}!", GREETING_PREFIX_FOR_ENGLISH, name)
}
```

Running tests should satisfy the new requirement without breaking existing functionality.

Now we can refactor again. If you have `clippy` configured well, you'll see a warning under `if name == ""`:

```plain
1. comparison to empty slice
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#comparison_to_empty
`#[warn(clippy::comparison_to_empty)]` on by default [comparison_to_empty]
2. using `is_empty` is clearer and more explicit: `name.is_empty()` [comparison_to_empty]
```

We can use the `is_empty` method and improve by shadowing the `name` variable:

```rust,ignore
pub fn greet(name: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    format!("{}{}!", GREETING_PREFIX_FOR_ENGLISH, name)
}
```

Clear test specifications are crucial. Refactoring applies to tests too! While our simple tests don't need refactoring yet, as tests grow complex, refactoring becomes essential.

### Back to Source Control

Now that we're happy with the code, amend the previous commit so we only check in the polished version with its test.

### Discipline

Let's review the cycle:
- Write a test
- Make the compiler pass
- Run the test, see it fail, and check the error message is meaningful
- Write enough code to make the test pass
- Refactor

This may seem tedious, but sticking to the feedback loop is crucial. It ensures relevant tests and helps design good software through safe refactoring.

Seeing tests fail is important - it shows you what error messages look like. Working with codebases where failing tests don't clearly indicate problems is very difficult.

Fast tests and simple test-running tools create flow state when coding. Without tests, you're committed to manual verification by running software, which breaks flow state. You won't save time, especially long-term.

## Keep Going with More Requirements

We have more requirements! We need to support a second parameter specifying the greeting language. If we don't recognize a language, default to English.

We should confidently use TDD to build this functionality!

### Spanish

Write a test for Spanish. Add it to the existing test suite:

```rust,ignore
#[test]
fn sut_returns_hola_if_language_is_spanish() {
    // Arrange
    let name = "Elodie";
    let language = "Spanish";

    // Act
    let actual = greet(name, language);

    // Assert
    let expected = "Hola, Elodie!";
    assert_eq!(expected, actual);
}
```

You might feel uncomfortable not passing the second argument to `greet` yet. But this is good! It means you're following TDD and not guessing what the code should look like. Let tests guide you.

Don't cheat! Test first. The compiler should complain about calling `greet` with two arguments instead of one:

```bash
error[E0061]: this function takes 1 argument but 2 arguments were supplied
  --> src/lib.rs:28:22
   |
28 |         let actual = greet(name, language);
   |                      ^^^^^       -------- unexpected argument #2 of type `&str`
```

Fix compilation by adding another string argument to `greet`:

```rust,ignore
pub fn greet(name: &str, language: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    format!("{}{}!", GREETING_PREFIX_FOR_ENGLISH, name)
}
```

Running tests again will complain about insufficient arguments to `greet` in other tests and `main.rs`. Fix them by passing empty strings as the second argument:

```bash
error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> src/lib.rs:24:22
   |
24 |         let actual = greet("Chris");
   |                      ^^^^^------ argument #2 of type `&str` is missing
```

All tests should compile and pass except our new scenario:

```bash
thread 'specs_for_greet::sut_returns_hola_with_given_name_if_language_is_spanish' panicked at src/lib.rs:60:9:
assertion `left == right` failed
  left: "Hola, Elodie!"
 right: "Hello, Elodie!"
```

Use `if` to check if the language equals "Spanish":

```rust,ignore
pub fn greet(name: &str, language: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    let prefix = if language == "Spanish" {
        "Hola, "
    } else {
        GREETING_PREFIX_FOR_ENGLISH
    };
    format!("{}{}!", prefix, name)
}
```

Tests should now pass.

Time to refactor. You should see problems: "magic" strings, some repeated. Try refactoring yourself, re-running tests with every change to ensure nothing breaks:

```rust
const SPANISH: &str = "Spanish";
const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";
const GREETING_PREFIX_FOR_SPANISH: &str = "Hola, ";

pub fn greet(name: &str, language: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    let prefix = if language == SPANISH {
        GREETING_PREFIX_FOR_SPANISH
    } else {
        GREETING_PREFIX_FOR_ENGLISH
    };
    format!("{}{}!", prefix, name)
}
```

Refactor tests too - their names and intentions aren't clear after introducing language. Change test names to be more descriptive:

```rust
#[test]
fn sut_returns_hello_in_english_if_language_is_empty() {
    // Arrange
    let name = "Chris";
    let language = "";

    // Act
    let actual = greet(name, language);

    // Assert
    let expected = "Hello, Chris!";
    assert_eq!(expected, actual);
}

#[test]
fn sut_returns_world_as_default_name_if_name_is_empty() {
    // Arrange
    let name = "";
    let language = "";

    // Act
    let actual = greet(name, language);

    // Assert
    let expected = "Hello, World!";
    assert_eq!(expected, actual);
}
```

You might wonder why we only test empty language for the default name "World" without testing other languages. You're right - we should test the default name specification with all languages. We'll learn parameterized tests in following chapters for better handling. For now, we'll leave it as is.

### French

- Write a test asserting that passing "French" gets "Bonjour, "
- See it fail, check the error message is readable
- Make the smallest reasonable change

You might write something like this:

```rust
const SPANISH: &str = "Spanish";
const FRENCH: &str = "French";
const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";
const GREETING_PREFIX_FOR_SPANISH: &str = "Hola, ";
const GREETING_PREFIX_FOR_FRENCH: &str = "Bonjour, ";

pub fn greet(name: &str, language: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    let prefix = if language == SPANISH {
        GREETING_PREFIX_FOR_SPANISH
    } else if language == FRENCH {
        GREETING_PREFIX_FOR_FRENCH
    } else {
        GREETING_PREFIX_FOR_ENGLISH
    };
    format!("{}{}!", prefix, name)
}
```

## Two Last Refactoring

### `match` Statement

When you have many `if` statements checking particular values, pattern matching is common. We can use `match` to make code more readable and extensible for adding language support:

```rust,ignore
pub fn greet(name: &str, language: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    let prefix = match language {
        SPANISH => GREETING_PREFIX_FOR_SPANISH,
        FRENCH => GREETING_PREFIX_FOR_FRENCH,
        _ => GREETING_PREFIX_FOR_ENGLISH,
    };
    format!("{}{}!", prefix, name)
}
```

Write a test for a greeting in your chosen language and see how simple it is to extend our function.

### Functionality Extraction

You could argue our function is getting large. The simplest refactor would be extracting functionality into another function:

```rust,ignore
pub fn greet(name: &str, language: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    let prefix = determine_greeting_prefix(language);
    format!("{}{}!", prefix, name)
}

fn determine_greeting_prefix(language: &str) -> &str {
    match language {
        SPANISH => GREETING_PREFIX_FOR_SPANISH,
        FRENCH => GREETING_PREFIX_FOR_FRENCH,
        _ => GREETING_PREFIX_FOR_ENGLISH,
    }
}
```

## Wrapping Up

Who knew you could get so much from `Hello, World`? You should now understand:

### Some Rust Syntax
- Writing tests
- Declaring functions with arguments and return types
- Using `const`, `if`, and `match` statements
- Declaring variables and constants

### The TDD Process
- Write a failing test and see it fail so we know we've written a relevant test for our requirements and seen that it produces an understandable failure description
- Write the smallest amount of code to make it pass so we know we have working software
- Refactor, backed by test safety to ensure well-crafted, easy-to-work-with code

We've gone from `greet()` to `greet("name")` to `greet("name", "French")` in small, understandable steps.

This is trivial compared to "real-world" software, but the principles stand. TDD is a skill requiring practice, but breaking problems into smaller, testable components makes writing software much easier.