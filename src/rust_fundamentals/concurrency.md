# Concurrency

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/concurrency).

## The First Requirement: Faster Website Checker

Here is the setup: a colleague has written a function, `CheckWebsites`, that checks the status of a list of URLs.

```rust
use std::collections::HashMap;

pub trait WebsiteChecker {
    fn check(&self, url: &str) -> bool;
}

fn check_websites<'a>(urls: &'a [&str], checker: impl WebsiteChecker) -> HashMap<&'a str, bool> {
    let mut results = HashMap::new();
    for &url in urls {
        let is_up = checker.check(url);
        results.insert(url, is_up);
    }
    results
}
```

It returns a map of each URL checked to a boolean value: `true` for a good response, and `false` for a bad response.

You also have to pass in a `WebsiteChecker` implementation, which is a trait with a single method, `check`, that takes a URL and returns a boolean indicating whether the website is up or not. The `check_websites` function uses this to check all the websites.

Using [dependency injection](./dependency_injection.md) has allowed them to test the function without making real HTTP calls, making it reliable and fast.

Here is the test they have written:

```rust
#[cfg(test)]
mod specs_for_check_websites {
    use std::collections::HashMap;

    use super::WebsiteChecker;
    use super::check_websites;

    struct WebsiteCheckerMock {
        bad_websites: Vec<String>,
    }

    impl WebsiteChecker for WebsiteCheckerMock {
        fn check(&self, url: &str) -> bool {
            !self.bad_websites.contains(&url.to_string())
        }
    }

    #[test]
    fn sut_records_the_result_of_website_checker_into_result_correctly() {
        // Arrange
        let bad_website = "waat://furhurterwe.geds";
        let websites = [
            "http://google.com",
            "http://blog.gypsydave5.com",
            bad_website,
        ];
        let website_checker_stub = WebsiteCheckerMock {
            bad_websites: vec![bad_website.to_string()],
        };

        // Act
        let actual = check_websites(&websites, website_checker_stub);

        // Assert
        let expected = HashMap::from([
            ("http://google.com", true),
            ("http://blog.gypsydave5.com", true),
            (bad_website, false),
        ]);
        assert_eq!(expected, actual);
    }
}
```

The function is in production and being used to check hundreds of websites. But your colleague has started to get complaints that it's slow, so they've asked you to help speed it up.

### Write a Test First

We can use a benchmark test as we learned with criterion crate, but let's make it simple for now. We will write a test recording the execution time of `check_websites` function.

```rust
#[cfg(test)]
mod benches_for_check_websites {
    use std::time::Duration;
    use std::time::Instant;

    use super::WebsiteChecker;
    use super::check_websites;

    struct SlowWebsiteChecker {
        delay: Duration,
    }

    impl SlowWebsiteChecker {
        fn new(delay: Duration) -> Self {
            Self { delay }
        }
    }

    impl WebsiteChecker for SlowWebsiteChecker {
        fn check(&self, _url: &str) -> bool {
            std::thread::sleep(self.delay);
            true
        }
    }

    #[test]
    fn check_sut_execution_time() {
        // Arrange
        let urls = (0..10).map(|_| "http://example.com").collect::<Vec<_>>();
        let checker = SlowWebsiteChecker::new(Duration::from_millis(20));

        // Act
        let start = Instant::now();
        let _ = check_websites(&urls, checker);
        let duration = start.elapsed();

        // Assert
        println!("Execution time: {:?}", duration);
    }
}
```

The test uses a new fake implementation of the `WebsiteChecker` trait. `SlowWebsiteChecker` is deliberately slow. It uses `std::thread::sleep` to wait exactly twenty milliseconds and then it returns true. And then it will run the `check_websites` function with a list of ten URLs.

To check the execution time, we use `println!` to print the duration of the execution. By default, the test will not print anything to the console, so we can only see the execution time when we run the test with `cargo nextest run --no-capture`.

### Try and Run the Test

Let's run the test to see how long it takes by running the following command:

```bash
cargo nextest run check_sut_execution_time --no-capture

running 1 test
Execution time: 244.381875ms
test specs_for_check_websites::check_sut_execution_time ... ok
```

