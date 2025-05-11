# Mocking

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/mocking).

You have been asked to write a program which counts down from 3, printing each number on a new line (with a 1-second pause) and when it reaches zero it will print "Go!" and exit.

```bash
3
2
1
Go!
```

We'll tackle this by writing a function called Countdown which we will then put inside a main program so it looks something like this:

```rust,ignore
use mocking::countdown;

fn main() {
    countdown();
}
```

While this is a pretty trivial program, to test it fully we will need as always to take an iterative, test-driven approach.

What do I mean by iterative? We make sure we take the smallest steps we can to have useful software.

We don't want to spend a long time with code that will theoretically work after some hacking because that's often how developers fall down rabbit holes. It's an important skill to be able to slice up requirements as small as you can so you can have working software.

Here's how we can divide our work up and iterate on it:

- Print 3
- Print 3, 2, 1 and Go!
- Wait a second between each line

## The First Requirement: Print 3

### Write the Test First

Our software needs to print to stdout and we saw how we could use dependency injection to facilitate testing this in the DI section.

```rust
#[cfg(test)]
mod specs_for_countdown {
    use super::countdown;

    #[test]
    fn sut_writes_3() {
        // Arrange
        let mut buffer = Vec::new();

        // Act
        countdown(&mut buffer);

        // Assert
        assert_eq!(String::from_utf8(buffer).unwrap(), "3");
    }
}
```

If anything like buffer is unfamiliar to you, re-read [the previous section](./dependency_injection.md).

We know we want our `countdown` function to write data somewhere and `Write` is the de-facto way of capturing that as an trait in Rust.

- In main we will send to `Stdout` so our users see the countdown printed to the terminal
- In test we will send to `Vec<u8>` so our tests can capture what data is being generated

### Try and Run the Test

```bash
error[E0425]: cannot find function `countdown` in this scope
  --> src/lib.rs:17:9
   |
17 |         countdown(&mut buffer);
   |         ^^^^^^^^^ not found in this scope
```

### Write the Minimal Amount of Code

Define `countdown` and try again.

```rust,ignore
pub fn countdown(out: &mut dyn Write) {}
#
# #[cfg(test)]
# mod specs_for_countdown {
#     // use super::countdown;
#
#     #[test]
#     fn sut_writes_3() {
#         // Arrange
#         let mut buffer = Vec::new();
#
#         // Act
#         countdown(&mut buffer);
#
#         // Assert
#         assert_eq!(String::from_utf8(buffer).unwrap(), "3");
#     }
# }
```

```bash
    thread 'specs_for_countdown::sut_writes_3' panicked at src/lib.rs:22:9:
    assertion `left == right` failed
      left: ""
     right: "3"
```

Perfect!

### Write Enough Code to Make It Pass

```rust
# use std::io::Write;
#
pub fn countdown(out: &mut dyn Write) {
    out.write_all(b"3").unwrap();
}
#
# #[cfg(test)]
# mod specs_for_countdown {
#     use super::countdown;
#
#     #[test]
#     fn sut_writes_3() {
#         // Arrangk
#         let mut buffer = Vec::new();
#
#         // Act
#         countdown(&mut buffer);
#
#         // Assert
#         assert_eq!(String::from_utf8(buffer).unwrap(), "3");
#     }
# }
```

We're using `write_all` to write the bytes to the buffer and `unwrap` to panic if it fails. `write_all` is a method on `Write`, and it is similar to `write` but it will keep writing until all the bytes are written or an error occurs. `b"3"` is a byte string literal, which is a string literal that is represented as a sequence of bytes. It is similar to a regular string literal, but it is prefixed with a `b` and the type of the string is `&[u8]` instead of `&str`.

Re-run the tests and they should be passing.

### Refactor

To complete matters, let's now wire up our function into a `main`, so we have some working software to reassure ourselves we're making progress.

```rust,ignore
use std::io::stdout;

use mocking::countdown;

fn main() {
    countdown(&mut stdout());
}
```

Try and run the program, and be amazed at your handy work.

Yes, this seems trivial, but this approach is what I would recommend for any project. Take a thin slice of functionality and make it work end-to-end, backed by tests.

Next, we can make it print 2, 1 and then "Go!".

## The Second Requirement: Print 3, 2, 1 and Go

### Write the Test First

By investing in getting the overall plumbing working right, we can iterate on our solution safely and easily. We will no longer need to stop and re-run the program to be confident of it working as all the logic is tested.

```rust
# use std::io::Write;
#
# pub fn countdown(out: &mut dyn Write) {
#     out.write_all(b"3").unwrap();
# }
#
#[cfg(test)]
mod specs_for_countdown {
    use super::countdown;

    #[test]
    fn sut_writes_3_2_1_go() {
        // Arrangk
        let mut buffer = Vec::new();

        // Act
        countdown(&mut buffer);

        // Assert
        let actual = String::from_utf8(buffer).unwrap();
        let expected = "3\n2\n1\nGo!";
        assert_eq!(expected, actual);
    }
}
```

### Try and Run the Test

```bash
    thread 'specs_for_countdown::sut_writes_3_2_1_go' panicked at src/lib.rs:22:9:
    assertion `left == right` failed
      left: "3\n2\n1\nGo!"
     right: "3"
```

### Write Enough Code to Make It Pass

