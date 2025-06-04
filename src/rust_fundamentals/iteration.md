# Iteration

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/iteration).

For repetitive tasks, Rust provides `loop`, `while`, and `for`. The `loop` keyword creates an infinite loop that you exit with `break`. The `while` loop continues until a condition becomes false. The `for` loop iterates over collections like arrays or vectors.

Let's write a test for a function that repeats a character 10 times.

Nothing new here - try writing it yourself for practice.

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

Maintain discipline! You don't need new knowledge to make the test fail properly. Just write enough to compile and verify your test is well-written:

```rust
pub fn repeat(c: &str) -> String {
    String::from("")
}
```

Isn't it satisfying to know you already understand enough Rust to write tests for basic problems? This means you can experiment with production code confidently, knowing it behaves as expected.

Run the test again to see it fail with a meaningful message:

```bash
thread 'specs_for_repeat::sut_repeats_given_argument_10_times' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: "aaaaa"
 right: ""
```

## Write Enough Code to Make It Pass

The `for` syntax is straightforward and follows C-like conventions:

```rust
pub fn repeat(c: &str) -> String {
    let mut repeated = String::new();
    for _ in 0..10 {
        repeated = repeated + c;
    }
    repeated
}
```

Unlike C, Java, or JavaScript, Rust requires no parentheses around the for statement components, and braces `{}` are always mandatory.

Run the test - it should pass.

For additional loop syntax variants, see [here](https://doc.rust-lang.org/stable/rust-by-example/flow_control.html).

## Refactor

Time to refactor! Extract the magic number into a constant and use the `+=` assignment operator:

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

Rust's unstable benchmarking system uses `cargo bench`. For stable Rust, use the [criterion](https://github.com/bheisler/criterion.rs) crate - a powerful, flexible benchmarking library that provides simple APIs and detailed statistical reports.

Let's configure criterion following the [documentation](https://bheisler.github.io/criterion.rs/book/getting_started.html). Add criterion to your `Cargo.toml`:

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

The `harness = false` tells Cargo not to use the standard benchmark harness, which is necessary for criterion.

Create `benches/repeat.rs`:

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

The function name `bench_repeat` should be clear and descriptive. The `c.bench_function` method takes a benchmark name and a closure containing the code to benchmark. The `black_box` function prevents compiler optimizations that could skew results.

The final two lines define the benchmark group and main function using the `criterion_group!` and `criterion_main!` macros.

Run the benchmark with `cargo bench`:

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

Find detailed output explanations [here](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html). The key metric is the middle execution time: 35.145 nanoseconds, estimated from 145 million iterations.

We can improve this. Our `repeat` function is inefficient because it creates new strings each iteration. Use `String::with_capacity` for pre-allocation and `push_str` for more efficient string appending:

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

Run `cargo bench` again to see the performance difference:

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

Performance improved by 42% with fewer outliers!

## Practice Exercises

- Modify the test so callers can specify repeat count, then update the code
- Add documentation to your function
- Explore the [str](https://doc.rust-lang.org/std/primitive.str.html) and [String](https://doc.rust-lang.org/std/string/struct.String.html) documentation
  - Find useful functions and experiment with test-driven exploration
  - Time spent learning the standard library pays tremendous dividends

## Wrapping Up

- More TDD practice
- Learned `for` loop usage
- Learned benchmark writing techniques