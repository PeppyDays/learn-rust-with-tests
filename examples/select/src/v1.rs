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