```rust
# use std::io::Write;
#
pub fn countdown(out: &mut dyn Write) {
    for i in (1..=3).rev() {
        out.write_all(format!("{}\n", i).as_bytes()).unwrap();
    }
    out.write_all(b"Go!").unwrap();
}
#
# #[cfg(test)]
# mod specs_for_countdown {
#     use super::countdown;
#
#     #[test]
#     fn sut_writes_3_2_1_go() {
#         // Arrangk
#         let mut buffer = Vec::new();
#
#         // Act
#         countdown(&mut buffer);
#
#         // Assert
#         let actual = String::from_utf8(buffer).unwrap();
#         let expected = "3\n2\n1\nGo!";
#         assert_eq!(expected, actual);
#     }
# }
```

Use a `for` loop counting and `rev` to reverse the order. `format!` is a macro that formats the string and returns it as a `String`. `as_bytes()` converts the string to a byte slice, which is what `write_all` needs. Finally, send the string "Go!" to the buffer.

### Refactor

There is not much to refactor other than refactoring some magic values into named constants.

```rust
use std::io::Write;

const COUNTDOWN_START: usize = 3;
const FINAL_WORD: &str = "Go!";

pub fn countdown(out: &mut dyn Write) {
    for i in (1..=COUNTDOWN_START).rev() {
        out.write_all(format!("{}\n", i).as_bytes()).unwrap();
    }
    out.write_all(FINAL_WORD.as_bytes()).unwrap();
}
#
# #[cfg(test)]
# mod specs_for_countdown {
#     use super::countdown;
#
#     #[test]
#     fn sut_writes_3_2_1_go() {
#         // Arrange
#         let mut buffer = Vec::new();
#
#         // Act
#         countdown(&mut buffer);
#
#         // Assert
#         let actual = String::from_utf8(buffer).unwrap();
#         let expected = "3\n2\n1\nGo!";
#         assert_eq!(expected, actual);
#     }
# }
```

## The Third Requirement: Wait a Second Between Each Line

If you run the program now, you should get the desired output but we don't have it as a dramatic countdown with the 1-second pauses.

Rust lets you achieve this with `sleep`. Try adding it in to our code.

```rust
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

const COUNTDOWN_START: usize = 3;
const FINAL_WORD: &str = "Go!";

pub fn countdown(out: &mut dyn Write) {
    for i in (1..=COUNTDOWN_START).rev() {
        out.write_all(format!("{}\n", i).as_bytes()).unwrap();
        sleep(Duration::from_secs(1));
    }
    out.write_all(FINAL_WORD.as_bytes()).unwrap();
}
```

If you run the program it works as we want it to.

### Mocking

The tests still pass and the software works as intended but we have some problems:

- Our tests take 3 seconds to run
  - Every forward-thinking post about software development emphasises the importance of quick feedback loops
  - Slow tests ruin developer productivity
  - Imagine if the requirements get more sophisticated warranting more tests
    - Are we happy with 3s added to the test run for every new test of Countdown?
- We have not tested an important property of our function.

We have a dependency on `sleep` which we need to extract so we can then control it in our tests.

If we can mock `sleep`, we can use dependency injection to use it instead of a "real" `sleep` and then we can spy on the calls to make assertions on them.

### Write the Test First

Let's define our dependency as an interface. This lets us then use a real Sleeper in `main` and a spy sleeper in our tests. By using an interface our `countdown` function is oblivious to this and adds some flexibility for the caller.

```rust
pub trait Sleeper {
    fn sleep(&self);
}
```

I made a design decision that our `countdown` function would not be responsible for how long the sleep is. This simplifies our code a little for now at least and means a user of our function can configure that sleepiness however they like.

Now we need to make a mock of it for our tests to use.

```rust
# #[cfg(test)]
# mod specs_for_countdown {
#     use std::cell::RefCell;
#
#     use super::Sleeper;
#     use super::countdown;
#
    struct SleeprSpy {
        calls: RefCell<usize>,
    }

    impl Sleeper for SleeprSpy {
        fn sleep(&self) {
            *self.calls.borrow_mut() += 1;
        }
    }
#
#     #[test]
#     fn sut_writes_3_2_1_go() {
#         // Arrange
#         let mut buffer = Vec::new();
#
#         // Act
#         countdown(&mut buffer);
#
#         // Assert
#         let actual = String::from_utf8(buffer).unwrap();
#         let expected = "3\n2\n1\nGo!";
#         assert_eq!(expected, actual);
#     }
# }
```

Spies are a kind of mock which can record how a dependency is used. They can record the arguments sent in, how many times it has been called, etc. In our case, we're keeping track of how many times `sleep` is called so we can check it in our test.

Update the tests to inject a dependency on our spy and assert that the sleep has been called 3 times.

