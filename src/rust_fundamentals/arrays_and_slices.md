# Arrays and Slices

<!-- TODO: Modify the content to use `<const N: usize>` instead of vectors, referencing [maps](./maps.md) -->

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/arrays).

Arrays let you store multiple elements of the same type in a variable, maintaining a specific order.

When working with arrays, iteration is a common operation. Let's leverage [our knowledge of `for` loops](./iteration.md) to create a `sum` function that takes an array of numbers and returns their total.

Time to apply our TDD skills!

## The First Requirement: Sum an Array of Five Numbers

### Write the Test First

Create a new project and add the following to `lib.rs`:

```rust
#[cfg(test)]
mod specs_for_sum {
    #[test]
    fn sut_returns_15_if_input_array_is_1_to_5() {
        // Arrange
        let numbers = [1, 2, 3, 4, 5];

        // Act
        let actual = sum(&numbers);

        // Assert
        let expected = 15;
        assert_eq!(expected, actual);
    }
}
```

Arrays have a fixed size that you define at declaration time. They're useful when you want data allocated on the stack rather than the heap. Learn more about this distinction [here](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap).

### Write the Minimal Amount of Code

Add a `sum` function to make the code compile:

```rust
pub fn sum(numbers: &[i32; 5]) -> i32 {
    0
}
```

The type `&[i32; 5]` represents a reference to an array of 5 integers. The `&` means we're borrowing the array rather than taking ownership. This is efficient because arrays have fixed sizes, and passing them by value would require copying all elements. The `; 5` specifies the array's size.

Your test should now fail with a clear message:

```bash
thread 'specs_for_sum::sut_returns_15_if_input_array_is_1_to_5' panicked at src/lib.rs:24:9:
assertion `left == right` failed
  left: 15
 right: 0
```

### Write Enough Code to Make It Pass

```rust
pub fn sum(numbers: &[i32; 5]) -> i32 {
    let mut total = 0;
    for i in 0..5 {
        total += numbers[i];
    }
    total
}
```

We access array elements using `array[index]` syntax. Here, we iterate 5 times using a `for` loop, adding each element to our `total`.

### Refactor

Let's use the `for...in` pattern for cleaner code:

```rust
pub fn sum(numbers: &[i32; 5]) -> i32 {
    let mut total = 0;
    for number in numbers {
        total += number;
    }
    total
}
```

The `for...in` loop lets you iterate directly over array elements. On each iteration, `number` holds the current element's value - a more idiomatic Rust approach.

An important characteristic of arrays is that their size is part of their type. You can't pass a `[i32; 4]` to a function expecting `[i32; 5]` - they're different types, just like `String` and `i32` are different.

You might find fixed-size arrays limiting, and you'd be right - most of the time, you'll want something more flexible.

Enter slices: dynamically sized views into collections. Create a slice from an array using `&`: for example, `&numbers[0..3]` creates a slice of the first 3 elements. Slice types are written as `&[T]`, where `T` is the element type. This flexibility allows functions to accept collections of any size.

Let's extend our function to handle collections of varying sizes.

## The Second Requirement: Sum an Array of Varying Sizes

### Write the Test First

We'll use the [slice type](https://doc.rust-lang.org/stable/book/ch04-03-slices.html) for collections of any size. The syntax is similar to arrays - just omit the size: `&[i32]` instead of `&[i32; 5]`.

```rust
#[cfg(test)]
mod specs_for_sum {
    use super::sum;

    #[test]
    fn sut_returns_15_if_input_array_is_1_to_5() {
        // Arrange
        let numbers = [1, 2, 3, 4, 5];

        // Act
        let actual = sum(&numbers);

        // Assert
        let expected = 15;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_6_if_input_array_is_1_to_3() {
        // Arrange
        let numbers = [1, 2, 3];

        // Act
        let actual = sum(&numbers);

        // Assert
        let expected = 6;
        assert_eq!(expected, actual);
    }
}
```

### Try and Run the Test

The code won't compile - we're trying to pass a 3-element array to a function expecting 5 elements:

