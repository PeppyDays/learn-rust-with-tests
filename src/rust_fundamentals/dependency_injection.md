# Dependency Injection

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/di).

It is assumed that you have read the [Structs, Methods and Traits](./structs_methods_and_traits.md) chapter before as some understanding of traits will be needed for this.

There are a lot of misunderstandings around dependency injection around the programming community. Hopefully, this guide will show you how

- You don't need a framework
- It does not overcomplicate your design
- It facilitates testing
- It allows you to write great, general-purpose functions

## The First Requirement: Greeting

We want to write a function that greets someone, just like we did in the [Hello, World](./hello_world.md) chapter but this time we are going to be testing the actual printing.

Just to recap, here is what that function could look like:

```rust
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

But how can we test this? Calling `println!` prints to stdout, which is pretty hard for us to capture using the testing framework.

What we need to do is to be able to inject (which is just a fancy word for pass in) the dependency of printing.

Our function doesn't need to care where or how the printing happens, so we should accept an interface rather than a concrete type.

If we do that, we can then change the implementation to print to something we control so that we can test it. In "real life" you would inject in something that writes to stdout.

If you look at the source code of `println!`, you can see a way for us to hook in.

```rust
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::io::_print($crate::format_args_nl!($($arg)*));
    }};
}
```

Okay, the macro just routes input arguments to the `std::io::_print` function. Let's dive deep into the function.

```rust,ignore
pub fn _print(args: fmt::Arguments<'_>) {
    print_to(args, stdout, "stdout");
}

pub fn stdout() -> Stdout {
    Stdout {
        inner: STDOUT
            .get_or_init(|| ReentrantLock::new(RefCell::new(LineWriter::new(stdout_raw())))),
    }
}
```

We're almost there. The `print_to` function is the one that actually does the printing. Let's look at it.

```rust,ignore
fn print_to<T>(args: fmt::Arguments<'_>, global_s: fn() -> T, label: &str)
where
    T: Write,
{
    if print_to_buffer_if_capture_used(args) {
        // Successfully wrote to capture buffer.
        return;
    }

    if let Err(e) = global_s().write_fmt(args) {
        panic!("failed printing to {label}: {e}");
    }
}
```

Interesting! Under the hood, `println!` just calls `print_to` passing in `Stdout`. `print_to` expects to get a function that generates a `Write` object. This is exactly what we need to do. What is `Write`?

```rust,ignore
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn is_write_vectored(&self) -> bool;
    fn flush(&mut self) -> io::Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()>;
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> io::Result<()>;
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&*self).write(buf)
    }
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        (&*self).write_vectored(bufs)
    }
    #[inline]
    fn is_write_vectored(&self) -> bool {
        io::Write::is_write_vectored(&&*self)
    }
    fn flush(&mut self) -> io::Result<()> {
        (&*self).flush()
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (&*self).write_all(buf)
    }
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        (&*self).write_all_vectored(bufs)
    }
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> io::Result<()> {
        (&*self).write_fmt(args)
    }
}
```

From this, we can infer that `Stdout` implements `Write`. And `println!` passes `Stdout` (as a functional form) to `print_to`, which then calls `write_fmt` on it.

`Write` is a trait that allows us to write bytes to a destination. It has a method `write` that takes a byte slice and returns the number of bytes written.

As you write more Rust code, you will find this interface popping up a lot because it's a great general purpose trait for "putting this data somewhere".

So we know under the covers we're ultimately using `Write` to send our greeting somewhere. Let's use this existing abstraction to make our code testable and more reusable.

### Write the Test First

```rust
#[cfg(test)]
mod specs_for_greet {
    use super::greet;

    #[test]
    fn sut_writes_greeting_to_bytes_buffer_correctly() {
        // Arrange
        let mut buffer: Vec<u8> = Vec::new();

        // Act
        greet(&mut buffer, "Chris");

        // Assert
        let actual = String::from_utf8(buffer).unwrap();
        let expected = "Hello, Chris!";
        assert_eq!(expected, actual);
    }
}
```

We expect that the first argument of `greet` is a `Write` trait object. If any type implements `Write`, we can pass it to `greet`.

We're writing the greeting to `Vec<u8>`, which is a byte vector. Byte array `&mut [u8]` implements `Write`, so `&mut Vec<u8>` implements `Write` as well due to the [deref coercion](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods). This means we can pass it to `greet` and it will work.

### Try and Run the Test

The test will not compile.

```bash
error[E0425]: cannot find function `greet` in this scope
  --> src/lib.rs:18:9
   |
18 |         greet(&mut buffer, "Chris");
   |         ^^^^^ not found in this scope
```

### Write the Minimal Amount of Code

Listen to the compiler and fix the problem.

```rust
use std::io::Write;

pub fn greet(writer: &mut dyn Write, name: &str) {
    println!("Hello, {}!", name);
}
```

```bash
    thread 'specs_for_greet::sut_writes_greeting_to_bytes_buffer_correctly' panicked at src/lib.rs:27:9:
    assertion `left == right` failed
      left: "Hello, Chris!"
     right: ""
```

The test fails. Notice that the name is getting printed out, but it's going to stdout.

### Write Enough Code to Make It Pass

Instead of calling `println!`, we need to call `write_all` on the `writer` object.

```rust,ignore
pub fn greet(writer: &mut dyn Write, name: &str) {
    let greeting = format!("Hello, {}!", name);
    writer.write_all(greeting.as_bytes()).unwrap();
}
```

The test now passes. We can put any `Write` trait object in there and it will work.

### Refactor

Nothing to do here.

## Wrapping Up

### Benefits of Dependency Injection

Our first round of code was not easy to test because it wrote data to somewhere we couldn't control.

Motivated by our tests we refactored the code so we could control where the data was written by injecting a dependency. Dependency injection enables us to do the followings.

#### Test our code

If you can't test a function easily, it's usually because of dependencies hard-wired into a function or global state. If you have a global database connection pool for instance that is used by some kind of service layer, it is likely going to be difficult to test and they will be slow to run.

DI will motivate you to inject in a database dependency (via an interface) which you can then mock out with something you can control in your tests.

#### Separate our concerns

Decoupling where the data goes from how to generate it. If you ever feel like a method/function has too many responsibilities, e.g. generating data and writing to a db? handling HTTP requests and doing domain level logic?

DI is probably going to be the tool you need.

#### Allow our code to be re-used in different contexts

The first "new" context our code can be used in is inside tests. But further on if someone wants to try something new with your function they can inject their own dependencies.

### What about Mocking?

Mocking will be covered in detail later (and it's not evil). You use mocking to replace real things you inject with a pretend version that you can control and inspect in your tests. In our case though, the standard library had something ready for us to use.