```rust,ignore
# use std::io::Write;
#
# const COUNTDOWN_START: usize = 3;
# const FINAL_WORD: &str = "Go!";
#
# pub fn countdown(out: &mut dyn Write) {
#     for i in (1..=COUNTDOWN_START).rev() {
#         out.write_all(format!("{}\n", i).as_bytes()).unwrap();
#         sleep(Duration::from_secs(1));
#     }
#     out.write_all(FINAL_WORD.as_bytes()).unwrap();
# }
#
#[cfg(test)]
mod specs_for_countdown {
    use std::cell::RefCell;
    use std::io::stdout;

    use super::Sleeper;
    use super::countdown;

    struct SleeprSpy {
        calls: RefCell<usize>,
    }

    impl SleeprSpy {
        fn new() -> Self {
            SleeprSpy {
                calls: RefCell::new(0),
            }
        }
    }

    impl Sleeper for SleeprSpy {
        fn sleep(&self) {
            *self.calls.borrow_mut() += 1;
        }
    }

    #[test]
    fn sut_writes_3_2_1_go() {
        // Arrange
        let mut buffer = Vec::new();
        let sleeper_dummy = SleeprSpy::new();

        // Act
        countdown(&mut buffer, &sleeper_dummy);

        // Assert
        let actual = String::from_utf8(buffer).unwrap();
        let expected = "3\n2\n1\nGo!";
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_calls_sleep_3_times() {
        // Arrange
        let sleeper_spy = SleeprSpy::new();

        // Act
        countdown(&mut stdout(), &sleeper_spy);

        // Assert
        assert_eq!(*sleeper_spy.calls.borrow(), 3);
    }
}
```

We added a new test `sut_calls_sleep_3_times` which checks that the `sleep` method is called 3 times. We also added a new method `new` to the `SleeprSpy` struct to create a new instance of it.

In the test `sut_writes_3_2_1_go`, we named the instance of `SleepSpy` as `sleeper_dummy` to make it clear that it is a dummy implementation. In this test, the instance does nothing but is able to call `countdown` with it.

If you're wondering the difference between spy and dummy, I recommend reading [this article](https://martinfowler.com/articles/mocksArentStubs.html). You can do distinguish dummy, fake, stub, spy and mock.

### Try and Run the Test

You will see an error message that function expects 1 argument but got 2.

### Write the Minimal Amount of Code

We need to update `countdown` to accept our `Sleeper` trait object.

```rust
# use std::io::Write;
# use std::thread::sleep;
# use std::time::Duration;
#
# const COUNTDOWN_START: usize = 3;
# const FINAL_WORD: &str = "Go!";
#
# pub trait Sleeper {
#     fn sleep(&self);
# }
#
pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
    for i in (1..=COUNTDOWN_START).rev() {
        out.write_all(format!("{}\n", i).as_bytes()).unwrap();
        sleep(Duration::from_secs(1));
    }
    out.write_all(FINAL_WORD.as_bytes()).unwrap();
}
#
# #[cfg(test)]
# mod specs_for_countdown {
#     use std::cell::RefCell;
#     use std::io::stdout;
#
#     use super::Sleeper;
#     use super::countdown;
#
#     struct SleeprSpy {
#         calls: RefCell<usize>,
#     }
#
#     impl SleeprSpy {
#         fn new() -> Self {
#             SleeprSpy {
#                 calls: RefCell::new(0),
#             }
#         }
#     }
#
#     impl Sleeper for SleeprSpy {
#         fn sleep(&self) {
#             *self.calls.borrow_mut() += 1;
#         }
#     }
#
#     #[test]
#     fn sut_writes_3_2_1_go() {
#         // Arrange
#         let mut buffer = Vec::new();
#         let sleeper_dummy = SleeprSpy::new();
#
#         // Act
#         countdown(&mut buffer, &sleeper_dummy);
#
#         // Assert
#         let actual = String::from_utf8(buffer).unwrap();
#         let expected = "3\n2\n1\nGo!";
#         assert_eq!(expected, actual);
#     }
#
#     #[test]
#     fn sut_calls_sleep_3_times() {
#         // Arrange
#         let sleeper_spy = SleeprSpy::new();
#
#         // Act
#         countdown(&mut stdout(), &sleeper_spy);
#
#         // Assert
#         assert_eq!(*sleeper_spy.calls.borrow(), 3);
#     }
# }
```

If you try again, your `main` will no longer compile for the same reason. Let's create a real sleeper which implements the trait we need.

```rust,ignore
pub trait Sleeper {
    fn sleep(&self);
}

pub struct DefaultSleeper;

impl Sleeper for DefaultSleeper {
    fn sleep(&self) {
        sleep(Duration::from_secs(1));
    }
}
```

```rust,ignore
use std::io::stdout;

use mocking::DefaultSleeper;
use mocking::countdown;

fn main() {
    countdown(&mut stdout(), &DefaultSleeper);
}
```

### Write Enough Code to Make It Pass

The test is now compiling but not passing because we're still calling the time.Sleep rather than the injected in dependency. Let's fix that.