```bash
  --> src/lib.rs:41:26
   |
41 |         let actual = sum(&numbers);
   |                      --- ^^^^^^^^ expected an array with a size of 5, found one with a size of 3
   |                      |
   |                      arguments to this function are incorrect
```

### Write the Minimal Amount of Code

We have two options:
- Break the existing API by changing the parameter to accept slices
- Create a new function

Since we control all uses of this function, let's simplify and maintain just one:

```rust
pub fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for number in numbers {
        total += number;
    }
    total
}
```

Run the tests - they pass!

You might wonder why this works when we're passing arrays but the function expects a slice. Rust automatically converts arrays to slices through [unsized coercion](https://doc.rust-lang.org/reference/type-coercions.html#unsized-coercions) - a convenient feature that allows seamless interoperability between arrays and slices.

### Write Enough Code to Make It Pass

Fixing the compiler errors was all we needed - the tests already pass!

### Refactor

Our function is already refactored. By replacing arrays with slices, we've made it more flexible without additional changes. 

Let's also improve our test suite:

```rust
#[cfg(test)]
mod specs_for_sum {
    use super::sum;

    #[test]
    fn sut_returns_15_if_input_array_is_1_to_5() {
        // Arrange
        let numbers = [1, 2, 3, 4, 5];

        // Act
        let actual = sum(&numbers);

        // Assert
        let expected = 15;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_6_if_input_array_is_1_to_3() {
        // Arrange
        let numbers = [1, 2, 3];

        // Act
        let actual = sum(&numbers);

        // Assert
        let expected = 6;
        assert_eq!(expected, actual);
    }
}
```

Always question the value of your tests. The goal isn't maximum test count but maximum confidence in your code. Too many tests become a maintenance burden. **Every test has a cost.**

In our case, these two tests are somewhat redundant. If the function works for one slice size, it likely works for any reasonable size.

