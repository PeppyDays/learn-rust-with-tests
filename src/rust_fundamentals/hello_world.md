# Hello, World

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/hello).

## How to Start

It is traditional for your first program in a new language to be [Hello, World](https://en.m.wikipedia.org/wiki/%22Hello,_World!%22_program). To start with that, we will create a new Rust project. You can do this by running the following command in your terminal anywhere you like:

```bash
cargo new hello
```

After that, navigate to the `hello/src` directory, and add `main.rs` (might already exist) and `lib.rs` files. We will add our code in these files. The `lib.rs` file is where you will write your core logic, and the `main.rs` file is where you will write simple code to run.

You can copy the following code into `lib.rs`:

```rust
pub fn greet() {
    println!("Hello, World!")
}
```

You can copy the following code into `main.rs`:

```rust,ignore
use hello::greet;

fn main() {
    greet();
}
```

To run it, type `cargo run` in the terminal. You should see the output:

```bash
> cargo run
   Compiling hello v0.1.0 (/Users/arine/../learn-rust-with-tests/examples/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/hello`
Hello, World!
```

## How It Works

The `lib.rs` file contains the core logic of your program. The `greet` function returns a string that says "Hello, World!". In Rust, the default visibility of functions is private, so we need to add the `pub` keyword to make it public. This allows us to use it in other files, like `main.rs`.

The `main.rs` file is where you run the program. It imports the `greet` function from the `lib.rs` file. The import statement is `use hello::greet;`. Where the `hello` is come from? We ran `cargo new hello` so that the library package name is `hello` as defined in `Cargo.toml`. Now it calls the `greet` function and prints greeting to the console.

## How to Test

### Refactoring

How do you test this? It is good to separate your domain code from the outside world. The `println!` macro is a side effect that prints to the standard out, and the string we send in is our domain.

So let's separate these concerns for easier test:

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

We have created a new function, but this time, we have added another keyword `String` to the definition. This means that the function returns a string.

### The First Test

Now add test cases for our `greet` function under the `greet` function in `lib.rs`:

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

To test it, you can simply type `cargo test`. You should see the output:

```bash
> cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/hello-b9ee1e14d7090257)

running 1 test
test specs_for_greet::sut_returns_hello_world_correctly ... ok
```

Just to check, try deliberately breaking the test by changing the expected value to something else, like `Hello, World!` to `Hello, stanger`.

Notice how you have not had to pick between multiple testing frameworks and then figure out how to install them. Everything you need is built into the language, and the syntax is the same as the rest of the code you will write.

### Writing Unit Tests

Writing a test is just like writing a function, with a few rules:

- It needs to be in a module marked with `#[cfg(test)]`
- The test function must be marked with `#[test]`

If you are the first with Rust, you might be surprised to see the tests in the same file as the code. This is a common practice in Rust, and usually unit tests are written in this manner. The `#[cfg(test)]` attribute tells the compiler to only compile this code when you run `cargo test`, which means test codes will not be included in the final binary. You can find more information about this in the [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-03-test-organization.html).

