use std::collections::HashMap;

use futures::future::join_all;
use tokio::task::spawn_blocking;

type WebsiteChecker = fn(&str) -> bool;

pub async fn check_website(urls: &[&str], checker: WebsiteChecker) -> HashMap<String, bool> {
    let results = join_all(urls.iter().map(|&url| {
        let url = url.to_string();
        spawn_blocking(move || {
            let is_up = checker(&url);
            (url, is_up)
        })
    }))
    .await;

    let mut records = HashMap::new();
    results.into_iter().for_each(|result| {
        if let Ok((url, is_up)) = result {
            records.insert(url, is_up);
        }
    });
    records
}

#[cfg(test)]
mod specs_for_check_website {
    use std::collections::HashMap;

    use super::check_website;

    #[tokio::test]
    async fn t1() {
        // Arrange
        let urls = [
            "http://google.com",
            "http://blog.gypsydave5.com",
            "waat://furhurterwe.geds",
        ];
        let checker = |url: &str| url != "waat://furhurterwe.geds";

        // Act
        let actual = check_website(&urls, checker).await;

        // Assert
        let expected = HashMap::from([
            ("http://google.com".to_string(), true),
            ("http://blog.gypsydave5.com".to_string(), true),
            ("waat://furhurterwe.geds".to_string(), false),
        ]);
        assert_eq!(expected, actual);
    }
}