Rust offers [Tarpaulin](https://github.com/xd009642/tarpaulin) for code coverage reports. While 100% coverage shouldn't be your ultimate goal, coverage tools help identify untested code. With strict TDD, you'll naturally achieve high coverage.

Try installing and running Tarpaulin:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin
```

You should see output like:

```bash
2025-05-06T15:38:52.562193Z  INFO cargo_tarpaulin::report: Coverage Results:
|| Uncovered Lines:
|| src/main.rs: 1-2
|| Tested/Total Lines:
|| src/lib.rs: 4/4 +0.00%
|| src/main.rs: 0/2 +0.00%
||
66.67% coverage, 4/6 lines covered, +0.00% change in coverage
```

`lib.rs` has 100% coverage. Try deleting one test and checking coverage again.

Once you're satisfied with your well-tested function, commit your work before tackling the next challenge.

## The Third Requirement: Sum Each Array

We need a `sum_all` function that takes multiple slices and returns a new slice containing each slice's total.

For example:
- `sum_all(&[&[1, 2], &[0, 9]])` returns `[3, 9]`
- `sum_all(&[&[1, 1, 1]])` returns `[3]`

Unlike Go's [variadic functions](https://gobyexample.com/variadic-functions), Rust doesn't have this feature. Instead, we'll use a slice of slices: `&[&[T]]` - essentially a collection of collections.

### Write the Test First

We'll create two arrays and pass them to `sum_all` as a slice of slices:

```rust
#[cfg(test)]
mod specs_for_sum_all {
    #[test]
    fn sut_returns_two_summed_up_elements_if_two_arrays_are_given() {
        // Arrange
        let numbers_1 = [1, 2];
        let numbers_2 = [0, 9];

        // Act
        let actual = sum_all(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = [3, 9];
        assert_eq!(expected, actual);
    }
}
```

### Try and Run the Test

```bash
error[E0425]: cannot find function `sum_all` in this scope
  --> src/lib.rs:27:22
   |
27 |         let actual = sum_all(&numbers_1, &numbers_2);
   |                      ^^^^^^^ not found in this scope
```

### Write the Minimal Amount of Code

Let's define `sum_all` based on our test's requirements. We'll start with `[T]` as the return type:

```rust,ignore
pub fn sum_all(numbers_to_sum: &[&[i32]]) -> [i32] {
    [0]
}
```

Building with `cargo build` produces:

```bash
error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
  --> src/lib.rs:26:39
   |
26 | pub fn sum_all(numbers_to_sum: &[&[i32]]) -> [i32] {
   |                                       ^^^^^ doesn't have a size known at compile-time
27 |     [0]
   |     --- this returned value is of type `[i32; 1]`
   |
   = help: the trait `Sized` is not implemented for `[i32]`
   = note: the return type of a function must have a statically known size
```

Rust requires compile-time knowledge of return type sizes. Let's try returning a slice reference `&[T]`:

```rust,ignore
pub fn sum_all(numbers_to_sum: &[&[i32]]) -> &[i32] {
    &[0]
}
```

Another error appears:

```bash
error[E0106]: missing lifetime specifier
  --> src/lib.rs:26:39
   |
26 | pub fn sum_all(numbers_to_sum: &[&[i32]]) -> &[i32] {
   |                         ---------     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `numbers`'s 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
   |
26 | pub fn sum_all<'a>(numbers_to_sum: &'a [&'a [i32]]) -> &'a [i32] {
   |               ++++           ++   ++             ++
```

The compiler suggests adding lifetimes, but its suggestion won't work for our use case. Since we're creating new data inside the function, we need different lifetimes:

```rust
pub fn sum_all<'a, 'b>(numbers_to_sum: &'a [&'a [i32]]) -> &'b [i32] {
    &[0]
}
```

It compiles! (Though this approach has issues we'll discover later.) Running the test shows:

```bash
thread 'specs_for_sum_all::sut_returns_two_summed_up_elements_if_two_arrays_are_given' panicked at src/lib.rs:45:9:
assertion `left == right` failed
  left: [3, 9]
 right: [0]
```

### Write Enough Code to Make It Pass

Let's implement `sum_all` using our existing `sum` function:

```rust,ignore
pub fn sum_all<'a, 'b>(numbers_to_sum: &'a [&'a [i32]]) -> &'b [i32] {
    let length = numbers_to_sum.len();
    let mut sums = [0; length];
    for (i, numbers) in numbers_to_sum.iter().enumerate() {
        sums[i] = sum(numbers);
    }
    &sums
}
```

Building again reveals:

```bash
error[E0435]: attempt to use a non-constant value in a constant
  --> src/lib.rs:28:23
   |
28 |     let mut sums = [0; length];
   |                       ^^^^^^ non-constant value
   |
help: consider using `const` instead of `let`
   |
27 |     const length: /* Type */ = numbers_to_sum.len();
   |     ~~~~~       ++++++++++++
```

We can't determine the array size at compile time because we don't know how many slices will be passed.

The solution? Use a vector - Rust's growable, heap-allocated array type:

```rust,ignore
pub fn sum_all(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    let mut sums = Vec::new();
    for numbers in numbers_to_sum {
        sums.push(sum(numbers));
    }
    sums
}

#[cfg(test)]
mod specs_for_sum_all {
    use super::sum_all;

    #[test]
    fn sut_returns_two_summed_up_elements_if_two_arrays_are_given() {
        // Arrange
        let numbers_1 = [1, 2];
        let numbers_2 = [0, 9];

        // Act
        let actual = sum_all(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![3, 9];
        assert_eq!(expected, actual);
    }
}
```

The test now passes!

### Refactor

We can optimize by pre-allocating the vector when we know its size:

```rust,ignore
pub fn sum_all(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    let mut sums = Vec::with_capacity(numbers_to_sum.len());
    for numbers in numbers_to_sum {
        sums.push(sum(numbers));
    }
    sums
}
```

### Write More Tests

Tests serve as documentation. While we've tested arrays and slices, our function also works with vectors. Let's add a test to demonstrate this:

```rust
#[cfg(test)]
mod specs_for_sum_all {
    use super::sum_all;

    #[test]
    fn sut_returns_two_summed_up_elements_if_two_arrays_are_given() {
        // Arrange
        let numbers_1 = [1, 2];
        let numbers_2 = [0, 9];

        // Act
        let actual = sum_all(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![3, 9];
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_two_summed_up_elements_if_two_vectors_are_given() {
        // Arrange
        let numbers_1 = vec![1, 2];
        let numbers_2 = vec![0, 9];

        // Act
        let actual = sum_all(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![3, 9];
        assert_eq!(expected, actual);
    }
}
```

## The Fourth Requirement: Sum All Tails

Our next requirement: create `sum_all_tails`, which calculates totals of each slice's "tail" (all elements except the first).

### Write the Test First

```rust
#[cfg(test)]
mod specs_for_sum_all_tails {
    #[test]
    fn sut_returns_last_elements_in_vector_correctly() {
        // Arrange
        let numbers_1 = vec![1, 2, 3];
        let numbers_2 = vec![0, 9, 10];

        // Act
        let actual = sum_all_tails(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![5, 19];
        assert_eq!(expected, actual);
    }
}
```

### Try and Run the Test

```bash
error[E0432]: unresolved import `super::sum_all_tails`
  --> src/lib.rs:28:9
   |
28 |     use super::sum_all_tails;
   |         ^^^^^^^^^^^^^^^^^^^^^ no `sum_all_tails` in the root
```

### Write the Minimal Amount of Code

Add the function to `lib.rs`:

```rust
pub fn sum_all_tails(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    Vec::new()
}
```

The test now fails as expected:

```bash
thread 'specs_for_sum_all_tails::sut_returns_last_elements_in_vector_correctly' panicked at src/lib.rs:45:9:
assertion `left == right` failed
  left: [5, 19]
 right: []
```

### Write Enough Code to Make It Pass

```rust,ignore
pub fn sum_all_tails(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    let mut sums = Vec::with_capacity(numbers_to_sum.len());
    for numbers in numbers_to_sum {
        sums.push(sum(&numbers[1..]));
    }
    sums
}
```

Slices support slicing! The syntax `slice[low..high]` creates a sub-slice. Omitting a bound captures everything to that side. Here, `numbers[1..]` means "from index 1 to the end". Experiment with slice operators to deepen your understanding.

### Refactor

Not much to refactor here.

Consider: what happens if you pass an empty slice? What's the tail of an empty slice? What does `empty_slice[1..]` do?

Let's test the empty slice case.

### Write the Test First

```rust
#[cfg(test)]
mod specs_for_sum_all_tails {
    use super::sum_all_tails;

    #[test]
    fn sut_returns_sum_of_each_collection_in_vector_correctly() {
        // Arrange
        let numbers_1 = vec![1, 2, 3];
        let numbers_2 = vec![0, 9, 10];

        // Act
        let actual = sum_all_tails(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![5, 19];
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_sets_summed_value_as_0_for_empty_collection() {
        // Arrange
        let numbers_1 = vec![];
        let numbers_2 = vec![3, 4, 5];

        // Act
        let actual = sum_all_tails(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![0, 9];
        assert_eq!(expected, actual);
    }
}
```

### Try and Run the Test

```bash
thread 'specs_for_sum_all_tails::sut_sets_summed_value_as_0_for_empty_collection' panicked at src/lib.rs:29:31:
range start index 1 out of range for slice of length 0
```

A runtime error! While the code compiled, it panics at runtime.

Compile-time errors are friends that help us write working software. Runtime errors are enemies that affect users.

### Write Enough Code to Make It Pass

```rust,ignore
pub fn sum_all_tails(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    let mut sums = Vec::with_capacity(numbers_to_sum.len());
    for numbers in numbers_to_sum {
        if numbers.is_empty() {
            sums.push(0);
        } else {
            sums.push(sum(&numbers[1..]));
        }
    }
    sums
}
```

### Refactor

Our solution works well and passes all tests. Time for the next challenge!

## Wrapping Up

We've covered:
- Arrays
- Slices  
- Vectors
- Test coverage tools
- Unsized coercion
- A glimpse of lifetimes

We've used these collection types with integers, but they work with any type - even other collections, as demonstrated with `&[&[T]]`.

We also encountered two special Rust concepts: coercion and lifetimes. While these topics deserve deeper study, knowing they exist is sufficient for now. The Rust Book offers excellent chapters on [lifetimes](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html) and [coercion](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods) when you're ready to explore further.