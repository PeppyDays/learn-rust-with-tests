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
