# Integers

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/integers).

Integers work as you'd expect. Let's write an `add` function to explore the basics. Create a new project with `cargo new integers`.

## Write the Test First

Create `src/lib.rs` and add the following test for our `add` function:

```rust
#[cfg(test)]
mod specs_for_add {
    #[test]
    fn sut_returns_4_if_arguments_are_2_and_2() {
        // Arrange
        let x = 2;
        let y = 2;

        // Act
        let actual = add(x, y);

        // Assert
        let expected = 4;
        assert_eq!(expected, actual);
    }
}
```

## Try and Run the Test

Run `cargo test` and examine the compilation error:

```bash
error[E0425]: cannot find function `add` in this scope
  --> src/lib.rs:10:22
   |
10 |         let actual = add(x, y);
   |                      ^^^ not found in this scope
```

## Write the Minimal Amount of Code

Write just enough code to satisfy the compiler - we want to verify our test fails for the right reason:

```rust
pub fn add(x: i64, y: i64) -> i64 {
    0
}
```

Run the tests again. The test should correctly report the failure:

```bash
thread 'specs_for_add::sut_returns_4_if_arguments_are_2_and_2' panicked at src/lib.rs:20:9:
assertion `left == right` failed
  left: 4
 right: 0
```

## Write Enough Code to Make It Pass

In strict TDD, we'd write the minimal code to pass. A pedantic programmer might try:

```rust
pub fn add(x: i64, y: i64) -> i64 {
    4
}
```

Ah! Foiled again - is TDD a sham?

We could write another test with different numbers, but that becomes a game of [cat and mouse](https://en.m.wikipedia.org/wiki/Cat_and_mouse). Some call this [EDFH](https://fsharpforfunandprofit.com/posts/return-of-the-edfh/).

Later, when we're more familiar with Rust syntax, I'll introduce "Property Based Testing" - a technique that helps find bugs and prevents such developer games.

For now, let's implement it properly:

```rust
pub fn add(x: i64, y: i64) -> i64 {
    x + y
}
```

Re-run the tests - they should pass.

## Refactor

There's not much to improve in the actual code here.

You can add documentation comments using `///` above function signatures. These appear in Rust documentation, just like the standard library docs:

```rust
/// `add` takes two integers and returns the sum of them.
pub fn add(x: i64, y: i64) -> i64 {
    x + y
}
```

## Documentation Tests

For extra thoroughness, you can create [documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html). You'll find many examples in the standard library documentation.

Code examples outside the codebase (like README files) often become outdated because they're not validated against actual code changes.

Documentation tests run with your regular tests. Since the Rust compiler validates these examples, you can trust that your documentation always reflects current code behavior.

Add documentation tests by including code blocks in documentation comments, surrounded by triple backticks:

````rust
/// `add` takes two integers and returns the sum of them.
///
/// ```
/// use integers::add;
///
/// let sum = add(1, 5);
/// assert_eq!(6, sum);
/// ```
pub fn add(x: i64, y: i64) -> i64 {
    x + y
}
````

This code appears in the generated documentation and runs as part of your test suite. If you change the example code incorrectly, the test will fail.

Running tests shows the documentation example executes automatically:

```bash
   Doc-tests integers

running 1 test
test src/lib.rs - add (line 6) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Documentation tests are particularly useful for demonstrating code that can't run as unit tests (like network access) while guaranteeing the examples at least compile.

View your documentation with `cargo doc --open`. This opens a browser window with your project's documentation, similar to viewing standard library documentation.

## Wrapping Up

What we've covered:
- More TDD workflow practice
- Integer arithmetic operations
- Writing clear documentation for code users
- Creating testable examples that stay current with your code