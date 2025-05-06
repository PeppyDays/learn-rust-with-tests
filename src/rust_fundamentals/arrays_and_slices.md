# Arrays and Slices

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/arrays).

Arrays allow you to store multiple elements of the same type in a variable in a particular order.

When you have arrays, it is very common to have to iterate over them. So let's use [our new-found knowledge of `for`](./iteration.md) to make a `sum` function. `sum` will take an array of numbers and return the total.

Let's use our TDD skills.

## The First Requirement: Sum an Array of 5 Numbers

### Write the Test First

Create a new project to work in. Create a new file `lib.rs` and insert the following:

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

Arrays have a fixed number of elements which you define when you declare the variable, and are useful when you want your data allocated on the stack rather than heap. You can see the distinction between stack and heap [here](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap).

### Write the Minimal Amount of Code

Now we need to add a function `sum` to avoid the compilation error. We can do this by adding the following code to `lib.rs`:

```rust
pub fn sum(numbers: &[i32; 5]) -> i32 {
    0
}
#
# #[cfg(test)]
# mod specs_for_sum {
#     use super::sum;
#
#     #[test]
#     fn sut_returns_15_if_input_array_is_1_to_5() {
#         // Arrange
#         let numbers = [1, 2, 3, 4, 5];
#
#         // Act
#         let actual = sum(&numbers);
#
#         // Assert
#         let expected = 15;
#         assert_eq!(expected, actual);
#     }
# }
```

The input variable type `&[i32; 5]` means that it is a reference of an array of 5 integers. The `&` symbol indicates that we are passing a reference to the array, not the array itself. This is important because arrays in Rust have a fixed size, and passing them by value would require copying the entire array, which can be inefficient. And the `; 5` indicates the size of the array.

Now your test should fail with a clear error message:

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
#
# #[cfg(test)]
# mod specs_for_sum {
#     use super::sum;
#
#     #[test]
#     fn sut_returns_15_if_input_array_is_1_to_5() {
#         // Arrange
#         let numbers = [1, 2, 3, 4, 5];
#
#         // Act
#         let actual = sum(&numbers);
#
#         // Assert
#         let expected = 15;
#         assert_eq!(expected, actual);
#     }
# }
```

To get the value out of an array at a particular index, just use `array[index]` syntax. In this case, we are using `for` to iterate 5 times to work through the array, and add each item onto `total`.

### Refactor

Let's introduce `in` in `for` to help clean up our code.

```rust
pub fn sum(numbers: &[i32; 5]) -> i32 {
    let mut total = 0;
    for number in numbers {
        total += number;
    }
    total
}
#
# #[cfg(test)]
# mod specs_for_sum {
#     use super::sum;
#
#     #[test]
#     fn sut_returns_15_if_input_array_is_1_to_5() {
#         // Arrange
#         let numbers = [1, 2, 3, 4, 5];
#
#         // Act
#         let actual = sum(&numbers);
#
#         // Assert
#         let expected = 15;
#         assert_eq!(expected, actual);
#     }
# }
```

`in` lets you iterate over an array. On each iteration, `number` will be set to the value of the current element in the array. This is a more idiomatic way to iterate over an array in Rust.

An interesting property of arrays is that the size is part of the type. If you try to pass an array [i32; 4] into a function that expects [i32; 5], it will not compile. They are different types so it is just the same as trying to pass a `String` into a function that wants an `i32`.

You may be thinking it is quite cumbersome that arrays have a fixed length, and most of the time you probably will not be using them!

Rust has slices which are a view into a collection. They are a dynamically sized view into an collection. You can create a slice from an array by using the `&` operator. For example, `&numbers[0..3]` creates a slice of the first 3 elements of the array `numbers`. Type of slice is `&[T]`, which is a reference to a slice. This means that you can pass slices to functions that expect `&[T]` without having to worry about the size of the array.

The next requirement will be to sum collections of varying sizes.

## The Second Requirement: Sum an Array of Varying Sizes

### Write the Test First

We will now use the [slice type](https://doc.rust-lang.org/stable/book/ch04-03-slices.html) which allows us to have collections of any size. The syntax is very similar to arrays, you just omit the size when declaring them. `numbers: &[i32; 5]` rather than `numbers: &[i32]`.

```rust
# pub fn sum(numbers: &[i32; 5]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
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

### Try and Rust the Test