The test module name is `specs_for_greet`, which means the specifications we want to implement for the `greet` function. The test function name is `sut_returns_hello_world_correctly`. The `sut` stands for [system under test](http://xunitpatterns.com/SUT.html), which is `greet` function in this case. There could be many conventions for naming test functions, but the most important thing is to be consistent. I prefer to use `specs_for_<name of sut>` for the module name containing test functions of the system under test, and `sut_<description of specification>` for the test function name. This way, it is easy to find the test functions for a specific system under test.

Inside the test function, we will follow triple-A pattern:

- Arrange
  - Prepare the test data or dependencies
  - Put the system into the required state for testing
- Act
  - Call the system under test and get the actual result
- Assert
  - Verify the actual result with the expected result

The `assert_eq!` macro is used to compare the expected and actual values. If they are not equal, the test will fail, and you will see an error message in the terminal.

### Hello, YOU

Now that we have a test, we can iterate on our software safely.

In the last example, we wrote the test after the code had been written so that you could get an example of how to write a test and declare a function. From this point on, we will be writing tests first.

Our next requirement is to let us specify the recipient of the greeting.

Let's start by capturing these requirements in a test. This is basic test-driven development and allows us to make sure our test is actually testing what we want. When you retrospectively write tests, there is the risk that your test may continue to pass even if the code doesn't work as intended.

We will replace the test we wrote to the new test in `specs_for_greet` module:

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

Now run `cargo test`, you should have a compilation error:

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

When using a statically typed language like Rust, it is important to listen to the compiler. The compiler understands how your code should snap together and work so you don't have to.

In this case the compiler is telling you what you need to do to continue. We have to change our `greet` function to accept an argument.

Edit the `greet` function to accept an argument of type `&str`:

```rust
pub fn greet(name: &str) -> String {
    String::from("Hello, World!")
}
```

If you try and run your tests, it fails to compile because you're not passing an argument in `main.rs`. Send in "world" to make it compile.

```rust,ignore
fn main() {
    println!("{}", greet("world"));
}
```

Now when you run your tests, you should see something like:

```plain
assertion `left == right` failed
  left: "Hello, Chris!"
 right: "Hello, World"
```

We finally have a compiling program but it is not meeting our requirements according to the test. Let's make the test pass by using the name argument and concatenate it with `Hello,`:

```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

When you run the tests, they should now pass. Normally, as part of the TDD cycle, we should now refactor.

### Note on Source Control

At this point, if you are using source control (which you should!) I would commit the code as it is. We have working software backed by a test.

I wouldn't push to main though, because I plan to refactor next. It is nice to commit at this point in case you somehow get into a mess with refactoring - you can always go back to the working version.

There's not a lot to refactor here, but we can introduce another language feature, constants.

### Constants

Constants are defines like so.

```rust
const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";
```

We can now refactor our code.

```rust
const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";

pub fn greet(name: &str) -> String {
    format!("{}{}!", GREETING_PREFIX_FOR_ENGLISH, name)
}
```

After refactoring, re-run your tests to make sure you haven't broken anything. It's worth thinking about creating constants to capture the meaning of values and sometimes to aid performance.

### Hello, World .. Again

The next requirement is when our function is called with an empty string it defaults to printing "Hello, World", rather than "Hello, ".

Start by writing a new failing test.

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

While we have a failing test, let's fix the code, using an if.

```rust
const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";

pub fn greet(name: &str) -> String {
    if name == "" {
        return format!("{}World!", GREETING_PREFIX_FOR_ENGLISH);
    }
    format!("{}{}!", GREETING_PREFIX_FOR_ENGLISH, name)
}
```

If we run our tests, we should see it satisfies the new requirement and haven't accidentally broken the other functionality.

Now we have a working program, we can refactor again. If you set the lint `clippy` well, you already see warning message as following under `if name == ""`.

```plain
1. comparison to empty slice
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#comparison_to_empty
`#[warn(clippy::comparison_to_empty)]` on by default [comparison_to_empty]
2. using `is_empty` is clearer and more explicit: `name.is_empty()` [comparison_to_empty]
```

We can use the `is_empty` method, and do better by shadowing the `name` variable.

```rust,ignore
pub fn greet(name: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    format!("{}{}!", GREETING_PREFIX_FOR_ENGLISH, name)
}
```

It is important that your tests are clear specifications of what the code needs to do. Refactoring is not just for the production code! Refactoring your tests is just as important. The tests for this example are very simple and no need to refactor, but as your tests get more complex, you will need to refactor them too. We will see more examples of this in the next chapters.

### Back to Source Control

Now that we are happy with the code, I would amend the previous commit so that we only check in the lovely version of our code with its test.

### Discipline

Let's go over the cycle again

- Write a test
- Make the compiler pass
- Run the test, see that it fails and check the error message is meaningful
- Write enough code to make the test pass
- Refactor

On the face of it this may seem tedious but sticking to the feedback loop is important. Not only does it ensure that you have relevant tests, it helps ensure you design good software by refactoring with the safety of tests.

Seeing the test fail is an important check because it also lets you see what the error message looks like. As a developer it can be very hard to work with a codebase when failing tests do not give a clear idea as to what the problem is.

By ensuring your tests are fast and setting up your tools so that running tests is simple you can get in to a state of flow when writing your code.

By not writing tests, you are committing to manually checking your code by running your software, which breaks your state of flow. You won't be saving yourself any time, especially in the long run.

## Keep Going with More Requirements

Goodness me, we have more requirements. We now need to support a second parameter, specifying the language of the greeting. If a language is passed in that we do not recognise, just default to English.

We should be confident that we can easily use TDD to flesh out this functionality! Write a test for a user passing in Spanish. Add it to the existing suite.

### Spanish

Write a test for a user passing in Spanish. Add it to the existing test suite.

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

You might feel uncomfortable about the fact that you are not passing the second argument to `greet` function. But this is a good thing! It means you are following the TDD cycle and not trying to guess what the code should look like. You are letting the tests guide you.

So, do not cheat! Test first. When you try to run the test, the compiler should complain because you are calling `greet` function with two arguments rather than one.

```bash
error[E0061]: this function takes 1 argument but 2 arguments were supplied
  --> src/lib.rs:28:22
   |
28 |         let actual = greet(name, language);
   |                      ^^^^^       -------- unexpected argument #2 of type `&str`
```

Fix the compilation problems by adding another string argument to `greet` function.

```rust,ignore
pub fn greet(name: &str, language: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    format!("{}{}!", GREETING_PREFIX_FOR_ENGLISH, name)
}
```

When you try and run the test again it will complain about not passing through enough arguments to `greet` function in your other tests and in `main.rs`. Fix the tests and `main.rs` to pass the second argument as an empty string.

```bash
error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> src/lib.rs:24:22
   |
24 |         let actual = greet("Chris");
   |                      ^^^^^------ argument #2 of type `&str` is missing
```

Fix them by passing through empty strings. Now all your tests should compile and pass, apart from our new scenario.

```bash
thread 'specs_for_greet::sut_returns_hola_with_given_name_if_language_is_spanish' panicked at src/lib.rs:60:9:
assertion `left == right` failed
  left: "Hola, Elodie!"
 right: "Hello, Elodie!"
```

We can use `if` here to check the language is equal to "Spanish" as we did with `name`.

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

The tests should now pass.

Now it is time to refactor. You should see some problems in the code, "magic" strings, some of which are repeated. Try and refactor it yourself, with every change make sure you re-run the tests to make sure your refactoring isn't breaking anything.

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

You should refactor tests as well because the name and intention of tests are not clear after introducing the language. You can change the test names to be more descriptive.

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

You might wonder why we only tests for empty language to check the default name "World". Why do we skip tests for other languages?

You're correct. We should also test the default name specification with all languages, and we will learn parameterised tests in the following chapter to do that much better. For now, we leave it as is.

### French

- Write a test asserting that if you pass in "French" you get "Bonjour, "
- See it fail, check the error message is easy to read
- Do the smallest reasonable change in the code

You may have written something that looks roughly like this.

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

When you have lots of if statements checking a particular value it is common to use a pattern matching instead. We can use `match` to refactor the code to make it easier to read and more extensible if we wish to add more language support later.

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

Write a test to now include a greeting in the language of your choice and you should see how simple it is to extend our amazing function.

### Functionality Extraction

You could argue that maybe our function is getting a little big. The simplest refactor for this would be to extract out some functionality into another function.

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

Who knew you could get so much out of `Hello, World`? By now you should have some understanding of the following.

### Some of Rust's Syntax

- Writing tests
- Declaring functions with arguments and return types
- Using `const`, `if` and `match` statements
- Declaring variables and constants

### The TDD Process

- Write a failing test and see it fail so we know we have written a relevant test for our requirements and seen that it produces an easy to understand description of the failure
- Writing the smallest amount of code to make it pass so we know we have working software
- Then refactor, backed with the safety of our tests to ensure we have well-crafted code that is easy to work with

In our case, we've gone from `greet()` to `greet("name")` and then to `greet("name", "French")` in small, easy-to-understand steps.

Of course, this is trivial compared to "real-world" software, but the principles still stand. TDD is a skill that needs practice to develop, but by breaking problems down into smaller components that you can test, you will have a much easier time writing software.
