use std::time::Duration;

use reqwest::Client;

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

#[cfg(test)]
mod specs_for_race {
    use std::time::Duration;

    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    use super::race;
    use super::race_with_configuration;

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

    async fn arrange_server(delay: Option<Duration>) -> String {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_delay(delay.unwrap_or_default()))
            .mount(&server)
            .await;
        server.uri()
    }

    fn arrange_server_with_error() -> String {
        "http://non-existent.url".to_string()
    }
}