```rust,ignore
# const COUNTDOWN_START: usize = 3;
# const FINAL_WORD: &str = "Go!";
#
# pub trait Sleeper {
#     fn sleep(&self);
# }
#
# pub struct DefaultSleeper;
#
# impl Sleeper for DefaultSleeper {
#     fn sleep(&self) {
#         sleep(Duration::from_secs(1));
#     }
# }
#
pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
    for i in (1..=COUNTDOWN_START).rev() {
        out.write_all(format!("{}\n", i).as_bytes()).unwrap();
        sleeper.sleep();
    }
    out.write_all(FINAL_WORD.as_bytes()).unwrap();
}
#
# #[cfg(test)]
# mod specs_for_countdown {
#     use std::cell::RefCell;
#     use std::io::stdout;
#
#     use super::Sleeper;
#     use super::countdown;
#
#     struct SleeprSpy {
#         calls: RefCell<usize>,
#     }
#
#     impl SleeprSpy {
#         fn new() -> Self {
#             SleeprSpy {
#                 calls: RefCell::new(0),
#             }
#         }
#     }
#
#     impl Sleeper for SleeprSpy {
#         fn sleep(&self) {
#             *self.calls.borrow_mut() += 1;
#         }
#     }
#
#     #[test]
#     fn sut_writes_3_2_1_go() {
#         // Arrange
#         let mut buffer = Vec::new();
#         let sleeper_dummy = SleeprSpy::new();
#
#         // Act
#         countdown(&mut buffer, &sleeper_dummy);
#
#         // Assert
#         let actual = String::from_utf8(buffer).unwrap();
#         let expected = "3\n2\n1\nGo!";
#         assert_eq!(expected, actual);
#     }
#
#     #[test]
#     fn sut_calls_sleep_3_times() {
#         // Arrange
#         let sleeper_spy = SleeprSpy::new();
#
#         // Act
#         countdown(&mut stdout(), &sleeper_spy);
#
#         // Assert
#         assert_eq!(*sleeper_spy.calls.borrow(), 3);
#     }
# }
```

The test should pass and no longer take 3 seconds.

### Remaining Problems

There's still another important property we haven't tested. Countdown should sleep before each next print, e.g:

- Print N
- Sleep
- Print N-1
- Sleep
- ..
- Print Go!

Our latest change only asserts that it has slept 3 times, but those sleeps could occur out of sequence.

When writing tests if you're not confident that your tests are giving you sufficient confidence, just break it! (make sure you have committed your changes to source control first though). Change the code to the following.

```rust,ignore
pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
    for i in (1..=COUNTDOWN_START).rev() {
        out.write_all(format!("{}\n", i).as_bytes()).unwrap();
    }
    for _ in (1..=COUNTDOWN_START).rev() {
        sleeper.sleep();
    }
    out.write_all(FINAL_WORD.as_bytes()).unwrap();
}
```

If you run your tests they should still be passing even though the implementation is wrong.

Let's use spying again with a new test to check the order of operations is correct.

We have two different dependencies and we want to record all of their operations into one list. So we'll create one spy for them both.

```rust,ignore
# use std::io::Write;
#
# const COUNTDOWN_START: usize = 3;
# const FINAL_WORD: &str = "Go!";
#
# pub trait Sleeper {
#     fn sleep(&self);
# }
#
# pub struct DefaultSleeper;
#
# impl Sleeper for DefaultSleeper {
#     fn sleep(&self) {
#         sleep(Duration::from_secs(1));
#     }
# }
#
# pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
#     for i in (1..=COUNTDOWN_START).rev() {
#         out.write_all(format!("{}\n", i).as_bytes()).unwrap();
#         sleeper.sleep();
#     }
#     out.write_all(FINAL_WORD.as_bytes()).unwrap();
# }
#
#[cfg(test)]
mod specs_for_countdown {
    use std::cell::RefCell;
    use std::io::Write;
    use std::io::stdout;
    use std::thread::sleep;
    use std::time::Duration;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use super::Sleeper;
    use super::countdown;

#     struct SleeprSpy {
#         calls: RefCell<usize>,
#     }
#
#     impl SleeprSpy {
#         fn new() -> Self {
#             SleeprSpy {
#                 calls: RefCell::new(0),
#             }
#         }
#     }
#
#     impl Sleeper for SleeprSpy {
#         fn sleep(&self) {
#             *self.calls.borrow_mut() += 1;
#         }
#     }
#
    struct CountdownOperationsSpy {
        sleep_command: &'static str,
        write_command: &'static str,
        calls: RefCell<Vec<(u128, &'static str)>>,
    }

    impl CountdownOperationsSpy {
        fn new() -> Self {
            CountdownOperationsSpy {
                sleep_command: "sleep",
                write_command: "write",
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl Sleeper for CountdownOperationsSpy {
        fn sleep(&self) {
            let mut calls = self.calls.borrow_mut();
            calls.push((get_current_timestamp(), self.sleep_command));
            sleep(Duration::from_millis(1));
        }
    }

    impl Write for CountdownOperationsSpy {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut calls = self.calls.borrow_mut();
            calls.push((get_current_timestamp(), self.write_command));
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn get_current_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
#
#     #[test]
#     fn sut_writes_3_2_1_go() {
#         // Arrange
#         let mut buffer = Vec::new();
#         let sleeper_dummy = SleeprSpy::new();
#
#         // Act
#         countdown(&mut buffer, &sleeper_dummy);
#
#         // Assert
#         let actual = String::from_utf8(buffer).unwrap();
#         let expected = "3\n2\n1\nGo!";
#         assert_eq!(expected, actual);
#     }
#
#     #[test]
#     fn sut_calls_sleep_3_times() {
#         // Arrange
#         let sleeper_spy = SleeprSpy::new();
#
#         // Act
#         countdown(&mut stdout(), &sleeper_spy);
#
#         // Assert
#         assert_eq!(*sleeper_spy.calls.borrow(), 3);
#     }
}
```

Our `CountdownOperationsSpy` implements both `Sleeper` and `Write`, recording every call with timestamp into one vector. In this test, we're only concerned about the order of operations, so just recording them as list of named operations is sufficient.

We can now add a test into our test suite which verifies our sleeps and prints operate in the order we expect.

