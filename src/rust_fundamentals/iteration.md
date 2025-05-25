# Iteration

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/iteration).

To do stuff repeatedly, you needs `loop`, `while` and `for`. The `loop` keyword creates an infinite loop, which you can break out of with the `break` keyword. The `while` loop is a conditional loop that continues until the condition is false. The `for` loop iterates over a collection, such as an array or a vector.

Let's write a test for a function that repeats a character 10 times.

There's nothing new so far, so try and write it yourself for practice.

## Write the Test First

```rust
#[cfg(test)]
mod specs_for_repeat {
    #[test]
    fn sut_repeats_given_argument_10_times() {
        // Arrange
        let input = "a";

        // Act
        let actual = repeat(input);

        // Assert
        let expected = "aaaaaaaaaa";
        assert_eq!(expected, actual);
    }
}
```

## Try and Run the Test

```bash
error[E0425]: cannot find function `repeat` in this scope
  --> src/lib.rs:15:22
   |
15 |         let actual = repeat(input);
   |                      ^^^^^^ not found in this scope
```

## Write the Minimal Amount of Code

Keep the discipline! You don't need to know anything new right now to make the test fail properly. All you need to do right now is enough to make it compile so you can check your test is written well.

```rust
pub fn repeat(c: &str) -> String {
    String::from("")
}
```

Isn't it nice to know you already know enough Rust to write tests for some basic problems? This means you can now play with the production code as much as you like and know it's behaving as you'd hope.

Then, run the test again and see it fail with the reasonable message.

```bash
thread 'specs_for_repeat::sut_repeats_given_argument_10_times' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: "aaaaa"
 right: ""
```

## Write Enough Code to Make It Pass

The `for` syntax is very unremarkable and follows mock C-like language.

```rust
pub fn repeat(c: &str) -> String {
    let mut repeated = String::new();
    for _ in 0..10 {
        repeated = repeated + c;
    }
    repeated
}
```

Unlike other languages like C, Java, or JavaScript, there are no parentheses surrounding the three components of the for statement and the braces { } are always required.

Run the test and it should pass.

Additional variants of the loop syntax are described [here](https://doc.rust-lang.org/stable/rust-by-example/flow_control.html).

## Refactor

Now it's time to refactor. You can extract the magic number 5 into a constant and introduce another construct `+=` assignment operator.

```rust
const REPEAT_COUNT: usize = 10;

pub fn repeat(c: &str) -> String {
    let mut repeated = String::new();
    for _ in 0..REPEAT_COUNT {
        repeated += c;
    }
    repeated
}
```

## Benchmarking

Writing benchmarks in Rust is provided in the unstable benchmarking system with `cargo bench`. To use the command in the stable version, you need to use the [criterion](https://github.com/bheisler/criterion.rs) crate, which is a powerful and flexible benchmarking library for Rust. It provides a simple and easy-to-use API for writing benchmarks, and it generates detailed reports with statistical analysis of the results.

Let's configure criterion in the project. All is described in the [criterion documentation](https://bheisler.github.io/criterion.rs/book/getting_started.html). First, add criterion dependency to your `Cargo.toml` file:

```toml
[package]
name = "iteration"
version = "0.1.0"
edition = "2024"

[dependencies]

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "repeat"
harness = false
```

The `harness = false` line tells Cargo not to use the standard benchmark harness, which is important to disable to use criterion.

After that, create a benchmark file in the `benches` directory called `repeat.rs` and add the following code:

```rust,ignore
use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;

use iteration::repeat;

pub fn bench_repeat(c: &mut Criterion) {
    c.bench_function("repeat a", |b| {
        b.iter(|| {
            let _ = repeat(black_box("a"));
        })
    });
}

criterion_group!(benches, bench_repeat);
criterion_main!(benches);
```

The function name `batch_repeat` doesn't matter, but it should be clear and understandable. Inside of the function is where the real work happens. The `c.bench_function` method takes a name for the benchmark and a closure that contains the code to be benchmarked. The `black_box` function is used to prevent the compiler from optimizing away the code being benchmarked, which can lead to inaccurate results.

The last two lines are used to define the benchmark group and the main function for the benchmark. The `criterion_group!` macro creates a group of benchmarks, and the `criterion_main!` macro defines the main function that runs the benchmarks.

Now you can run the benchmark with `cargo bench` command.

```bash
     Running benches/repeat.rs (target/release/deps/repeat-f26c84f3dcacddd5)
Benchmarking repeat a: Warming up for 3.0000 s
Benchmarking repeat a: Collecting 100 samples in estimated 5.0000 s (145M iterations)
repeat a                time:   [34.997 ns 35.145 ns 35.309 ns]
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  10 (10.00%) high severe
```

You can find the meaning of output in details [here](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html). The most important part is the expected time the function takes to run, which is the number in the middle of the output time - 35.145 nanoseconds. The value is estimated based on the number of 145 millions iterations.

We can do better than that. The `repeat` function is not very efficient because it creates a new string for each iteration. We can use the `String::with_capacity` method to create a string with a pre-allocated capacity, which will reduce the number of allocations and improve performance. Also we can use the `push_str` method to append the string to the existing string, which is more efficient than using the `+=` operator.

```rust
const REPEAT_COUNT: usize = 10;

pub fn repeat(c: &str) -> String {
    let mut repeated = String::with_capacity(REPEAT_COUNT * c.len());
    for _ in 0..REPEAT_COUNT {
        repeated.push_str(c);
    }
    repeated
}
```

Now you can run the benchmark again with `cargo bench` command, and see the difference in performance.

```bash
     Running benches/repeat.rs (target/release/deps/repeat-f26c84f3dcacddd5)
Gnuplot not found, using plotters backend
repeat a                time:   [20.103 ns 20.113 ns 20.122 ns]
                        change: [-42.642% -42.459% -42.298%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
```

The performance has improved by 42% and the number of outliers has decreased.

## Practice Exercises

- Change the test so a caller can specify how many times the character is repeated and then fix the code
- Write documentation on your function
- Have a look through the [str](https://doc.rust-lang.org/std/primitive.str.html) and [String](https://doc.rust-lang.org/std/string/struct.String.html) documentation
  - Find functions you think could be useful and experiment with them by writing tests like we have here
  - Investigating time learning the standard library will really pay off over time

## Wrapping Up

- More TDD practice
- Learned about `for` loops
- Learned how to write benchmarks
