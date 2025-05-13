use std::collections::HashMap;

type WebsiteChecker = fn(&str) -> bool;

fn check_website(url: &[&str], checker: WebsiteChecker) -> HashMap<String, bool> {
    let mut results = HashMap::new();
    for &site in url {
        results.insert(site.to_string(), checker(site));
    }
    results
}

#[cfg(test)]
mod specs_for_check_website {
    use std::collections::HashMap;

    use super::check_website;

    #[test]
    fn name() {
        // Arrange
        let urls = [
            "http://google.com",
            "http://blog.gypsydave5.com",
            "waat://furhurterwe.geds",
        ];

        // Act
        let actual = check_website(&urls, |url| url != "waat://furhurterwe.geds");

        // Assert
        let expected = HashMap::from([
            ("http://google.com".to_string(), true),
            ("http://blog.gypsydave5.com".to_string(), true),
            ("waat://furhurterwe.geds".to_string(), false),
        ]);
        assert_eq!(expected, actual);
    }
}
