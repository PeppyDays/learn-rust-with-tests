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
