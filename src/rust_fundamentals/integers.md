# Integers

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/integers).

Integers work as you would expect. Let's write an `add` function to try things out. Create a new project where you want to practice with `cargo new integers`.

## Write the Test First

Create a new file `src/lib.rs` and add the following code to test the `add` function:

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

Run the test with the command `cargo test` and inspect the compilation error.

```bash
error[E0425]: cannot find function `add` in this scope
  --> src/lib.rs:10:22
   |
10 |         let actual = add(x, y);
   |                      ^^^ not found in this scope
```

## Write the Minimal Amount of Code

Write enough code to satisfy the compiler and that's all - remember we want to check that our tests fail for the correct reason.

```rust
pub fn add(x: i64, y: i64) -> i64 {
    0
}
```

Now run the tests, and we should be happy that the test is correctly reporting what is wrong.

```bash
thread 'specs_for_add::sut_returns_4_if_arguments_are_2_and_2' panicked at src/lib.rs:20:9:
assertion `left == right` failed
  left: 4
 right: 0
```

## Write Enough Code to Make It Pass

In the strictest sense of TDD, we should now write the minimal amount of code to make the test pass. A pedantic programmer may do this:

```rust
pub fn add(x: i64, y: i64) -> i64 {
    4
}
```

Ah hah! Foiled again, TDD is a sham right?

We could write another test, with some different numbers to force that test to fail but that feels like a game of [cat and mouse](https://en.m.wikipedia.org/wiki/Cat_and_mouse). Some call this [EDFH](https://fsharpforfunandprofit.com/posts/return-of-the-edfh/).

Once we're more familiar with Rust's syntax, I will introduce a technique called "Property Based Testing", which would stop annoying developers and help you find bugs.

For now, let's fix it properly

```rust
pub fn add(x: i64, y: i64) -> i64 {
    x + y
}
```

If you re-run the tests, they should pass.

## Refactor

There's not a lot in the actual code we can really improve on here.

You can add documentation comments to functions prefixed with `///` above the function signature, and these will appear in Rust documents just like when you look at the standard library's documentation. We will see the documentation in a moment.

```rust
/// `add` takes two integers and returns the sum of them.
pub fn add(x: i64, y: i64) -> i64 {
    x + y
}
```

## Documentation Tests

If you really want to go the extra mile, you can make [documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html). You will find many examples in the standard library documentation.

Often code examples that can be found outside the codebase, such as a readme file, become out of date and incorrect compared to the actual code because they don't get checked.

Documentation tests are run whenever tests are executed. Because such examples are validated by the Rust compiler, you can be confident your documentation's examples always reflect current code behavior.

You can add documentation tests by adding a code block to the documentation comments. The code block should start and finish with ```. Here is an example:

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

Adding this code will cause the example to appear in the documentation. The example will also be run as part of the tests, so if you change the code in the example, it will fail.

Running tests, we can see the documentation function is executed with no further arrangement from us.

```bash
   Doc-tests integers

running 1 test
test src/lib.rs - add (line 6) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Documentation tests are useful for demonstrating code that cannot run as unit tests, such as that which accesses the network, while guaranteeing the example at least compiles.

To view the documentation, run the command `cargo doc --open`. This will open a browser window with the documentation for your project. You can also view the documentation for the standard library in the same way.

## Wrapping Up

What we have covered:

- More practice of the TDD workflow
- Integers and those addition
- Writing better documentation so users of our code can understand its usage quickly
- Examples of how to use our code, which are checked as part of our tests