```rust,ignore
# use std::io::Write;
#
# const COUNTDOWN_START: usize = 3;
# const FINAL_WORD: &str = "Go!";
#
# pub trait Sleeper {
#     fn sleep(&self);
# }
#
# pub struct DefaultSleeper;
#
# impl Sleeper for DefaultSleeper {
#     fn sleep(&self) {
#         sleep(Duration::from_secs(1));
#     }
# }
#
# pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
#     for i in (1..=COUNTDOWN_START).rev() {
#         out.write_all(format!("{}\n", i).as_bytes()).unwrap();
#         sleeper.sleep();
#     }
#     out.write_all(FINAL_WORD.as_bytes()).unwrap();
# }
#
# #[cfg(test)]
# mod specs_for_countdown {
#     use std::cell::RefCell;
#     use std::io::Write;
#     use std::io::stdout;
#     use std::thread::sleep;
#     use std::time::Duration;
#     use std::time::SystemTime;
#     use std::time::UNIX_EPOCH;
#
#     use super::Sleeper;
#     use super::countdown;
#
#     struct SleeprSpy {
#         calls: RefCell<usize>,
#     }
#
#     impl SleeprSpy {
#         fn new() -> Self {
#             SleeprSpy {
#                 calls: RefCell::new(0),
#             }
#         }
#     }
#
#     impl Sleeper for SleeprSpy {
#         fn sleep(&self) {
#             *self.calls.borrow_mut() += 1;
#         }
#     }
#
#     struct CountdownOperationsSpy {
#         sleep_command: &'static str,
#         write_command: &'static str,
#         calls: RefCell<Vec<(u128, &'static str)>>,
#     }
#
#     impl CountdownOperationsSpy {
#         fn new() -> Self {
#             CountdownOperationsSpy {
#                 sleep_command: "sleep",
#                 write_command: "write",
#                 calls: RefCell::new(Vec::new()),
#             }
#         }
#     }
#
#     impl Sleeper for CountdownOperationsSpy {
#         fn sleep(&self) {
#             let mut calls = self.calls.borrow_mut();
#             calls.push((get_current_timestamp(), self.sleep_command));
#             sleep(Duration::from_millis(1));
#         }
#     }
#
#     impl Write for CountdownOperationsSpy {
#         fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
#             let mut calls = self.calls.borrow_mut();
#             calls.push((get_current_timestamp(), self.write_command));
#             Ok(buf.len())
#         }
#
#         fn flush(&mut self) -> std::io::Result<()> {
#             Ok(())
#         }
#     }
#
#     fn get_current_timestamp() -> u128 {
#         SystemTime::now()
#             .duration_since(UNIX_EPOCH)
#             .unwrap()
#             .as_nanos()
#     }
#
#     #[test]
#     fn sut_writes_3_2_1_go() {
#         // Arrange
#         let mut buffer = Vec::new();
#         let sleeper_dummy = SleeprSpy::new();
#
#         // Act
#         countdown(&mut buffer, &sleeper_dummy);
#
#         // Assert
#         let actual = String::from_utf8(buffer).unwrap();
#         let expected = "3\n2\n1\nGo!";
#         assert_eq!(expected, actual);
#     }
#
#     #[test]
#     fn sut_calls_sleep_3_times() {
#         // Arrange
#         let sleeper_spy = SleeprSpy::new();
#
#         // Act
#         countdown(&mut stdout(), &sleeper_spy);
#
#         // Assert
#         assert_eq!(*sleeper_spy.calls.borrow(), 3);
#     }
#
    #[test]
    fn sut_sleeps_after_writing() {
        // Arrange
        let sleeper_spy = CountdownOperationsSpy::new();
        let mut writer_spy = CountdownOperationsSpy::new();

        // Act
        countdown(&mut writer_spy, &sleeper_spy);

        // Assert
        let merge_operations = |spy_1: CountdownOperationsSpy, spy_2: CountdownOperationsSpy| {
            let mut operations = spy_1.calls.borrow_mut().clone();
            operations.extend(spy_2.calls.borrow_mut().clone());
            operations.sort_by_key(|key| key.0);
            operations
        };
        let operations = merge_operations(writer_spy, sleeper_spy);
        let actual: Vec<&str> = operations.into_iter().map(|(_, command)| command).collect();
        let expected = vec![
            "write", "sleep", "write", "sleep", "write", "sleep", "write",
        ];
        assert_eq!(expected, actual);
    }
# }
```

This test should now fail. Revert `countdown` back to how it was to fix the test.

We now have two tests spying on the `Sleeper`, so we can now refactor our test so one is testing what is being printed and the other one is ensuring we're sleeping between the prints. Finally, we can delete our first spy it is not used anymore.