`check_websites` takes about 240 milliseconds to run with the slow checker. Let's try and make this faster.

### Write Enough Code to Make It Pass

Now we can finally talk about concurrency which, for the purposes of the following, means "having more than one thing in progress." This is something that we do naturally every day.

For instance, this morning I made a cup of tea. I put the kettle on and then, while I was waiting for it to boil, I got the milk out of the fridge, got the tea out of the cupboard, found my favourite mug, put the teabag into the cup and then, when the kettle had boiled, I put the water in the cup.

What I didn't do was put the kettle on and then stand there blankly staring at the kettle until it boiled, then do everything else once the kettle had boiled.

If you can understand why it's faster to make tea the first way, then you can understand how we will make `check_websites` faster. Instead of waiting for a website to respond before sending a request to the next website, we will tell our computer to make the next request while it is waiting.

Normally in Rust, when we call a function `do_something` we wait for it to return (even if it has no value to return, we still wait for it to finish). We say that this operation is blocking - it makes us wait for it to finish. An operation that does not block in Rust will run in a separate process. Think of a process as reading down the page of Rust code from top to bottom, going 'inside' each function when it gets called to read what it does. When a separate process starts, it's like another reader begins reading inside the function, leaving the original reader to continue down the page.

We will use [Tokio](https://tokio.rs), an asynchronous runtime for Rust, to run our code concurrently. This allows us to run multiple tasks at the same time without blocking the main thread. Rust only provides the interface for asynchronous programming, so we need to use external libraries like [Tokio](https://tokio.rs) or [Async Std](https://async.rs) to actually run our code concurrently.

To use Tokio, we need to add it to our `Cargo.toml`:

```toml
[dependencies]
async-trait = "0.1"
futures = "0.3"
tokio = { version = "1.45", features = ["rt-multi-thread", "macros", "time"] }
```

We will also install the `async-trait` and `futures` crates to help us write asynchronous code. These will be covered later.

Before starting, I recommend you read the [Tokio documentation](https://tokio.rs/tokio/tutorial). This guide will help you understand the basics of asynchronous programming in Rust.

Now we can change our code to asynchronous version.

```rust,ignore
use std::collections::HashMap;

#[async_trait::async_trait]
pub trait WebsiteChecker {
    async fn check(&self, url: &str) -> bool;
}

pub async fn check_websites<'a>(
    urls: &'a [&str],
    checker: impl WebsiteChecker,
) -> HashMap<&'a str, bool> {
    let mut results = HashMap::new();
    for &url in urls {
        let is_up = checker.check(url).await;
        results.insert(url, is_up);
    }
    results
}

#[cfg(test)]
mod benches_for_check_websites {
    use std::time::Duration;
    use std::time::Instant;

    use super::WebsiteChecker;
    use super::check_websites;

    struct SlowWebsiteChecker {
        delay: Duration,
    }

    impl SlowWebsiteChecker {
        fn new(delay: Duration) -> Self {
            Self { delay }
        }
    }

    #[async_trait::async_trait]
    impl WebsiteChecker for SlowWebsiteChecker {
        async fn check(&self, _url: &str) -> bool {
            std::thread::sleep(self.delay);
            true
        }
    }

    #[tokio::test]
    async fn check_sut_execution_time() {
        // Arrange
        let urls = (0..10).map(|_| "http://example.com").collect::<Vec<_>>();
        let checker = SlowWebsiteChecker::new(Duration::from_millis(20));

        // Act
        let start = Instant::now();
        let _ = check_websites(&urls, checker).await;
        let duration = start.elapsed();

        // Assert
        println!("Execution time: {:?}", duration);
    }
}

#[cfg(test)]
mod specs_for_check_websites {
    use std::collections::HashMap;

    use super::WebsiteChecker;
    use super::check_websites;

    struct WebsiteCheckerMock {
        bad_websites: Vec<String>,
    }

    #[async_trait::async_trait]
    impl WebsiteChecker for WebsiteCheckerMock {
        async fn check(&self, url: &str) -> bool {
            !self.bad_websites.contains(&url.to_string())
        }
    }

    #[tokio::test]
    async fn sut_records_the_result_of_website_checker_into_result_correctly() {
        // Arrange
        let bad_website = "waat://furhurterwe.geds";
        let websites = [
            "http://google.com",
            "http://blog.gypsydave5.com",
            bad_website,
        ];
        let website_checker_stub = WebsiteCheckerMock {
            bad_websites: vec![bad_website.to_string()],
        };

        // Act
        let actual = check_websites(&websites, website_checker_stub).await;

        // Assert
        let expected = HashMap::from([
            ("http://google.com", true),
            ("http://blog.gypsydave5.com", true),
            (bad_website, false),
        ]);
        assert_eq!(expected, actual);
    }
}
```

We added `#[async_trait::async_trait]` to the `WebsiteChecker` trait to allow us to use asynchronous methods in traits. The `check_websites` function is now also asynchronous by adding `async`, and we call the `check` method with `.await`.

In the test, we implemented the `WebsiteChecker` trait for `WebsiteCheckerMock` using `#[async_trait::async_trait]`, allowing us to use the asynchronous `check` method.

Finally, we changed the test function to be asynchronous by using `#[tokio::test]`. This is necessary because we are now running asynchronous code, and we need a runtime to execute it.

If you run the test, you will see that it passes. How about the benchmark test? You should see that nothing has changed.

```bash
running 1 test
Execution time: 240.927708ms
test benches_for_check_websites::check_sut_execution_time ... ok
```

What happened? We have made our code asynchronous, but we are still waiting for each website to respond before moving on to the next one.

```rust,ignore
for &url in urls {
    let is_up = checker.check(url).await;
    results.insert(url, is_up);
}
```

The line `checker.check(url).await` is still blocking the loop until the website responds. Only when the first website is checked, the next one is checked. This is not what we want. To make this concurrent, we need to run all the checks at the same time. We can do this by using [`futures::join_all`](https://docs.rs/futures/latest/futures/future/fn.join_all.html) to run all the checks concurrently. In the documentation, you can see an example as following.

```rust,ignore
use futures::future::join_all;

async fn foo(i: u32) -> u32 { i }
let futures = vec![foo(1), foo(2), foo(3)];
assert_eq!(join_all(futures).await, [1, 2, 3]);
```

This will run all the futures in the vector concurrently and wait for all of them to finish. We can use this to run all the website checks concurrently.

```rust,ignore
pub async fn check_websites<'a>(
    urls: &'a [&str],
    checker: impl WebsiteChecker,
) -> HashMap<&'a str, bool> {
    let mut futures = Vec::new();
    for &url in urls {
        let future = checker.check(url);
        futures.push(future);
    }
    let responses = join_all(futures).await;

    let mut result = HashMap::new();
    for (&url, response) in urls.iter().zip(responses) {
        result.insert(url, response);
    }
    result
}
```

```rust,ignore
#[async_trait::async_trait]
impl WebsiteChecker for SlowWebsiteChecker {
    async fn check(&self, url: &str) -> bool {
        println!("start to check {}", url);
        std::thread::sleep(self.delay);
        println!("finish checking {}", url);
        true
    }
}
```

Looks good! We have changed the `check_websites` function to use `join_all` to run all the checks concurrently, and added start and finish logging on each URL check. Now, we can run the benchmark test again to see how long it takes.

```bash
running 1 test
start to check http://example.com
finish checking http://example.com
start to check http://example.com
finish checking http://example.com
..
start to check http://example.com
finish checking http://example.com
Execution time: 237.814667ms
test benches_for_check_websites::check_sut_execution_time ... ok
```

Oh, it is still about the same time! Why? What is blocking our thread? If you read the Tokio documentation, you might have noticed that we are using `std::thread::sleep` in our `SlowWebsiteChecker`. This is blocking the thread, which means that it is not allowing other tasks to run while it is sleeping. We need to use `tokio::time::sleep` instead, which is non-blocking and allows other tasks to run while it is waiting.

Change the `SlowWebsiteChecker` implementation to use `tokio::time::sleep`, and run the test again.

```rust,ignore
// std::thread::sleep(self.delay);
tokio::time::sleep(self.delay).await;
```

```bash
running 1 test
start to check http://example.com
start to check http://example.com
..
finish checking http://example.com
finish checking http://example.com
Execution time: 22.675792ms
test benches_for_check_websites::check_sut_execution_time ... ok
```

Yay! Now the execution time is around 20 milliseconds, which is much faster than before. We have successfully made our `check_websites` function concurrent.

### Refactor

If you are interested in the functional programming style, you can change the `check_websites` function without for loop.

```rust,ignore
pub async fn check_websites<'a>(
    urls: &'a [&str],
    checker: impl WebsiteChecker,
) -> HashMap<&'a str, bool> {
    let futures = urls
        .iter()
        .map(|&url| checker.check(url))
        .collect::<Vec<_>>();
    let responses = join_all(futures).await;

    urls.iter()
        .zip(responses)
        .map(|(&url, response)| (url, response))
        .collect::<HashMap<_, _>>()
}
```

## The Second Requirement: Faster Website Checker with Multiple Threads

Now we have a concurrent website checker, but it can be throttled when there are many tasks to perform. The code we wrote essentially uses only a single thread. Remember that when we use `std::thread::sleep`, it blocks the thread, and we can only run one task at a time on that thread. This means that if we have many websites to check, we will still be waiting for each website to respond sequentially.

We can make it even better by using multiple threads. This is where the Tokio runtime comes in handy. It allows us to run our code on multiple threads, which can significantly speed up our website checker.

### Write a Test First

To make a test for multiple threads, we need to change the way we run our tests. Instead of using `#[tokio::test]`, we will use `#[tokio::test(flavor = "multi_thread", worker_threads = 5)]`. This will allow us to run our tests on multiple threads, with a maximum of five worker threads. If you see [here](https://docs.rs/tokio/latest/tokio/attr.test.html), you can see that the default of `#[tokio::test]` is `flavor = "current_thread"` with single-threaded execution. By setting `flavor = "multi_thread", worker_threads = 5`, we can run our tests on multiple threads.

```rust,ignore
#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn check_sut_execution_time() {
    // Arrange
    let urls = (0..10).map(|_| "http://example.com").collect::<Vec<_>>();
    let checker = SlowWebsiteChecker::new(Duration::from_millis(20));

    // Act
    let start = Instant::now();
    let _ = check_websites(&urls, checker).await;
    let duration = start.elapsed();

    // Assert
    println!("Execution time: {:?}", duration);
}
```

To make sure that we are actually using multiple threads, we can change the `tokio::time::sleep` method to the blocking `std::thread::sleep` and run the test again. If we use a single thread, the test will take a long time because the main thread is blocked. If we use multiple threads, the test will finish quickly because the worker threads can run concurrently.

```rust,ignore
#[async_trait::async_trait]
impl WebsiteChecker for SlowWebsiteChecker {
    async fn check(&self, url: &str) -> bool {
        println!("start to check {}", url);
        std::thread::sleep(self.delay);
        println!("finish checking {}", url);
        true
    }
}
```

### Try and Run the Test

Let's run the test `check_sut_execution_time` again:

```bash
running 1 test
start to check http://example.com
finish checking http://example.com
start to check http://example.com
finish checking http://example.com
..
start to check http://example.com
finish checking http://example.com
Execution time: 233.619ms
test benches_for_check_websites::check_sut_execution_time ... ok
```

We can see that the test takes about 230 milliseconds to run. Even though we implemented the `check_websites` function to run concurrently, it is still slow because we are using a blocking `sleep` with a single thread.

### Write Enough Code to Make It Pass

Now we will use `tokio::spawn` to run each website check in a separate task able to run in different threads. This will allow us to run multiple tasks concurrently even though the task blocks the thread.

If you refer to the [Spawning](https://tokio.rs/tokio/tutorial/spawning) tutorial, you can see that there are some limitations when using `tokio::spawn`.

- The passed arguments to `tokio::spawn` must be `'static`, meaning that the spawned task must not contain any references to data owned outside the task
- The spawned task must implement `Send` trait because Tokio runtime moves the tasks between threads

We need to run the `check` method in `tokio::spawn`, so we have to change the `WebsiteChecker` trait not to refer to any data outside the task.

```rust,ignore
#[async_trait::async_trait]
pub trait WebsiteChecker: Clone + Send + 'static {
    async fn check(&self, url: String) -> bool;
}
```

After that, we can change the `check_websites` function to spawn a task and change the input type of `check` method to `String` instead of `&str`.

```rust,ignore
pub async fn check_websites<'a>(
    urls: &'a [&str],
    checker: impl WebsiteChecker,
) -> HashMap<&'a str, bool> {
    let handles = urls
        .iter()
        .map(|&url| {
            let url = url.to_string();
            let checker = checker.clone();
            tokio::spawn(async move { checker.check(url).await })
        })
        .collect::<Vec<_>>();
    let responses = join_all(handles).await;

    urls.iter()
        .zip(responses)
        .map(|(&url, response)| match response {
            Ok(is_up) => (url, is_up),
            Err(_) => (url, false),
        })
        .collect::<HashMap<_, _>>()
}
```

We ran `tokio::spawn` on each `checker.check(url).await` call, passing the URL as a `String` and a cloned checker. As mentioned, the asynchronous task can run on any thread, so references are not allowed, which means we have to transfer owned types. That's why we added the `Clone` trait to the `WebsiteChecker` trait as well.

We also collect the handles of the spawned tasks and wait for them to finish using `join_all`.

Finally, we handle the result of each task. If the task was successful, we return the result; if it failed, we return `false`.

We need to fix other implementations of `WebsiteChecker` trait to match the new signature of `check` method. This is the final code until now.

```rust,ignore
use std::collections::HashMap;

use futures::future::join_all;

#[async_trait::async_trait]
pub trait WebsiteChecker: Clone + Send + 'static {
    async fn check(&self, url: String) -> bool;
}

pub async fn check_websites<'a>(
    urls: &'a [&str],
    checker: impl WebsiteChecker,
) -> HashMap<&'a str, bool> {
    let handles = urls
        .iter()
        .map(|&url| {
            let url = url.to_string();
            let checker = checker.clone();
            tokio::spawn(async move { checker.check(url).await })
        })
        .collect::<Vec<_>>();
    let responses = join_all(handles).await;

    urls.iter()
        .zip(responses)
        .map(|(&url, response)| match response {
            Ok(is_up) => (url, is_up),
            Err(_) => (url, false),
        })
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod benches_for_check_websites {
    use std::time::Duration;
    use std::time::Instant;

    use super::WebsiteChecker;
    use super::check_websites;

    #[derive(Clone)]
    struct SlowWebsiteChecker {
        delay: Duration,
    }

    impl SlowWebsiteChecker {
        fn new(delay: Duration) -> Self {
            Self { delay }
        }
    }

    #[async_trait::async_trait]
    impl WebsiteChecker for SlowWebsiteChecker {
        async fn check(&self, url: String) -> bool {
            println!("start to check {}", url);
            std::thread::sleep(self.delay);
            println!("finish checking {}", url);
            true
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn check_sut_execution_time() {
        // Arrange
        let urls = (0..10).map(|_| "http://example.com").collect::<Vec<_>>();
        let checker = SlowWebsiteChecker::new(Duration::from_millis(20));

        // Act
        let start = Instant::now();
        let _ = check_websites(&urls, checker).await;
        let duration = start.elapsed();

        // Assert
        println!("Execution time: {:?}", duration);
    }
}

#[cfg(test)]
mod specs_for_check_websites {
    use std::collections::HashMap;

    use super::WebsiteChecker;
    use super::check_websites;

    #[derive(Clone)]
    struct WebsiteCheckerMock {
        bad_websites: Vec<String>,
    }

    #[async_trait::async_trait]
    impl WebsiteChecker for WebsiteCheckerMock {
        async fn check(&self, url: String) -> bool {
            !self.bad_websites.contains(&url.to_string())
        }
    }

    #[tokio::test]
    async fn sut_records_the_result_of_website_checker_into_result_correctly() {
        // Arrange
        let bad_website = "waat://furhurterwe.geds";
        let websites = [
            "http://google.com",
            "http://blog.gypsydave5.com",
            bad_website,
        ];
        let website_checker_stub = WebsiteCheckerMock {
            bad_websites: vec![bad_website.to_string()],
        };

        // Act
        let actual = check_websites(&websites, website_checker_stub).await;

        // Assert
        let expected = HashMap::from([
            ("http://google.com", true),
            ("http://blog.gypsydave5.com", true),
            (bad_website, false),
        ]);
        assert_eq!(expected, actual);
    }
}
```

Let's run the test `check_sut_execution_time` again to see how long it takes.

```bash
running 1 test
start to check http://example.com
start to check http://example.com
start to check http://example.com
start to check http://example.com
start to check http://example.com
finish checking http://example.com
finish checking http://example.com
start to check http://example.com
start to check http://example.com
finish checking http://example.com
start to check http://example.com
finish checking http://example.com
finish checking http://example.com
start to check http://example.com
start to check http://example.com
finish checking http://example.com
finish checking http://example.com
finish checking http://example.com
finish checking http://example.com
finish checking http://example.com
Execution time: 50.451584ms
test benches_for_check_websites::check_sut_execution_time ... ok
```

The result shows a slightly different pattern. The first five checks started at the same time, and then they finished one by one. This is because we have five worker threads, and each thread can run one task at a time. If we adjust the number of threads, we can see the difference in execution time. For example, if we change `worker_threads = 10`, we can run ten tasks at the same time, so the execution time will be roughly 20 milliseconds.

### Refactor

We can change the test `check_sut_execution_time` to assert that the execution runs concurrently. By setting the number of threads and URLs to be the same, we should see that the execution time is less than the sum of the delays of each URL check.

```rust,ignore
#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn sut_executes_checkers_concurrently() {
    // Arrange
    let urls = (0..5).map(|_| "http://example.com").collect::<Vec<_>>();
    let checker = SlowWebsiteChecker::new(Duration::from_millis(20));

    // Act
    let start = Instant::now();
    let _ = check_websites(&urls, checker).await;
    let duration = start.elapsed();

    // Assert
    assert!(duration.as_millis() <= 30);
}
```

When writing asynchronous code, it is important to remember that blocking operations such as `std::thread::sleep` will block the entire thread, preventing other tasks from running. Always use non-blocking operations like `tokio::time::sleep` to allow other tasks to run concurrently.

If you need to run blocking code in an asynchronous context, you can use `tokio::task::spawn_blocking` to run the blocking code in a separate thread pool.

## Wrapping Up

### What We Learned

This exercise has been a little lighter on the TDD than usual. In a way we've been taking part in one long refactoring of the `check_websites` function; the inputs and outputs never changed, it just got faster. But the tests we had in place, as well as the benchmark we wrote, allowed us to refactor the function in a way that maintained confidence that the software was still working, while demonstrating that it had actually become faster.

In making it faster we learned about:

- concurrency, which is the ability to run multiple tasks at the same time, allowing us to check multiple websites concurrently
- asynchronous programming, which allows us to write code that can run concurrently without blocking the main thread

### Make It Fast

One formulation of an agile way of building software, often misattributed to Kent Beck, is:

> [Make it work, make it right, make it fast](https://wiki.c2.com/?MakeItWorkMakeItRightMakeItFast) - Kent Beck

Where 'work' is making the tests pass, 'right' is refactoring the code, and 'fast' is optimizing the code to make it, for example, run quickly. We can only 'make it fast' once we've made it work and made it right. We were lucky that the code we were given was already demonstrated to be working, and didn't need to be refactored. We should never try to 'make it fast' before the other two steps have been performed because

> [Premature optimization is the root of all evil](https://wiki.c2.com/?PrematureOptimization) - Donald Knuth
