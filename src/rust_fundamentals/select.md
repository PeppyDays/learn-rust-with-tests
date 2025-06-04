# Select

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/select).

## The First Requirement: Find Faster URL

You've been asked to create a function called WebsiteRacer that takes two URLs and "races" them by sending HTTP GET requests, returning whichever URL responds first. If neither returns within 10 seconds, it should return an error.

For this, we will be using:

- [tokio](https://docs.rs/tokio/latest/tokio/) and [futures](https://docs.rs/futures/latest/futures/) for asynchronous programming
  - `tokio::select!` to synchronise processes
- [reqwest](https://docs.rs/reqwest/latest/reqwest/) to make the HTTP calls
- [wiremock](https://docs.rs/wiremock/latest/wiremock/) to help us test them

### Write the Test First

Let's start with something naive to get us going.

```rust
#[cfg(test)]
mod specs_for_race {
    use super::race;

    #[tokio::test]
    async fn sut_returns_fastest_url_correctly() {
        // Arrange
        let slow_url = "http://slow.example.com/slow";
        let fast_url = "http://fast.example.com/fast";

        // Act
        let actual = race(slow_url, fast_url).await.unwrap();

        // Assert
        let expected = fast_url;
        assert_eq!(expected, actual);
    }
}
```

We know this isn't perfect and has problems, but it's a start. Avoid getting hung up on perfection initially.

### Try to Run the Test

```bash
error[E0432]: unresolved import `super::race`
 --> src/v1.rs:7:9
  |
7 |     use super::race;
  |         ^^^^^^^^^^^ no `race` in `v1`
```

### Write the Minimal Amount of Code

```rust
pub async fn race<'a>(url_1: &'a str, url_2: &'a str) -> Result<&'a str, &'static str> {
    Err("not implemented yet")
}
```

```bash
thread 'specs_for_race::sut_returns_fastest_url_correctly' panicked at src/lib.rs:16:53:
called `Result::unwrap()` on an `Err` value: "not implemented yet"
```

### Write Enough Code to Make It Pass

To make HTTP requests, we'll use the reqwest crate. Since it supports asynchronous calls, we'll use it with tokio and futures crates. First, add them to `Cargo.toml`:

```toml
[dependencies]
futures = "0.3"
reqwest = { version = "0.12", default-features = false, features = [
    "json",
    "rustls-tls",
] }
tokio = { version = "1.45", features = ["rt-multi-thread", "macros"] }
```

```rust,ignore
use std::time::Instant;

use reqwest::Client;

pub async fn race<'a>(url_1: &'a str, url_2: &'a str) -> Result<&'a str, &'static str> {
    let client = Client::new();

    let start_1 = Instant::now();
    let _ = client.get(url_1).send().await.unwrap();
    let duration_1 = start_1.elapsed();

    let start_2 = Instant::now();
    let _ = client.get(url_2).send().await.unwrap();
    let duration_2 = start_2.elapsed();

    if duration_1 < duration_2 {
        Ok(url_1)
    } else {
        Ok(url_2)
    }
}
```

For each URL:

- We use `std::time::Instant::now` to record the time before attempting to get the URL
- Then we use `reqwest::Client.send` to perform an HTTP GET request
  - This function returns `Result<Response, Error>`, but we're not interested in these values yet
- `std::time::Instant.elapsed` returns the duration difference

Once completed, we simply compare durations to find the fastest.

### Problems

This may or may not make the test pass. The problem is we're reaching out to real websites to test our logic.

Testing HTTP code is so common that Rust provides the [wiremock](https://docs.rs/wiremock/latest/wiremock/) crate for creating controllable mock HTTP servers.

As covered in the [mocking](./mocking.md) and [dependency injection](./dependency_injection.md) chapters, we ideally avoid relying on external services for testing because they can be:

- Slow
- Flaky
- Unable to test edge cases

Let's change our tests to use mocks for reliable, controllable test servers.

```rust
#[cfg(test)]
mod specs_for_race {
    use std::time::Duration;

    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    use super::race;

    #[tokio::test]
    async fn sut_returns_fastest_url_correctly() {
        // Arrange
        let slow_server = MockServer::start().await;
        let slow_url = slow_server.uri();
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(20)))
            .mount(&slow_server)
            .await;

        let fast_server = MockServer::start().await;
        let fast_url = fast_server.uri();
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&fast_server)
            .await;

        // Act
        let actual = race(&slow_url, &fast_url).await.unwrap();

        // Assert
        let expected = &fast_url;
        assert_eq!(expected, actual);
    }
}
```

The syntax may look busy, but take your time.

`MockServer::start` starts a new mock HTTP server locally with an actual port for sending requests. It returns a `MockServer` instance for server configuration.

We use `Mock::given` to set up server mocks. We specify the HTTP method and path to match, then use `respond_with` to define the response for matching requests. Note the 20-millisecond delay for the slow server, simulating a slow response. Finally, we mount the mock using `mount`, making it active and ready to respond.

There's no magic here. `MockServer` simplifies testing by finding an open port and allowing cleanup after tests.

Re-running the test should now pass and be faster. Experiment with these delays to deliberately break the test.

### Refactor

We have duplication in both production and test code.

```rust,ignore
use std::time::Duration;
use std::time::Instant;

use reqwest::Client;

pub async fn race<'a>(url_1: &'a str, url_2: &'a str) -> Result<&'a str, &'static str> {
    let client = Client::new();

    let duration_1 = measure_response_duration(&client, url_1).await;
    let duration_2 = measure_response_duration(&client, url_2).await;

    if duration_1 < duration_2 {
        Ok(url_1)
    } else {
        Ok(url_2)
    }
}

async fn measure_response_duration(client: &Client, url: &str) -> Duration {
    let start = Instant::now();
    let _ = client.get(url).send().await.unwrap();
    start.elapsed()
}

#[cfg(test)]
mod specs_for_race {
    use std::time::Duration;

    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    use super::race;

    #[tokio::test]
    async fn sut_returns_fastest_url_correctly() {
        // Arrange
        let slow_url = arrange_server(Some(Duration::from_millis(20))).await;
        let fast_url = arrange_server(None).await;

        // Act
        let actual = race(&slow_url, &fast_url).await.unwrap();

        // Assert
        let expected = &fast_url;
        assert_eq!(expected, actual);
    }

    async fn arrange_server(delay: Option<Duration>) -> String {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_delay(delay.unwrap_or_default()))
            .mount(&server)
            .await;
        server.uri()
    }
}
```

We've refactored mock server creation into an `arrange_server` function to remove uninteresting code from tests and reduce repetition.

## The Second Requirement: Synchronize Finding Processes

Why test website speeds sequentially when Rust excels at concurrency? We should check both simultaneously. We don't care about exact response times - just which returns first.

We'll introduce `select`, a construct that helps synchronize processes easily and clearly.

We're refactoring our `race` function to use `select`, so no new test is needed. We can change the implementation and verify it still passes.

```rust,ignore
use reqwest::Client;

pub async fn race<'a>(url_1: &'a str, url_2: &'a str) -> Result<&'a str, &'static str> {
    let client = Client::new();

    tokio::select! {
        response = ping(&client, url_1) => {
            if response.is_ok() {
                return Ok(url_1);
            }
        }
        response = ping(&client, url_2) => {
            if response.is_ok() {
                return Ok(url_2);
            }
        }
    }

    Err("no successful response received")
}

async fn ping<'a>(client: &'a Client, url: &str) -> Result<(), &'a str> {
    let _ = client
        .get(url)
        .send()
        .await
        .map_err(|_| "failed to send request")?;
    Ok(())
}
```

Let's break this down.

We've defined a `ping` function that checks the URL. If sending a request fails, it returns an error. If successful, it returns `Ok(())` since we don't care about response code or body.

The `tokio::select!` macro lets us wait for multiple asynchronous operations to complete. It returns as soon as one operation completes, allowing us to handle the result accordingly. The macro can handle more than two branches. Each branch follows this structure:

```plain
<pattern> = <async expression> => <handler>,
```

When evaluated, all `<async expression>`s are aggregated and executed concurrently. When an expression completes, the result is matched against `<pattern>`. If matching, all remaining async expressions are dropped and `<handler>` executes.

If `<pattern>` doesn't match the asynchronous computation result, remaining expressions continue executing concurrently until the next completes. The same logic applies to that result.

Our select macro has two branches, one per URL. If the first URL's response arrives first and succeeds, we return `Ok(url_1)`. If the second URL responds first and succeeds, we return `Ok(url_2)`. If neither returns a successful response, we return an error.

These changes make our code's intent very clear while simplifying the implementation.

## The Third Requirement: Ignore Sending Request Error Branch

In our current implementation, what happens when a request fails? If the first URL is down or unreachable, the `ping` function returns an error, the `tokio::select!` macro consumes the first branch, exits the select macro, and returns the error.

This behavior isn't what we want. Even if the first URL fails, we should still check the second URL. If the second URL succeeds, we should return it.

### Write the Test First

```rust,ignore
#[tokio::test]
async fn sut_ignores_result_of_sending_failure_request() {
    // Arrange
    let not_working_url = "http://non-existent.url".to_string();
    let working_url = arrange_server(Some(Duration::from_millis(20))).await;

    // Act
    let actual = race(&not_working_url, &working_url).await.unwrap();

    // Assert
    let expected = &working_url;
    assert_eq!(expected, actual);
}

#[tokio::test]
async fn sut_returns_error_if_two_urls_are_failed_to_send_request() {
    // Arrange
    let not_working_url = "http://non-existent.url".to_string();

    // Act
    let actual = race(&not_working_url, &not_working_url).await.unwrap_err();

    // Assert
    let expected = "no successful response received";
    assert_eq!(expected, actual);
}
```

In the first test, we set the first URL to a non-existent URL and the second to a mock server responding after 20 milliseconds. The first URL will fail before the second responds. We want to ensure the first URL's failure is ignored and the second URL is returned.

The second test verifies that when both URLs fail to send requests, we return an error.

### Try to Run the Test

```bash
thread 'specs_for_race::sut_ignores_result_of_sending_failure_request' panicked at src/lib.rs:64:65:
called `Result::unwrap()` on an `Err` value: "no successful response received"
```

### Write Enough Code to Make It Pass

```rust,ignore
pub async fn race<'a>(url_1: &'a str, url_2: &'a str) -> Result<&'a str, &'static str> {
    let client = Client::new();

    tokio::select! {
        Ok(_) = ping(&client, url_1) => {
            Ok(url_1)
        }
        Ok(_) = ping(&client, url_2) => {
            Ok(url_2)
        }
        else => {
            Err("no successful response received")
        }
    }
}
```

Remember that the select macro binds asynchronous expression results to patterns? If the pattern doesn't match, the branch drops and the select macro continues to the next branch. Using this property, we can ignore the first ping's failure and keep waiting for the second ping to complete. The else branch executes when all branches drop, meaning both pings failed.

Run the tests again - they should pass.

### Refactor

Nothing special to refactor, but we can extract the error case in tests.

```rust,ignore
#[tokio::test]
async fn sut_ignores_result_of_sending_failure_request() {
    // Arrange
    let not_working_url = arrange_server_with_error();
    let working_url = arrange_server(Some(Duration::from_millis(20))).await;

    // Act
    let actual = race(&not_working_url, &working_url).await.unwrap();

    // Assert
    let expected = &working_url;
    assert_eq!(expected, actual);
}

#[tokio::test]
async fn sut_returns_error_if_two_urls_are_failed_to_send_request() {
    // Arrange
    let not_working_url = arrange_server_with_error();

    // Act
    let actual = race(&not_working_url, &not_working_url).await.unwrap_err();

    // Assert
    let expected = "no successful response received";
    assert_eq!(expected, actual);
}

fn arrange_server_with_error() -> String {
    "http://non-existent.url".to_string()
}
```

## The Fourth Requirement: Timeout After 10 Seconds

Our final requirement is returning an error if Racer takes longer than 10 seconds.

### Write the Test First

```rust,ignore
#[tokio::test]
async fn sut_returns_error_if_a_server_does_not_respond_within_10s() {
    // Arrange
    let url_1 = arrange_server(Some(Duration::from_secs(11))).await;
    let url_2 = arrange_server(Some(Duration::from_secs(12))).await;

    // Act
    let actual = race(&url_1, &url_2).await.unwrap_err();

    // Assert
    let expected = "no successful response received";
    assert_eq!(expected, actual);
}
```

We've made our test servers take longer than 10 seconds to return, exercising this scenario.

### Try to Run the Test

```bash
thread 'specs_for_race::sut_returns_error_if_a_server_does_not_respond_within_10s' panicked at src/lib.rs:88:49:
called `Result::unwrap_err()` on an `Ok` value: "http://127.0.0.1:60023"
```

### Write Enough Code to Make It Pass

```rust,ignore
async fn ping<'a>(client: &'a Client, url: &str) -> Result<(), &'a str> {
    let _ = client
        .get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|_| "failed to send request")?;
    Ok(())
}
```

By adding a timeout to the request, we ensure that requests taking longer than 10 seconds return an error.

You could also put a timeout on the `tokio::select!` macro itself, but since we need an error branch when both branches fail to send requests, the only place to modify the timeout is in the `ping` function.

### Refactor

The problem is this test takes 10 seconds to run. For such simple logic, this doesn't feel great.

We can make the timeout configurable. In our test, we can use a very short timeout, while real-world usage can use 10 seconds.

How about adding a `timeout` parameter to the `race` function?

```rust,ignore
pub async fn race<'a>(
    url_1: &'a str,
    url_2: &'a str,
    timeout: Option<Duration>,
) -> Result<&'a str, &'static str> {
    let client = Client::new();

    tokio::select! {
        Ok(_) = ping(&client, url_1, timeout) => {
            Ok(url_1)
        }
        Ok(_) = ping(&client, url_2, timeout) => {
            Ok(url_2)
        }
        else => {
            Err("no successful response received")
        }
    }
}

async fn ping<'a>(client: &'a Client, url: &str, timeout: Option<Duration>) -> Result<(), &'a str> {
    let mut request = client.get(url);
    if let Some(timeout) = timeout {
        request = request.timeout(timeout);
    }
    let _ = request.send().await.map_err(|_| "failed to send request")?;
    Ok(())
}
```

This works, but we need to change all callers and tests to pass the timeout. Before rushing in, let's consider:

- Do we care about the timeout in the "happy" test?
- The requirements were explicit about the timeout

Given this knowledge, let's refactor to accommodate both our tests and code users.

```rust,ignore
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn race<'a>(url_1: &'a str, url_2: &'a str) -> Result<&'a str, &'static str> {
    race_with_configuration(url_1, url_2, Some(DEFAULT_TIMEOUT)).await
}

pub async fn race_with_configuration<'a>(
    url_1: &'a str,
    url_2: &'a str,
    timeout: Option<Duration>,
) -> Result<&'a str, &'static str> {
    let client = Client::new();

    tokio::select! {
        Ok(_) = ping(&client, url_1, timeout) => {
            Ok(url_1)
        }
        Ok(_) = ping(&client, url_2, timeout) => {
            Ok(url_2)
        }
        else => {
            Err("no successful response received")
        }
    }
}

async fn ping<'a>(client: &'a Client, url: &str, timeout: Option<Duration>) -> Result<(), &'a str> {
    let mut request = client.get(url);
    if let Some(timeout) = timeout {
        request = request.timeout(timeout);
    }
    let _ = request.send().await.map_err(|_| "failed to send request")?;
    Ok(())
}
```

Our users and tests can use `race` (which uses `race_with_configuration` under the hood) while our sad path test can use `race_with_configuration`.

```rust,ignore
#[tokio::test]
async fn sut_returns_error_if_a_server_does_not_respond_within_timeout() {
    // Arrange
    let timeout = Some(Duration::from_millis(20));
    let url_1 = arrange_server(Some(Duration::from_millis(22))).await;
    let url_2 = arrange_server(Some(Duration::from_millis(22))).await;

    // Act
    let actual = race_with_configuration(&url_1, &url_2, timeout)
        .await
        .unwrap_err();

    // Assert
    let expected = "no successful response received";
    assert_eq!(expected, actual);
}
```

## Wrapping Up

### Select

- Helps you wait on multiple asynchronous operations
- Sometimes you'll want to include sleep in branches to prevent infinite blocking

### Wiremock

- A convenient way to create test servers for reliable and controllable tests
- Uses the same interfaces as real HTTP servers, providing consistency with less to learn
