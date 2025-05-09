# Errors

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/errors).

We learned about structs in the last section which let us capture a number of values related around a concept.

At some point you may wish to use structs to manage state, exposing methods to let users change the state in a way that you can control.

Fintech loves Rust and uhhh bitcoins? So let's show what an amazing banking system we can make.

Let's make a Wallet struct which lets us deposit Bitcoin.

## The First Requirement: Deposit Bitcoin

### Write the Test First

```rust
#[cfg(test)]
mod specs_for_wallet {
    #[test]
    fn sut_deposits_correctly() {
        // Arrange
        let wallet = Wallet {};

        // Act
        wallet.deposit(100);
        let actual = wallet.balance();

        // Assert
        assert_eq!(100, actual);
    }
}
```

### Try to Run the Test

```bash
error[E0422]: cannot find struct, variant or union type `Wallet` in this scope
 --> src/lib.rs:7:22
  |
7 |         let wallet = Wallet {};
  |                      ^^^^^^ not found in this scope
```

In the [previous example](./structs_methods_and_traits.md) we accessed fields directly with the field name, however in our very secure wallet we don't want to expose our inner state to the rest of the world. We want to control access via methods.

We're good to run tests with a command `cargo test`, but we have another famous test platform called [nextest](https://nexte.st/) which is a drop-in replacement for the standard test framework. The main benefits of using nextest are:

