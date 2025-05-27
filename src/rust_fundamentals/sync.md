# Sync

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/sync).

We briefly touched on interior mutability in the [mocking](./mocking.md) chapter when creating hand-made mock objects. In this chapter, we'll explore how to use interior mutability with concurrency in production code. Before proceeding, I recommend reading the [Interior Mutability](https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html) and [Shared State Concurrency](https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html) chapters in the Rust Book.

## The First Requirement: Counter Without Concurrency

We want to create a counter that will eventually be safe to use concurrently.

Let's start with a simple counter and verify that it works correctly in a single-threaded environment.

### Write the Test First

We need an API that provides a method to increment the counter and retrieve its current value.

```rust
#[cfg(test)]
mod specs_for_counter {
    use super::Counter;

    #[test]
    fn sut_is_increased_3_times_and_leaves_it_at_3() {
        // Arrange
        let count = 3;
        let mut counter = Counter::new();

        // Act
        for _ in 0..count {
            counter.increase();
        }

        // Assert
        let actual = counter.value();
        assert_eq!(count, actual);
    }
}
```

### Try and Run the Test

```bash
error[E0433]: failed to resolve: use of undeclared type `Counter`
  --> src/lib.rs:28:27
   |
28 |         let mut counter = Counter::new();
   |                           ^^^^^^^ use of undeclared type `Counter`
   |
help: consider importing this struct
   |
24 +     use crate::Counter;
```

### Write the Minimal Amount of Code

Let's define our `Counter` type.

```rust
#[derive(Default, Debug)]
pub struct Counter;
```

Now the compiler complains about missing methods:

```bash
error[E0599]: no function or associated item named `new` found for struct `lib::Counter`
  --> src/v1.rs:28:36
   |
2  | pub struct Counter {
   | ------------------ function or associated item `new` not found for this struct
...
28 |         let mut counter = Counter::new();
   |                                    ^^^ function or associated item not found in `Counter`
```

Let's add the required methods to make the test compile:

```rust,ignore
impl Counter {
    fn new() -> Self {
        Self::default()
    }

    fn increase(&mut self) {}

    fn value(&self) -> usize {
        0
    }
}
```

Now the test runs but fails as expected:

```bash
thread 'lib::specs_for_counter::sut_is_increased_3_times_and_leaves_it_at_3' panicked at src/lib.rs:33:9:
assertion `left == right` failed
  left: 3
 right: 0
```

### Write Enough Code to Make It Pass

This should be straightforward. We need to store state in our `Counter` and increment it on each call to `increase`:

```rust
#[derive(Default, Debug)]
pub struct Counter {
    value: usize,
}

impl Counter {
    fn new() -> Self {
        Self::default()
    }

    fn increase(&mut self) {
        self.value += 1;
    }

    fn value(&self) -> usize {
        self.value
    }
}
```

## The Second Requirement: Counter With Concurrency

That was simple enough. Now we have a new requirement: the counter must be safe to use in a concurrent environment.

First, let's add the necessary dependencies:

```toml
[dependencies]
futures = "0.3"
tokio = { version = "1.45", features = ["rt-multi-thread", "macros"] }
```

### Write the Test First

```rust,ignore
#[tokio::test]
async fn sut_runs_concurrently_safe() {
    // Arrange
    let count = 1000;
    let counter = Arc::new(Counter::new());

    // Act
    let handles = (0..count)
        .map(|_| {
            let counter = Arc::clone(&counter);
            tokio::spawn(async move {
                counter.increase();
            })
        })
        .collect::<Vec<_>>();
    join_all(handles).await;

    // Assert
    let actual = counter.value();
    assert_eq!(actual, count);
}
```

This test creates 1000 concurrent tasks, each calling `counter.increase()` once.

We use `Arc` (Atomic Reference Counting) to share ownership of the counter across multiple threads.

### Try to Run the Test

```bash
error[E0596]: cannot borrow data in an `Arc` as mutable
  --> src/lib.rs:55:21
   |
55 |                     counter.increase();
   |                     ^^^^^^^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not
 implemented for `Arc<lib::Counter>`
```

Let's understand this error. We're calling `increase` on an `Arc<Counter>`, but `increase` requires `&mut self` - a mutable reference. Rust attempts to dereference the `Arc` as mutable using the `DerefMut` trait, but this fails. `Arc` is designed for shared ownership and only provides immutable access to its contents.

We need to use interior mutability to solve this problem.

### Write Enough Code to Make It Pass

First, let's change the signature of `increase` to take `&self` instead of `&mut self`:

```rust,ignore
impl Counter {
    fn increase(&self) {
        self.value += 1;
    }
}
```

Now we get a different error:

```bash
  --> src/v2.rs:12:9
   |
12 |         self.value += 1;
   |         ^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be written
```

Rust is telling us we can't modify `self.value` through a shared reference. This is where interior mutability comes in:

```rust
use std::sync::RwLock;

#[derive(Default, Debug)]
pub struct Counter {
    value: RwLock<usize>,
}

impl Counter {
    fn new() -> Self {
        Self::default()
    }

    fn increase(&self) {
        *self.value.write().unwrap() += 1;
    }

    fn value(&self) -> usize {
        *self.value.read().unwrap()
    }
}
```

By wrapping our `value` in a `RwLock`, we can modify it even with just a shared reference (`&self`). Instead of direct access, we use `write()` and `read()` methods to manage locks on the internal value.

Rust offers four main types for interior mutability: `Cell`, `RefCell`, `Mutex`, and `RwLock`. You can find detailed information [here](https://marabos.nl/atomics/basics.html#cell).

For concurrent access, only `Mutex` and `RwLock` are thread-safe. `Mutex` provides exclusive access for both reading and writing, while `RwLock` allows multiple concurrent readers or one exclusive writer. Choose based on your specific use case.

### Refactor

In our first test, we declared the counter as mutable (`let mut counter = Counter::new()`). Since we no longer need `&mut self` for `increase()`, we can remove the `mut` keyword.

## Wrapping up

We've successfully explored interior mutability and its implementations in Rust:

- Used `RwLock` to coordinate access to shared state
- Understood the trade-offs between different interior mutability types
- Leveraged Rust's type system for compile-time safety guarantees

### The power of Rust's safety guarantees

In other languages like Go, you must manually ensure proper synchronization. Without it, you could easily create a [buggy counter](https://quii.gitbook.io/learn-go-with-tests/go-fundamentals/sync#try-to-run-the-test-1) that fails under concurrent access.

With Rust, creating such bugs is nearly impossible. The compiler prevents you from building unsafe concurrent code without explicit use of `unsafe` blocks. Try creating a counter that doesn't work properly with concurrency using only safe Rust - you'll find the compiler stops you at every turn. This compile-time safety is one of Rust's greatest strengths.

### When to use different synchronization primitives?

- **`Cell`**: Simple value replacement for `Copy` types, no borrowing needed
- **`RefCell`**: Runtime borrow checking for single-threaded scenarios  
- **`Mutex`**: Thread-safe exclusive access, blocks all threads during access
- **`RwLock`**: Thread-safe with concurrent readers or exclusive writers

Choose the right tool based on your concurrency needs and performance requirements.