This does not compile because we are trying to pass an array of 3 integers into a function that expects an array of 5 integers. We need to change the function signature to accept a slice instead.

```bash
  --> src/lib.rs:41:26
   |
41 |         let actual = sum(&numbers);
   |                      --- ^^^^^^^^ expected an array with a size of 5, found one with a size of 3
   |                      |
   |                      arguments to this function are incorrect
```

### Write the Minimal Amount of Code

The problem here is we can either:

- Break the existing API by changing the argument to be a slice rather than array
  - We could potentially ruin someone's day because our other tests will no longer compile!
- Create a new function

In our case, no one else is using our function, so rather than having two functions to maintain, let's have just one.

```rust
pub fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for number in numbers {
        total += number;
    }
    total
}
#
# #[cfg(test)]
# mod specs_for_sum {
#     use super::sum;
#
#     #[test]
#     fn sut_returns_15_if_input_array_is_1_to_5() {
#         // Arrange
#         let numbers = [1, 2, 3, 4, 5];
#
#         // Act
#         let actual = sum(&numbers);
#
#         // Assert
#         let expected = 15;
#         assert_eq!(expected, actual);
#     }
#
#     #[test]
#     fn sut_returns_6_if_input_array_is_1_to_3() {
#         // Arrange
#         let numbers = [1, 2, 3];
#
#         // Act
#         let actual = sum(&numbers);
#
#         // Assert
#         let expected = 6;
#         assert_eq!(expected, actual);
#     }
# }
```

If you try to run the tests, they will pass.