- Faster test execution up to 3x, thanks to the parallelism described [here](https://nexte.st/docs/design/how-it-works/)
- Better test output
- Test subsets with filtering

The first benefit is meaningful on TDD workflow. While following TDD workflow, we have to run tests in many times. As running tests faster, we can get feedback faster. So we can write code faster.

We'll use nextest in this book for the following chapters. You can install it with the following command:

```bash
cargo install nextest
```

Then you can run the tests with the following command:

```bash
cargo nextest run
```

### Write the Minimal Amount of Code

The compiler doesn't know what a Wallet is so let's tell it not to see the compiler error.

```rust
pub struct Wallet {}

impl Wallet {
    pub fn deposit(&self, amount: u64) {}

    pub fn balance(&self) -> u64 {
        0
    }
}
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use super::Wallet;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let wallet = Wallet {};
#
#         // Act
#         wallet.deposit(100);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(100, actual);
#     }
# }
```

Now we can run the test again and see the meaningful error message.

```bash
    thread 'specs_for_wallet::sut_deposits_correctly' panicked at src/lib.rs:25:9:
    assertion `left == right` failed
      left: 100
     right: 0
```

### Write Enough Code to Make It Pass

We will need some kind of balance variable in our struct to store the state.

You might encounter some problems. The first one is that we need to make the `balance` field mutable in `deposit()` method. The second one is that we need to make the struct itself mutable in the test.

During writing a code, I also found that we can use a term `open` to create a new `Wallet` instance because we usually say `open an account` in the banking world. So I created a new method `open()` to create a new `Wallet` instance.

```rust
pub struct Wallet {
    balance: u64,
}

impl Wallet {
    pub fn open() -> Self {
        Self { balance: 0 }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }
}

#[cfg(test)]
mod specs_for_wallet {
    use super::Wallet;

    #[test]
    fn sut_deposits_correctly() {
        // Arrange
        let mut wallet = Wallet::open();

        // Act
        wallet.deposit(100);
        let actual = wallet.balance();

        // Assert
        assert_eq!(100, actual);
    }
}
```

Now, run the test again and see the result.

```bash
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
────────────
 Nextest run ID d882b99d-3227-48a1-9aa8-a16dc787b95a with nextest profile: default
    Starting 1 test across 1 binary
        PASS [   0.003s] pointers specs_for_wallet::sut_deposits_correctly
────────────
     Summary [   0.003s] 1 test run: 1 passed, 0 skipped
```

### Refactor

We said we were making a Bitcoin wallet but we have not mentioned them so far. We've been using `u64` because they're a good type for counting things! `u64` is fine in terms of the way it works but it's not descriptive.

Rust has a feature called [type alias](https://doc.rust-lang.org/book/ch20-03-advanced-types.html#creating-type-synonyms-with-type-aliases) which allows us to create a new name for an existing type. This is useful when you want to give a more meaningful name to a type, or when you want to create a type that is more specific than the original type.

The another way we can think of is using [new type pattern](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types) which is a wrapper around the existing type. This is useful when you want to add some domain specific functionality on top of existing types and hide internal implementation details.

For now, let's use type alias to make it simple.

```rust
pub type BitCoin = u64;

pub struct Wallet {
    balance: BitCoin,
}

impl Wallet {
    pub fn open() -> Self {
        Self { balance: 0 }
    }

    pub fn deposit(&mut self, amount: BitCoin) {
        self.balance += amount;
    }

    pub fn balance(&self) -> BitCoin {
        self.balance
    }
}

#[cfg(test)]
mod specs_for_wallet {
    use super::BitCoin;
    use super::Wallet;

    #[test]
    fn sut_deposits_correctly() {
        // Arrange
        let mut wallet = Wallet::open();
        let amount: BitCoin = 100;

        // Act
        wallet.deposit(amount);
        let actual = wallet.balance();

        // Assert
        assert_eq!(amount, actual);
    }
}
```

`BitCoin` is just a alias or synonym of `u64`, so we can use `BitCoin` just like `u64`.

One limitation of type alias is that we cannot implement external traits on it. For example, we cannot implement `Display` trait on `BitCoin` type. This is because `BitCoin` is just a alias of `u64`, and we cannot implement external traits on external types due to the [orphan rule](https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type).

If we have to add some domain specific functionality on top of existing types, we can use the new type pattern. This allows us to avoid the orphan rule and implement external traits on our own types.

## The Second Requirement: Withdraw Bitcoin

The requirement is for the wallet to be able to withdraw Bitcoin. Pretty much the opposite of `deposit()`, so let's write a test for it.

### Write the Test First

```rust
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
# impl Wallet {
#     pub fn open() -> Self {
#         Self { balance: 0 }
#     }
#
#     pub fn deposit(&mut self, amount: BitCoin) {
#         self.balance += amount;
#     }
#
#     pub fn balance(&self) -> BitCoin {
#         self.balance
#     }
# }
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use super::Wallet;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         let amount = 100;
#
#         // Act
#         wallet.deposit(amount);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(amount, actual);
#     }
#
    #[test]
    fn sut_withdraws_correctly() {
        // Arrange
        let mut wallet = Wallet::open();
        wallet.deposit(20);

        // Act
        wallet.withdraw(10);
        let actual = wallet.balance();

        // Assert
        let expected = 10;
        assert_eq!(expected, actual);
    }
# }
```

### Try to Run the Test

```bash
error[E0599]: no method named `withdraw` found for struct `Wallet` in the current scope
  --> src/lib.rs:49:16
   |
6  | pub struct Wallet {
   | ----------------- method `withdraw` not found for this struct
...
49 |         wallet.withdraw(10);
   |                ^^^^^^^^ method not found in `Wallet`
```

### Write the Minimal Amount of Code

```rust
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
impl Wallet {
    pub fn open() -> Self {
        Self { balance: 0 }
    }

    pub fn deposit(&mut self, amount: BitCoin) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: BitCoin) {}

    pub fn balance(&self) -> BitCoin {
        self.balance
    }
}
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use super::Wallet;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         let amount = 100;
#
#         // Act
#         wallet.deposit(amount);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(amount, actual);
#     }
#
#     #[test]
#     fn sut_withdraws_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         wallet.withdraw(10);
#         let actual = wallet.balance();
#
#         // Assert
#         let expected = 10;
#         assert_eq!(expected, actual);
#     }
# }
```

```bash
    thread 'specs_for_wallet::sut_withdraws_correctly' panicked at src/lib.rs:56:9:
    assertion `left == right` failed
      left: 10
     right: 20
```

### Write Enough Code to Make It Pass

```rust
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
impl Wallet {
    pub fn open() -> Self {
        Self { balance: 0 }
    }

    pub fn deposit(&mut self, amount: BitCoin) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: BitCoin) {
        self.balance -= amount;
    }

    pub fn balance(&self) -> BitCoin {
        self.balance
    }
}
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use super::Wallet;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         let amount = 100;
#
#         // Act
#         wallet.deposit(amount);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(amount, actual);
#     }
#
#     #[test]
#     fn sut_withdraws_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         wallet.withdraw(10);
#         let actual = wallet.balance();
#
#         // Assert
#         let expected = 10;
#         assert_eq!(expected, actual);
#     }
# }
```

### Refactor

Nothing to refactor now.

## The Third Requirement: Withdraw More Than Balance

What should happen if you try to Withdraw more than is left in the account? For now, our requirement is to assume there is not an overdraft facility.

How do we signal a problem when using Withdraw?

In Rust, if you want to indicate an error it is idiomatic for your function to return an `Result` for the caller to check and act on.

The `Result` type is an enum that can be either `Ok` or `Err`. The `Ok` variant contains the value returned by the function, and the `Err` variant contains an error value.

Let's try this out in a test.

### Write the Test First

```rust
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
# impl Wallet {
#     pub fn open() -> Self {
#         Self { balance: 0 }
#     }
#
#     pub fn deposit(&mut self, amount: BitCoin) {
#         self.balance += amount;
#     }
#
#     pub fn withdraw(&mut self, amount: BitCoin) {
#         self.balance -= amount;
#     }
#
#     pub fn balance(&self) -> BitCoin {
#         self.balance
#     }
# }
#
#[cfg(test)]
mod specs_for_wallet {
    use super::Wallet;

    #[test]
    fn sut_deposits_correctly() {
        // Arrange
        let mut wallet = Wallet::open();
        let amount = 100;

        // Act
        wallet.deposit(amount);
        let actual = wallet.balance();

        // Assert
        assert_eq!(amount, actual);
    }

    #[test]
    fn sut_withdraws_correctly() {
        // Arrange
        let mut wallet = Wallet::open();
        wallet.deposit(20);

        // Act
        wallet.withdraw(10);
        let actual = wallet.balance();

        // Assert
        let expected = 10;
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(20, 10)]
    #[case(20, 20)]
    fn sut_returns_ok_if_withdrawing_less_than_or_equal_balance(
        #[case] balance: BitCoin,
        #[case] amount: BitCoin,
    ) {
        // Arrange
        let mut wallet = Wallet::open();
        wallet.deposit(balance);

        // Act
        let actual = wallet.withdraw(amount);

        // Assert
        assert!(actual.is_ok());
    }

    #[test]
    fn sut_returns_error_if_withdrawing_more_than_balance() {
        // Arrange
        let mut wallet = Wallet::open();
        wallet.deposit(20);

        // Act
        let actual = wallet.withdraw(30);

        // Assert
        assert!(actual.is_err());
    }
}
```

There are two things to note here.

First, we've added a new test to check that the `withdraw` method returns an `Ok` result when the amount is less than or equal to the balance. You might think "why not just assert the result in the `sut_withdraws_correctly` test?". You can do this if you are accustomed to the business logic so that you are confident to have larger test cases. But if you are not sure about what you are doing, it is better to write test cases with a small scope.

For the case of `withdraw`, it has two responsibilities: to change the balance and to return a result. So we can separate them into two tests. The `sut_withdraws_correctly` test is only checking the balance change after the withdrawal. The `sut_returns_ok_if_withdrawing_less_than_or_equal_balance` test is checking the result of the withdrawal. This is a good practice to keep your tests small and focused. As I said before, you can combine them if you are confident about it.

Second, as the test name `sut_returns_ok_if_withdrawing_less_than_or_equal_balance` suggests, we have to test two cases: one for the case of withdrawing less than the balance and one for the case of withdrawing equal to the balance. To do that, as we learned in the [previous section](./structs_methods_and_traits.md), we can use parameterised tests.

### Try to Run the Test

```bash
error[E0599]: no method named `is_err` found for unit type `()` in the current scope
  --> src/lib.rs:73:24
   |
73 |         assert!(result.is_err());
   |                        ^^^^^^ method not found in `()`
```

The wording is perhaps a little unclear, but our previous intent with `withdraw` was just to call it, it will never return a value. To make this compile we will need to change it so it has a return type.

### Write the Minimal Amount of Code

```rust
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
# impl Wallet {
#     pub fn open() -> Self {
#         Self { balance: 0 }
#     }
#
#     pub fn deposit(&mut self, amount: BitCoin) {
#         self.balance += amount;
#     }
#
    pub fn withdraw(&mut self, amount: BitCoin) -> Result<(), String> {
        self.balance -= amount;
        Ok(())
    }
#
#     pub fn balance(&self) -> BitCoin {
#         self.balance
#     }
# }
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use rstest::rstest;
#
#     use super::BitCoin;
#     use super::Wallet;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         let amount = 100;
#
#         // Act
#         wallet.deposit(amount);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(amount, actual);
#     }
#
#     #[test]
#     fn sut_withdraws_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         wallet.withdraw(10);
#         let actual = wallet.balance();
#
#         // Assert
#         let expected = 10;
#         assert_eq!(expected, actual);
#     }
#
#     #[rstest]
#     #[case(20, 10)]
#     #[case(20, 20)]
#     fn sut_returns_ok_if_withdrawing_less_than_or_equal_balance(
#         #[case] balance: BitCoin,
#         #[case] amount: BitCoin,
#     ) {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(balance);
#
#         // Act
#         let actual = wallet.withdraw(amount);
#
#         // Assert
#         assert!(actual.is_ok());
#     }
#
#     #[test]
#     fn sut_returns_error_if_withdrawing_more_than_balance() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         let actual = wallet.withdraw(30);
#
#         // Assert
#         assert!(actual.is_err());
#     }
# }
```

Again, it is very important to just write enough code to satisfy the compiler. We correct our `withdraw` method to return error and for now we have to return something so let's just return `Ok(())`.

### Write Enough Code to Make It Pass

```rust
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
# impl Wallet {
#     pub fn open() -> Self {
#         Self { balance: 0 }
#     }
#
#     pub fn deposit(&mut self, amount: BitCoin) {
#         self.balance += amount;
#     }
#
    pub fn withdraw(&mut self, amount: BitCoin) -> Result<(), String> {
        if amount > self.balance {
            return Err(String::from("oh no"));
        }
        self.balance -= amount;
        Ok(())
    }
#
#     pub fn balance(&self) -> BitCoin {
#         self.balance
#     }
# }
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use rstest::rstest;
#
#     use super::BitCoin;
#     use super::Wallet;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         let amount = 100;
#
#         // Act
#         wallet.deposit(amount);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(amount, actual);
#     }
#
#     #[test]
#     fn sut_withdraws_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         wallet.withdraw(10);
#         let actual = wallet.balance();
#
#         // Assert
#         let expected = 10;
#         assert_eq!(expected, actual);
#     }
#
#     #[rstest]
#     #[case(20, 10)]
#     #[case(20, 20)]
#     fn sut_returns_ok_if_withdrawing_less_than_or_equal_balance(
#         #[case] balance: BitCoin,
#         #[case] amount: BitCoin,
#     ) {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(balance);
#
#         // Act
#         let actual = wallet.withdraw(amount);
#
#         // Assert
#         assert!(actual.is_ok());
#     }
#
#     #[test]
#     fn sut_returns_error_if_withdrawing_more_than_balance() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         let actual = wallet.withdraw(30);
#
#         // Assert
#         assert!(actual.is_err());
#     }
# }
```

### Refactor

If your linter `clippy` is enabled to check on the fly, you might see the following warning on `wallet.withdraw(10);` in the test `sut_withdraws_correctly`.

```bash
Diagnostics:
1. unused `std::result::Result` that must be used
   this `Result` may be an `Err` variant, which should be handled
   `#[warn(unused_must_use)]` on by default [unused_must_use]
2. use `let _ = ...` to ignore the resulting value: `let _ = ` [unused_must_use]
```

This is because we didn't handle the `Result` instance returned from `withdraw` method. This warning is useful because it reminds us that we should handle the error case. In this case, we can ignore the result because we are not interested in the result of the withdrawal. But in general, we should handle the result to avoid unexpected errors.

Let's fix this by using `let _ =` to ignore the result.

```rust
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
# impl Wallet {
#     pub fn open() -> Self {
#         Self { balance: 0 }
#     }
#
#     pub fn deposit(&mut self, amount: BitCoin) {
#         self.balance += amount;
#     }
#
#     pub fn withdraw(&mut self, amount: BitCoin) -> Result<(), String> {
#         if amount > self.balance {
#             return Err(String::from("oh no"));
#         }
#         self.balance -= amount;
#         Ok(())
#     }
#
#     pub fn balance(&self) -> BitCoin {
#         self.balance
#     }
# }
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use rstest::rstest;
#
#     use super::BitCoin;
#     use super::Wallet;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         let amount = 100;
#
#         // Act
#         wallet.deposit(amount);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(amount, actual);
#     }
#
    #[test]
    fn sut_withdraws_correctly() {
        // Arrange
        let mut wallet = Wallet::open();
        wallet.deposit(20);

        // Act
        _ = wallet.withdraw(10);
        let actual = wallet.balance();

        // Assert
        let expected = 10;
        assert_eq!(expected, actual);
    }
#
#     #[rstest]
#     #[case(20, 10)]
#     #[case(20, 20)]
#     fn sut_returns_ok_if_withdrawing_less_than_or_equal_balance(
#         #[case] balance: BitCoin,
#         #[case] amount: BitCoin,
#     ) {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(balance);
#
#         // Act
#         let actual = wallet.withdraw(amount);
#
#         // Assert
#         assert!(actual.is_ok());
#     }
#
#     #[test]
#     fn sut_returns_error_if_withdrawing_more_than_balance() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         let actual = wallet.withdraw(30);
#
#         // Assert
#         assert!(actual.is_err());
#     }
# }
```

## The Fourth Requirement: Enhanced Error Reporting

Hopefully when returning an error with string "oh no", you were thinking that we might iterate on that because it doesn't seem that useful to return.

The error gets returned to the caller. The caller should be able to handler differently based on the error types. For example, if the user tries to withdraw more than the balance, we need to report the details error message to the user. If there are temporal errors like network errors, the caller need to retry the operation. So we need to define the error types and return them to the caller.

There are several ways to define error types in Rust, and the most common way is to use an enum.

### Write the Test First

Update the test `sut_returns_error_if_withdrawing_more_than_balance` to check the error type and message.

```rust
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
# impl Wallet {
#     pub fn open() -> Self {
#         Self { balance: 0 }
#     }
#
#     pub fn deposit(&mut self, amount: BitCoin) {
#         self.balance += amount;
#     }
#
#     pub fn withdraw(&mut self, amount: BitCoin) -> Result<(), String> {
#         if amount > self.balance {
#             return Err(String::from("oh no"));
#         }
#         self.balance -= amount;
#         Ok(())
#     }
#
#     pub fn balance(&self) -> BitCoin {
#         self.balance
#     }
# }
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use rstest::rstest;
#
#     use super::BitCoin;
#     use super::Wallet;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         let amount = 100;
#
#         // Act
#         wallet.deposit(amount);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(amount, actual);
#     }
#
#     #[test]
#     fn sut_withdraws_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         _ = wallet.withdraw(10);
#         let actual = wallet.balance();
#
#         // Assert
#         let expected = 10;
#         assert_eq!(expected, actual);
#     }
#
#     #[rstest]
#     #[case(20, 10)]
#     #[case(20, 20)]
#     fn sut_returns_ok_if_withdrawing_less_than_or_equal_balance(
#         #[case] balance: BitCoin,
#         #[case] amount: BitCoin,
#     ) {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(balance);
#
#         // Act
#         let actual = wallet.withdraw(amount);
#
#         // Assert
#         assert!(actual.is_ok());
#     }
#
    #[test]
    fn sut_returns_error_if_withdrawing_more_than_balance() {
        // Arrange
        let mut wallet = Wallet::open();
        wallet.deposit(20);

        // Act
        let actual = wallet.withdraw(30).unwrap_err();

        // Assert
        assert!(matches!(actual, WalletError::InsufficientFunds(_),));
        assert_eq!(actual.to_string(), "cannot withdraw, insufficient funds");
    }
# }
```

In the test, we have used `wallet.withdraw(30).unwrap_err()` to get the error value from the `Result` type. If the `Result` is `Ok`, it will panic. If the `Result` is `Err`, it will return the error value. So the statement have two purposes: to assert that the result is error and to get the error value.

We also used `matches!` macro in assert stage to check if the error is of type `WalletError::InsufficientFunds`. [`matches!`](https://doc.rust-lang.org/std/macro.matches.html) macro returns whether the given expression matches the provided pattern. `matches!(actual, Wallet::InsufficientFunds(_))` checks if the `actual` value is of type `WalletError::InsufficientFunds`. The `_` is a wildcard pattern that matches any value. So we can use it to check if the error is of type `WalletError::InsufficientFunds` without caring about the value.

The error message value for providing an helpful message to users can be asserted with `assert_eq!(actual.to_string(), "cannot withdraw, insufficient funds")`. The `to_string()` method is implemented for the `WalletError` enum, so we can use it to get the string representation of the error.

We can use `assert_eq!(actual, WalletError::InsufficientFunds(String::from("cannot withdraw, insufficient funds")))` to check the error value in a single statement. Sure, we can do that! To compare the two values, we need to implement the `PartialEq` trait for the `WalletError` enum by deriving above the enum like `#[derive(PartialEq)]`. In my perspective, this is not a good practice to implement useless things only for testing. Enum comparison is not needed in the production code. In the caller side of the `withdraw` method, we might handle the error by pattern matching of error enum. So we don't need to implement the `PartialEq` trait for the `WalletError` enum. So I prefer to use `matches!` macro to check the error type and `to_string()` method to check the error message in this context.

### Try to Run the Test

```bash
  --> src/v4.rs:93:34
   |
93 |         assert!(matches!(actual, WalletError::InsufficientFunds(_)));
   |                                  ^^^^^^^^^^^ use of undeclared type `WalletError`
   |
help: consider importing this enum
```

### Write the Minimal Amount of Code

```rust,ignore
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
impl Wallet {
#     pub fn open() -> Self {
#         Self { balance: 0 }
#     }
#
#     pub fn deposit(&mut self, amount: BitCoin) {
#         self.balance += amount;
#     }
#
    pub fn withdraw(&mut self, amount: BitCoin) -> Result<(), WalletError> {
        if amount > self.balance {
            return Err(WalletError::InsufficientFunds(String::from("oh no")));
        }
        self.balance -= amount;
        Ok(())
    }
#
#     pub fn balance(&self) -> BitCoin {
#         self.balance
#     }
}

#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("{0}")]
    InsufficientFunds(String),
}
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use rstest::rstest;
#
#     use super::BitCoin;
#     use super::Wallet;
#     use super::WalletError;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         let amount = 100;
#
#         // Act
#         wallet.deposit(amount);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(amount, actual);
#     }
#
#     #[test]
#     fn sut_withdraws_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         _ = wallet.withdraw(10);
#         let actual = wallet.balance();
#
#         // Assert
#         let expected = 10;
#         assert_eq!(expected, actual);
#     }
#
#     #[rstest]
#     #[case(20, 10)]
#     #[case(20, 20)]
#     fn sut_returns_ok_if_withdrawing_less_than_or_equal_balance(
#         #[case] balance: BitCoin,
#         #[case] amount: BitCoin,
#     ) {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(balance);
#
#         // Act
#         let actual = wallet.withdraw(amount);
#
#         // Assert
#         assert!(actual.is_ok());
#     }
#
#     #[test]
#     fn sut_returns_error_if_withdrawing_more_than_balance() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         let actual = wallet.withdraw(30).unwrap_err();
#
#         // Assert
#         assert!(matches!(actual, WalletError::InsufficientFunds(_)));
#         assert_eq!(actual.to_string(), "cannot withdraw, insufficient funds");
#     }
# }
```

We declared an enum `WalletError` to represent the error types. We can declare the error as struct, but we choose enum because we would need to declare more errors in the future. For example, we might need to declare `InvalidAmountError` in the future. So we can use enum to represent multiple errors with a single type.

We also used [thiserror](https://github.com/dtolnay/thiserror) crate. You need to add the following line to your `Cargo.toml` file.

```toml
[dependencies]
thiserror = "2"
```

The thiserror crate provides a convenient way to define error types in Rust. If you declare error type without any help from external crates, you at least have to do the following things:

- Implement `Debug` trait for the error type
- Implement `Display` trait for the error type
- Implement `Error` trait for the error type

To make error type more advanced, you might need to implement all the details of `Error` trait. This makes our code more complex and harder to read. The `thiserror` crate provides a convenient way to derive these traits for you. So you can just use `#[derive(Debug, thiserror::Error)]` and use e.g. `#[error("{0}")]`, and thiserror crate will take care of the rest. To check the details, you can see [Error Handle in Rust](https://lpalmieri.com/posts/error-handling-rust/) blog post.

Now the test shows the meaningful error message.

```bash
    thread 'specs_for_wallet::sut_returns_error_if_withdrawing_more_than_balance' panicked at src/lib.rs:107:9:
    assertion `left == right` failed
      left: "oh no"
     right: "cannot withdraw, insufficient funds"
```

### Write Enough Code to Make It Pass

```rust,ignore
# pub type BitCoin = u64;
#
# pub struct Wallet {
#     balance: BitCoin,
# }
#
impl Wallet {
#     pub fn open() -> Self {
#         Self { balance: 0 }
#     }
#
#     pub fn deposit(&mut self, amount: BitCoin) {
#         self.balance += amount;
#     }
#
    pub fn withdraw(&mut self, amount: BitCoin) -> Result<(), WalletError> {
        if amount > self.balance {
            return Err(WalletError::InsufficientFunds(String::from(
                "cannot withdraw, insufficient funds",
            )));
        }
        self.balance -= amount;
        Ok(())
    }
#
#     pub fn balance(&self) -> BitCoin {
#         self.balance
#     }
}
#
# #[derive(Debug, thiserror::Error)]
# pub enum WalletError {
#     #[error("{0}")]
#     InsufficientFunds(String),
# }
#
# #[cfg(test)]
# mod specs_for_wallet {
#     use rstest::rstest;
#
#     use super::BitCoin;
#     use super::Wallet;
#     use super::WalletError;
#
#     #[test]
#     fn sut_deposits_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         let amount = 100;
#
#         // Act
#         wallet.deposit(amount);
#         let actual = wallet.balance();
#
#         // Assert
#         assert_eq!(amount, actual);
#     }
#
#     #[test]
#     fn sut_withdraws_correctly() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         _ = wallet.withdraw(10);
#         let actual = wallet.balance();
#
#         // Assert
#         let expected = 10;
#         assert_eq!(expected, actual);
#     }
#
#     #[rstest]
#     #[case(20, 10)]
#     #[case(20, 20)]
#     fn sut_returns_ok_if_withdrawing_less_than_or_equal_balance(
#         #[case] balance: BitCoin,
#         #[case] amount: BitCoin,
#     ) {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(balance);
#
#         // Act
#         let actual = wallet.withdraw(amount);
#
#         // Assert
#         assert!(actual.is_ok());
#     }
#
#     #[test]
#     fn sut_returns_error_if_withdrawing_more_than_balance() {
#         // Arrange
#         let mut wallet = Wallet::open();
#         wallet.deposit(20);
#
#         // Act
#         let actual = wallet.withdraw(30).unwrap_err();
#
#         // Assert
#         assert!(matches!(actual, WalletError::InsufficientFunds(_)));
#         assert_eq!(actual.to_string(), "cannot withdraw, insufficient funds");
#     }
# }
```

### Refactor

Nothing to refactor.

## Wrapping Up

### Type Alias and New Type Pattern

- Rust provides two ways to create new types: type alias and new type pattern
- Type alias is a synonym for an existing type
  - Useful when you want to give a more meaningful name
  - Cannot implement external traits to extend functionality
- New type pattern is a wrapper around an existing type
  - Useful when you want to add functionalities by implementing external traits

I recommend changing the code to use new type pattern instead of type alias after referencing [here](https://doc.rust-lang.org/rust-by-example/generics/new_types.html). You might need to implement `Add` and `AddAssign` traits as well.

### Errors

- Errors are the way to signify failure when calling a function/method
- Checking for a message written as string in an error would result in a flaky test
- Used enum and thiserror create to define error types
- This is not the end of the story with error handling, you can do more sophisticated things but this is just an intro