```rust,ignore
# use std::io::Write;
#
# const COUNTDOWN_START: usize = 3;
# const FINAL_WORD: &str = "Go!";
#
# pub trait Sleeper {
#     fn sleep(&self);
# }
#
# pub struct DefaultSleeper;
#
# impl Sleeper for DefaultSleeper {
#     fn sleep(&self) {
#         sleep(Duration::from_secs(1));
#     }
# }
#
# pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
#     for i in (1..=COUNTDOWN_START).rev() {
#         out.write_all(format!("{}\n", i).as_bytes()).unwrap();
#         sleeper.sleep();
#     }
#     out.write_all(FINAL_WORD.as_bytes()).unwrap();
# }
#
#[cfg(test)]
mod specs_for_countdown {
    use std::cell::RefCell;
    use std::io::Write;
    use std::io::stdout;
    use std::thread::sleep;
    use std::time::Duration;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use super::Sleeper;
    use super::countdown;

    struct CountdownOperationsSpy {
        sleep_command: &'static str,
        write_command: &'static str,
        calls: RefCell<Vec<(u128, &'static str)>>,
    }

    impl CountdownOperationsSpy {
        fn new() -> Self {
            CountdownOperationsSpy {
                sleep_command: "sleep",
                write_command: "write",
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl Sleeper for CountdownOperationsSpy {
        fn sleep(&self) {
            let mut calls = self.calls.borrow_mut();
            calls.push((get_current_timestamp(), self.sleep_command));
            sleep(Duration::from_millis(1));
        }
    }

    impl Write for CountdownOperationsSpy {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut calls = self.calls.borrow_mut();
            calls.push((get_current_timestamp(), self.write_command));
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn get_current_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }

    #[test]
    fn sut_writes_3_2_1_go() {
        // Arrange
        let mut buffer = Vec::new();
        let sleeper_dummy = CountdownOperationsSpy::new();

        // Act
        countdown(&mut buffer, &sleeper_dummy);

        // Assert
        let actual = String::from_utf8(buffer).unwrap();
        let expected = "3\n2\n1\nGo!";
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_calls_sleep_3_times() {
        // Arrange
        let sleeper_spy = CountdownOperationsSpy::new();

        // Act
        countdown(&mut stdout(), &sleeper_spy);

        // Assert
        assert_eq!(sleeper_spy.calls.borrow().len(), 3);
    }

    #[test]
    fn sut_sleeps_after_writing() {
        // Arrange
        let sleeper_spy = CountdownOperationsSpy::new();
        let mut writer_spy = CountdownOperationsSpy::new();

        // Act
        countdown(&mut writer_spy, &sleeper_spy);

        // Assert
        let merge_operations = |spy_1: CountdownOperationsSpy, spy_2: CountdownOperationsSpy| {
            let mut operations = spy_1.calls.borrow_mut().clone();
            operations.extend(spy_2.calls.borrow_mut().clone());
            operations.sort_by_key(|key| key.0);
            operations
        };
        let operations = merge_operations(writer_spy, sleeper_spy);
        let actual: Vec<&str> = operations.into_iter().map(|(_, command)| command).collect();
        let expected = vec![
            "write", "sleep", "write", "sleep", "write", "sleep", "write",
        ];
        assert_eq!(expected, actual);
    }
}
```

We now have our function and its 3 important properties properly tested.

## The Fourth Requirement: Configurable Sleeper

A nice feature would be for the Sleeper to be configurable. This means that we can adjust the sleep time in our main program.

### Write the Test First

Let's first create a new type for `ConfigurableSleeper` that accepts what we need for configuration and testing.

```rust,ignore
pub struct ConfigurableSleeper {
    duration: Duration,
    sleep_function: Box<dyn Fn(Duration)>,
}

impl ConfigurableSleeper {
    pub fn new(duration: Duration, sleep_function: impl Fn(Duration) + 'static) -> Self {
        ConfigurableSleeper {
            duration,
            sleep_function: Box::new(sleep_function),
        }
    }
}
```

We are using duration to configure the time slept and sleep as a way to pass in a sleep function. We can put spy functions in here to test the sleep time.

It looks a bit difficult, so let me explain it a bit.

The `sleep_function` in the struct is `Box<dyn Fn(Duration)>`. Type of properties in struct must be known at compile time, but `Fn` is a trait and we don't know what type it will be. So we make it as trait object as `dyn Fn(Duration)`. After that use `Box` to put it on the heap, so that the type and size is known at compile time.

If you want to know more about `Fn`, `FnMut` and `FnOnce`, I recommend reading the Rust Book [here](https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits).

In the `new` construction method, we are using `impl Fn(Duration) + 'static` to accept any type that implements the `Fn` trait and has a static lifetime. The function trait object must outlive the struct, so we need to use `'static` to ensure that it does.

Now we can add a test to check that the `ConfigurableSleeper.sleep` sends the correct duration to the sleep function by injecting a spy function.

```rust
# use std::io::Write;
# use std::time::Duration;
#
# const COUNTDOWN_START: usize = 3;
# const FINAL_WORD: &str = "Go!";
#
# pub trait Sleeper {
#     fn sleep(&self);
# }
#
# pub struct ConfigurableSleeper {
#     duration: Duration,
#     sleep_function: Box<dyn Fn(Duration)>,
# }
#
# impl ConfigurableSleeper {
#     pub fn new(duration: Duration, sleep_function: impl Fn(Duration) + 'static) -> Self {
#         ConfigurableSleeper {
#             duration,
#             sleep_function: Box::new(sleep_function),
#         }
#     }
# }
#
# pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
#     for i in (1..=COUNTDOWN_START).rev() {
#         out.write_all(format!("{}\n", i).as_bytes()).unwrap();
#         sleeper.sleep();
#     }
#     out.write_all(FINAL_WORD.as_bytes()).unwrap();
# }
#
#[cfg(test)]
mod specs_for_configurable_sleeper {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::time::Duration;

    use super::ConfigurableSleeper;
    use super::Sleeper;

    #[test]
    fn sut_delivers_duration_to_sleep_function_correctly() {
        // Arrange
        let recorder = Rc::new(RefCell::new(Duration::from_secs(0)));
        let duration_spy = recorder.clone();
        let sut = ConfigurableSleeper::new(Duration::from_secs(5), move |duration| {
            *duration_spy.borrow_mut() = duration;
        });

        // Act
        sut.sleep();

        // Assert
        let actual = recorder.take();
        let expected = Duration::from_secs(5);
        assert_eq!(expected, actual);
    }
}
```

