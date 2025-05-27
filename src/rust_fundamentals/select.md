# Select

<!-- TODO: Remove future!! -->

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/select).

## The First Requirement: Find Faster URL

You have been asked to make a function called WebsiteRacer which takes two URLs and "races" them by hitting them with an HTTP GET and returning the URL which returned first. If none of them return within 10 seconds then it should return an error.

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

We know this isn't perfect and has problems, but it's a start. It's important not to get too hung-up on getting things perfect first time.

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

To make a HTTP request, we will use the reqwest crate. The crate supports asynchronous call, so let's use tokio and futures crates together. First, we need to add it to our `Cargo.toml`:

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

- We use `std::time::Instant::now` to record just before we try and get the URL
- Then we use `reqwest::Client.send` to try and perform an HTTP GET request against the URL
  - This function returns an `Result<Response, Error>`, but so far we are not interested in these values
- `std::time::Instant.elapsed` returns a duration of the difference

Once we have done this we simply compare the durations to see which is the quickest.

### Problems

This may or may not make the test pass for you. The problem is we're reaching out to real websites to test our own logic.

Testing code that uses HTTP is so common that Rust has a crate called [wiremock](https://docs.rs/wiremock/latest/wiremock/) which allows us to create mock HTTP servers that we can control.

In the [mocking](./mocking.md) and [dependency injection](./dependency_injection.md) chapters, we covered how ideally we don't want to be relying on external services to test our code because they can be:

- Slow
- Flaky
- Can't test edge cases

Let's change our tests to use mocks so we have reliable servers to test against that we can control.

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

The syntax may look a bit busy but just take your time.

`MockServer::start` starts a new mock HTTP server in local laptop with actual port, which we can use to send requests to. It returns a `MockServer` instance that we can use to configure the server.

We then use `Mock::given` to set up a mock for the server. We specify the HTTP method and path we want to match against, and then we use `respond_with` to define what the server should respond with when it receives a request that matches our criteria. You can see that we set a delay of 20 milliseconds for the slow server, which simulates a slow response. Finally, we mount the mock to the server using `mount`, which makes it active and ready to respond to requests.

It turns out there's really no extra magic here. `MockServer` which makes it easier to use with testing, as it finds an open port to listen on and then you can close it when you're done with your test.

If you re-run the test it will definitely pass now and should be faster. Play with these sleeps to deliberately break the test.

### Refactor

We have some duplication in both our production code and test code.

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

We've refactored creating our mock servers into a function called `arrange_server` to move some uninteresting code out of the test and reduce repetition.

## The Second Requirement: Synchronize Finding Processes

Why are we testing the speeds of the websites one after another when Rust is great at concurrency? We should be able to check both at the same time. We don't really care about the exact response times of the requests, we just want to know which one comes back first.

To do this, we're going to introduce a new construct called `select` which helps us synchronise processes really easily and clearly.

We're trying to refactor our `race` function to use `select`, so that no need to write a new test for it. We can just change the implementation and see if it still passes.

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

We have defined a function `ping` which creates checks the URL. If sending a request fails, it returns an error. If it succeeds, it returns `Ok(())` because we don't care about the response code or body.

The `tokio::select!` macro allows us to wait for multiple asynchronous operations to complete. It will return as soon as one of the operations completes, and we can handle the result accordingly. The `tokio::select!` macro can handle more than two branches. Each branch is structured as:

```plain
<pattern> = <async expression> => <handler>,
```

When the macro is evaluated, all the `<async expression>`s are aggregated and executed concurrently. When an expression completes, the result is matched against `<pattern>`. If the result matches the pattern, then all remaining async expressions are dropped and `<handler>` is executed.

If `<pattern>` does not match the result of the asynchronous computation, then the remaining asynchronous expressions continue to execute concurrently until the next one completes. At this time, the same logic is applied to that result.

In our select macro, we have two branches, one for each URL. If the response from the first URL comes first and is successful, we return `Ok(url_1)`. If the response from the second URL comes first and is successful, we return `Ok(url_2)`. If neither URL returns a successful response, we return an error.

After these changes, the intent behind our code is very clear and the implementation is actually simpler.

## The Third Requirement: Ignore Sending Request Error Branch

In our current implementation, what happens if sending a request fails? Suppose that the first URL is down or not reachable. In that case, the `ping` function will return an error, and the `tokio::select!` macro will consume the first branch and leave the select macro, and return the error.

But the behaviour is not what we want. Even though the first URL failed, we still want to check the second URL. If the second URL is successful, we should return it.

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

In the first test, we set the first URL to a non-existent URL, and the second URL to a mock server that will respond after 20 milliseconds. The first URL will fail to send a request before the second URL responds. We want to ensure that the failure from the first URL is ignored, and the second URL is returned as the result.

The second test checks that if both URLs fail to send a request, we return an error.

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

Do you remember that select macro binds the result of the asynchronous expression to a pattern? If the pattern does not match, the branch is dropped and the select macro continues to the next branch. By using this property, we can ignore the result of the first ping when it fails, and keep waiting for the second ping to complete. The else branch is executed when all branches are dropped, which means that both pings failed.

Run the tests again and they should pass.

### Refactor

Nothing special to refactor, but we can just extract the error case in the tests.

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

## The Forth Requirement: Timeout After 10 Seconds

Our final requirement was to return an error if Racer takes longer than 10 seconds.

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

We've made our test servers take longer than 10s to return to exercise this scenario.

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

By putting a timeout on the request, we can ensure that if the request takes longer than 10 seconds, it will return an error.

You can also put a timeout on the `tokio::select!` macro itself, but in this case, there should be an error branch when the two branches are failed to send a request. So the only place we can modify the timeout is in the `ping` function.

### Refactor

The problem we have is that this test takes 10 seconds to run. For such a simple bit of logic, this doesn't feel great.

What we can do is make the timeout configurable. So in our test, we can have a very short timeout and then when the code is used in the real world it can be set to 10 seconds.

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

This works, but we need to change all our caller and tests to pass the timeout in. Before rushing in, let's listen to them.

- Do we care about the timeout in the "happy" test?
- The requirements were explicit about the timeout

Given this knowledge, let's do a little refactoring to be sympathetic to both our tests and the users of our code.

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

Our users and our tests can use `race` (which uses `race_with_configuration` under the hood) and our sad path test can use `race_with_configuration`.

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
- Sometimes you'll want to include sleep in one of your branches to prevent your system blocking forever

### Wiremock

- A convenient way of creating test servers so you can have reliable and controllable tests
- Uses the same interfaces as the "real" http servers which is consistent and less for you to learn