You might wonder why the tests pass because the function signature wants a slice, but we are passing an array. The reason is that Rust will automatically convert the array into a slice when you pass it to the function. This is called [unsized coercion](https://doc.rust-lang.org/reference/type-coercions.html#unsized-coercions). It is a very useful feature of Rust that allows you to work with arrays and slices interchangeably in many cases.

### Write Enough Code to Make It Pass

It turns out that fixing the compiler problems were all we need to do here, and the tests pass!

### Refactor

We already refactored the function. All we did was replace arrays with slices, so no extra changes are required. Remember that we must not neglect our test code in the refactoring stage. We can further improve our tests.

```rust
# pub fn sum(numbers: &[i32]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
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

It is important to question the value of your tests. It should not be a goal to have as many tests as possible, but rather to have as much confidence as possible in your code base. Having too many tests can turn in to a real problem and it just adds more overhead in maintenance. **Every test has a cost.**

In our case, you can see that having two tests for this function is redundant. If it works for a slice of one size it's very likely it'll work for a slice of any size (within reason).

Rust has code coverage report tool called [Tarpaulin](https://github.com/xd009642/tarpaulin). Whilst striving for 100% coverage should not be your end goal, the coverage tool can help identify areas of your code not covered by tests. If you have been strict with TDD, it's quite likely you'll have close to 100% coverage anyway.

Try install Tarpaulin and run it on your code. You can install and run tests with the following command:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin
```

You should see output similar to this.

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

`lib.rs` has 100% coverage. Now delete one of the tests and check the coverage again.

Now that we are happy we have a well-tested function you should commit your great work before taking on the next challenge.

## The Third Requirement: Sum Each Array

We need a new function called `sum_all` which will take a varying number of slices, returning a new slice containing the totals for each slice passed in.

For example, if we pass `&[1, 2]` and `&[0, 9]`, `sum_all` function would return `[3, 9]`. Or `&[1, 1, 1]` would return `[3]`.

One problem here is that we don't know how many slices we will be passing in. In Go, you would use a [variadic function](https://gobyexample.com/variadic-functions) to easily achieve this, but Rust doesn't have that feature.

We can use a slice of slices to solve this problem. The syntax is `&[&[T]]`. Simply speaking, ii means a collection of collections.

### Write the Test First

After arranging two arrays, we will pass them into `sum_all` by merging into a new array. This enables us to pass in a varying number of arrays.

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

We need to define `sum_all` according to what our test wants. However you might be confused about the return type of the function. Let's use `[T]` for now, and we will fix it later.

```rust,ignore
pub fn sum_all(numbers_to_sum: &[&[i32]]) -> [i32] {
    [0]
}
```

If we build it with `cargo build`, we will see the following error:

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

Rust doesn't allow you to return a dynamically sized type. This is because the compiler needs to know the size of the return type at compile time. We need to fix it. Let's try to return slice `&[T]` instead because its size is known at compile time.

```rust,ignore
pub fn sum_all(numbers_to_sum: &[&[i32]]) -> &[i32] {
    &[0]
}
```

After building it again, we will see the following error:

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

Okay, lifetimes. Let's try to add a lifetime, but the compiler's suggestion is not correct. We need to create an output inside the function and return it, so that the lifetime of the output is different from the input's lifetime. Give the different lifetimes as follows.

```rust
pub fn sum_all<'a, 'b>(numbers_to_sum: &'a [&'a [i32]]) -> &'b [i32] {
    &[0]
}
```

Now it compiles! Not sure what it means, but it compiles. (But, spoiler! This will not work later.) Let's keep going for now. If we run the test, we will see the expected error:

```bash
thread 'specs_for_sum_all::sut_returns_two_summed_up_elements_if_two_arrays_are_given' panicked at src/
lib.rs:45:9:
assertion `left == right` failed
  left: [3, 9]
 right: [0]
```

### Write Enough Code to Make It Pass

Now we need to implement the `sum_all` function. We can use the `sum` function we created earlier to sum each array and return a new array with the results.

```rust,ignore
# pub fn sum(numbers: &[i32]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
pub fn sum_all<'a, 'b>(numbers_to_sum: &'a [&'a [i32]]) -> &'b [i32] {
    let length = numbers_to_sum.len();
    let mut sums = [0; length];
    for (i, numbers) in numbers_to_sum.iter().enumerate() {
        sums[i] = sum(numbers);
    }
    &sums
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
        let expected = [3, 9];
        assert_eq!(expected, actual);
    }
}
```

If we build it again, we will see the following error:

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

The reason is that we cannot determine the output size at compile time. Rust wants to set the exact size of returned variable `sums` array, but we cannot do that because we don't know how many arrays we will be passing in.

We can use a vector instead of an array. A vector is a growable collection type in Rust. It is a heap-allocated, dynamically sized array. You can think of it as a collection of elements that can grow and shrink in size as needed.

Let's change the implementation of `sum_all` and its tests to use a vector instead of an array.

```rust
# pub fn sum(numbers: &[i32]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
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

Finally, we can run the test and it should pass!

### Refactor

As we discussed earlier, we can pre-allocate the vector with the size of the input slice if the size is known. This will help us avoid the overhead of resizing the vector as we add elements to it.

```rust
# pub fn sum(numbers: &[i32]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
pub fn sum_all(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    let mut sums = Vec::with_capacity(numbers_to_sum.len());
    for numbers in numbers_to_sum {
        sums.push(sum(numbers));
    }
    sums
}
#
# #[cfg(test)]
# mod specs_for_sum_all {
#     use super::sum_all;
#
#     #[test]
#     fn sut_returns_two_summed_up_elements_if_two_arrays_are_given() {
#         // Arrange
#         let numbers_1 = [1, 2];
#         let numbers_2 = [0, 9];
#
#         // Act
#         let actual = sum_all(&[&numbers_1, &numbers_2]);
#
#         // Assert
#         let expected = vec![3, 9];
#         assert_eq!(expected, actual);
#     }
# }
```

### Write More Tests

Tests can be documentation to help others understand how to use your code. We wrote a test only for working with arrays or slices of integers, but it also works with vectors. If we think working with vectors matters, we can add a test for it.

```rust
# pub fn sum(numbers: &[i32]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
# pub fn sum_all(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
#     let mut sums = Vec::with_capacity(numbers_to_sum.len());
#     for numbers in numbers_to_sum {
#         sums.push(sum(numbers));
#     }
#     sums
# }
#
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

## The Fourth Requirement: Gather All Trails

Our next requirement is to add `sum_all_trails` function, where it will calculate the totals of the "tails" of each slice. The tail of a collection is all items in the collection except the first one (the "head").

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

Add the function `sum_all_trails` to `lib.rs`:

```rust
# pub fn sum(numbers: &[i32]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
pub fn sum_all_tails(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    Vec::new()
}
#
# #[cfg(test)]
# mod specs_for_sum_all_tails {
#     use super::sum_all_tails;
#
#     #[test]
#     fn sut_returns_last_elements_in_vector_correctly() {
#         // Arrange
#         let numbers_1 = vec![1, 2, 3];
#         let numbers_2 = vec![0, 9, 10];
#
#         // Act
#         let actual = sum_all_tails(&[&numbers_1, &numbers_2]);
#
#         // Assert
#         let expected = vec![5, 19];
#         assert_eq!(expected, actual);
#     }
# }
```

And re-run the tests. You should see the following error:

```bash
thread 'specs_for_sum_all_tails::sut_returns_last_elements_in_vector_co
rrectly' panicked at src/lib.rs:45:9:
assertion `left == right` failed
  left: [5, 19]
 right: []
```

### Write Enough Code to Make It Pass

```rust
# pub fn sum(numbers: &[i32]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
pub fn sum_all_tails(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    let mut sums = Vec::with_capacity(numbers_to_sum.len());
    for numbers in numbers_to_sum {
        sums.push(sum(&numbers[1..]));
    }
    sums
}
#
# #[cfg(test)]
# mod specs_for_sum_all_trails {
#     use super::sum_all_tails;
#
#     #[test]
#     fn sut_returns_last_elements_in_vector_correctly() {
#         // Arrange
#         let numbers_1 = vec![1, 2, 3];
#         let numbers_2 = vec![0, 9, 10];
#
#         // Act
#         let actual = sum_all_tails(&[&numbers_1, &numbers_2]);
#
#         // Assert
#         let expected = vec![5, 19];
#         assert_eq!(expected, actual);
#     }
# }
```

Slices can be sliced! The syntax is `slice[low..high]`. If you omit the value on one of the sides of the `..`, it captures everything to that side of it. In our case, we are saying "take from 1 to the end" with `numbers[1..]`. You may wish to spend some time writing other tests around slices and experiment with the slice operator to get more familiar with it.

### Refactor

Not a lot to refactor this time.

What do you think would happen if you passed in an empty slice into our function? What is the "tail" of an empty slice? What happens when you tell Rust to capture all elements from `empty_slice[1..]`?

Just add a test for the case where the slice is empty.

### Write the Test First

```rust
# pub fn sum(numbers: &[i32]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
# pub fn sum_all_tails(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
#     let mut sums = Vec::with_capacity(numbers_to_sum.len());
#     for numbers in numbers_to_sum {
#         sums.push(sum(&numbers[1..]));
#     }
#     sums
# }
#
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

Oh, no! It's important to note that while the test has compiled, it has a runtime error.

Compile time errors are our friend because they help us write software that works, runtime errors are our enemies because they affect our users.

### Write Enough Code to Make It Pass

```rust
# pub fn sum(numbers: &[i32]) -> i32 {
#     let mut total = 0;
#     for number in numbers {
#         total += number;
#     }
#     total
# }
#
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
#
# #[cfg(test)]
# mod specs_for_sum_all_tails {
#     use super::sum_all_tails;
#
#     #[test]
#     fn sut_returns_sum_of_each_collection_in_vector_correctly() {
#         // Arrange
#         let numbers_1 = vec![1, 2, 3];
#         let numbers_2 = vec![0, 9, 10];
#
#         // Act
#         let actual = sum_all_tails(&[&numbers_1, &numbers_2]);
#
#         // Assert
#         let expected = vec![5, 19];
#         assert_eq!(expected, actual);
#     }
#
#     #[test]
#     fn sut_sets_summed_value_as_0_for_empty_collection() {
#         // Arrange
#         let numbers_1 = vec![];
#         let numbers_2 = vec![3, 4, 5];
#
#         // Act
#         let actual = sum_all_tails(&[&numbers_1, &numbers_2]);
#
#         // Assert
#         let expected = vec![0, 9];
#         assert_eq!(expected, actual);
#     }
# }
```

### Refactor

Nothing to refactor here. We have a working solution and the tests are passing. We can move on to the next challenge.

## Wrapping Up

We have covered:

- Arrays
- Slices
- Vectors
- Test coverage tool
- Unsized coercion
- A little bit of lifetimes

We've used slices, arrays and vectors with integers. But they work with any other types too including arrays/slices themselves, as we used `&[&[T]]`.

We also dealt with two special concepts of Rust - coercion and lifetimes. You could spend a lot of time learning about them, but for now, just know that they exist and you can look them up when you need to. The Rust book has a great section on [lifetimes](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html) and [coercion](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods). I recommend reading them when you have time.