It gets a bit tricky here. We need to put a function as a property of the struct, and we need to spy which argument is passed to the function. We cannot do this with a normal function, so we have to use a closure to record the argument.

With the test above, we created a variable `recorder` to record the argument passed to the function. It is wrapped with `Rc` and `RefCell` to allow multiple ownership and interior mutability. Why do we need that? Because the sleep function signature is `Fn(Duration)` which means no mutation is allowed. So we need to use `RefCell` to allow mutation while maintaining the function signature. And we have to use `Rc` to allow multiple ownership for a closure in the function and for getting the value as a spy. If you remove `Rc` and `RefCell` one by one, you will see the compiler's error messages and you will understand why we need them.

To be honest, I couldn't write the test right away. I had to try several times to get it right while implementing the code and fixing the test code. I recommend you to try it out and see how it works. It is a bit tricky, but once you understand it, it will be easier to write tests for similar cases in the future.

### Write the Minimal Amount of Code

```rust
# use std::io::Write;
# use std::time::Duration;
#
# const COUNTDOWN_START: usize = 3;
# const FINAL_WORD: &str = "Go!";
#
# pub trait Sleeper {
#     fn sleep(&self);
# }
#
# pub struct ConfigurableSleeper {
#     duration: Duration,
#     sleep_function: Box<dyn Fn(Duration)>,
# }
#
# impl ConfigurableSleeper {
#     pub fn new(duration: Duration, sleep_function: impl Fn(Duration) + 'static) -> Self {
#         ConfigurableSleeper {
#             duration,
#             sleep_function: Box::new(sleep_function),
#         }
#     }
# }
#
impl Sleeper for ConfigurableSleeper {
    fn sleep(&self) {}
}
#
# pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
#     for i in (1..=COUNTDOWN_START).rev() {
#         out.write_all(format!("{}\n", i).as_bytes()).unwrap();
#         sleeper.sleep();
#     }
#     out.write_all(FINAL_WORD.as_bytes()).unwrap();
# }
#
# #[cfg(test)]
# mod specs_for_configurable_sleeper {
#     use std::cell::RefCell;
#     use std::rc::Rc;
#     use std::time::Duration;
#
#     use super::ConfigurableSleeper;
#     use super::Sleeper;
#
#     #[test]
#     fn sut_delivers_duration_to_sleep_function_correctly() {
#         // Arrange
#         let recorder = Rc::new(RefCell::new(Duration::from_secs(0)));
#         let duration_spy = recorder.clone();
#         let sut = ConfigurableSleeper::new(Duration::from_secs(5), move |duration| {
#             *duration_spy.borrow_mut() = duration;
#         });
#
#         // Act
#         sut.sleep();
#
#         // Assert
#         let actual = recorder.take();
#         let expected = Duration::from_secs(5);
#         assert_eq!(expected, actual);
#     }
# }
```

With our new `sleep` function implemented, we have a failing test.

```bash
    thread 'specs_for_configurable_sleeper::sut_delivers_duration_to_sleep_function_correctly' panicked at src/lib.rs:69:9:
    assertion `left == right` failed
      left: 5s
     right: 0ns
```

### Write Enough Code to Make It Pass

All we need to do now is implementing the `sleep` function for `ConfigurableSleeper`.

```rust
# use std::io::Write;
# use std::time::Duration;
#
# const COUNTDOWN_START: usize = 3;
# const FINAL_WORD: &str = "Go!";
#
# pub trait Sleeper {
#     fn sleep(&self);
# }
#
# pub struct ConfigurableSleeper {
#     duration: Duration,
#     sleep_function: Box<dyn Fn(Duration)>,
# }
#
# impl ConfigurableSleeper {
#     pub fn new(duration: Duration, sleep_function: impl Fn(Duration) + 'static) -> Self {
#         ConfigurableSleeper {
#             duration,
#             sleep_function: Box::new(sleep_function),
#         }
#     }
# }
#
impl Sleeper for ConfigurableSleeper {
    fn sleep(&self) {
        (self.sleep_function)(self.duration);
    }
}
#
# pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
#     for i in (1..=COUNTDOWN_START).rev() {
#         out.write_all(format!("{}\n", i).as_bytes()).unwrap();
#         sleeper.sleep();
#     }
#     out.write_all(FINAL_WORD.as_bytes()).unwrap();
# }
#
# #[cfg(test)]
# mod specs_for_configurable_sleeper {
#     use std::cell::RefCell;
#     use std::rc::Rc;
#     use std::time::Duration;
#
#     use super::ConfigurableSleeper;
#     use super::Sleeper;
#
#     #[test]
#     fn sut_delivers_duration_to_sleep_function_correctly() {
#         // Arrange
#         let recorder = Rc::new(RefCell::new(Duration::from_secs(0)));
#         let duration_spy = recorder.clone();
#         let sut = ConfigurableSleeper::new(Duration::from_secs(5), move |duration| {
#             *duration_spy.borrow_mut() = duration;
#         });
#
#         // Act
#         sut.sleep();
#
#         // Assert
#         let actual = recorder.take();
#         let expected = Duration::from_secs(5);
#         assert_eq!(expected, actual);
#     }
# }
```

With this change all of the tests should be passing again and you might wonder why all the hassle as the main program didn't change at all. Hopefully it becomes clear after the following section.

### Cleanup and Refactor

The last thing we need to do is to actually use our `ConfigurableSleeper` in the main function.

```rust,ignore
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use mocking::ConfigurableSleeper;
use mocking::countdown;

fn main() {
    countdown(
        &mut stdout(),
        &ConfigurableSleeper::new(
            Duration::from_secs(1),
            Box::new(|duration| {
                sleep(duration);
            }),
        ),
    );
}
```

If we run the tests and the program manually, we can see that all the behavior remains the same.

Since we are using the `ConfigurableSleeper`, it is now safe to delete the `DefaultSleeper` implementation. Wrapping up our program and having a more generic Sleeper with arbitrary long countdowns.

## Mocking, Evil?

### Isn't Mocking Evil?

You may have heard mocking is evil. Just like anything in software development it can be used for evil, just like [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself).

People normally get in to a bad state when they don't listen to their tests and are not respecting the refactoring stage.

If your mocking code is becoming complicated or you are having to mock out lots of things to test something, you should listen to that bad feeling and think about your code. Usually it is a sign of

- The thing you are testing is having to do too many things (because it has too many dependencies to mock)
  - Break the module apart so it does less
- Its dependencies are too fine-grained
  - Think about how you can consolidate some of these dependencies into one meaningful module
- Your test is too concerned with implementation details
  - Favour testing expected behaviour rather than the implementation

Normally a lot of mocking points to bad abstraction in your code.

What people see here is a weakness in TDD but it is actually a strength, more often than not poor test code is a result of bad design or put more nicely, well-designed code is easy to test.

### Mocks and Tests are Still Making My Life Hard

Ever run into this situation?

- You want to do some refactoring
- To do this you end up changing lots of tests
- You question TDD and make a post on Medium titled "Mocking considered harmful"

This is usually a sign of you testing too much implementation detail. Try to make it so your tests are testing useful behaviour unless the implementation is really important to how the system runs.

It is sometimes hard to know what level to test exactly but here are some thought processes and rules I try to follow.

#### First Rule: Are you testing the implementation or the behaviour?

The definition of refactoring is that the code changes but the behaviour stays the same. If you have decided to do some refactoring in theory you should be able to make the commit without any test changes. So when writing a test ask yourself:

- Am I testing the behaviour I want, or the implementation details?
- If I were to refactor this code, would I have to make lots of changes to the tests?

#### Second Rule: Are you testing private functions?

Although Go lets you test private functions, I would avoid it as private functions are implementation detail to support public behaviour. Test the public behaviour. Sandi Metz describes private functions as being "less stable" and you don't want to couple your tests to them.

#### Third Rule: Are you handling three or more mocks?

I feel like if a test is working with more than three mocks then it is a red flag - time for a rethink on the design.

#### Fourth Rule: Are you using spies to test implementation details?

Use spies with caution. Spies let you see the insides of the algorithm you are writing which can be very useful but that means a tighter coupling between your test code and the implementation. Be sure you actually care about these details if you're going to spy on them.

### Can't Just Use a Mocking Framework?

Mocking requires no magic and is relatively simple; using a framework can make mocking seem more complicated than it is. We don't use automatic mocking in this chapter so that we get:

- a better understanding of how to mock
- practice implementing interfaces

In collaborative projects, there is value in auto-generating mocks. In a team, a mock generation tool codifies consistency around the test doubles. This will avoid inconsistently written test doubles which can translate to inconsistently written tests.

You should only use a mock generator that generates test doubles against an interface. Any tool that overly dictates how tests are written, or that use lots of 'magic', can get in the sea.

## Wrapping Up

### More on TDD Approach

When faced with less trivial examples, break the problem down into "thin vertical slices". Try to get to a point where you have working software backed by tests as soon as you can, to avoid getting in rabbit holes and taking a "big bang" approach.

Once you have some working software it should be easier to iterate with small steps until you arrive at the software you need.

> When to use iterative development? You should use iterative development only on projects that you want to succeed. From Martin Fowler.

### Mocking

Without mocking, important areas of your code will be untested. In our case we would not be able to test that our code paused between each print but there are countless other examples. Calling a service that can fail? Wanting to test your system in a particular state? It is very hard to test these scenarios without mocking.

Without mocks, you may have to set up databases and other third parties things just to test simple business rules. You're likely to have slow tests, resulting in slow feedback loops.

By having to spin up a database or a web service to test something you're likely to have fragile tests due to the unreliability of such services.

Once a developer learns about mocking it becomes very easy to over-test every single facet of a system in terms of the way it works rather than what it does. Always be mindful about the value of your tests and what impact they would have in future refactoring.

In this post about mocking we have only covered Spies, which are a kind of mock. Mocks are a type of "test double."

> Test Double is a generic term for any case where you replace a production object for testing purposes.

Under test doubles, there are various types like stubs, spies and indeed mocks! Check out [Martin Fowler's post](https://martinfowler.com/bliki/TestDouble.html) for more detail.
