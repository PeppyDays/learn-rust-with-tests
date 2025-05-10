use std::collections::HashMap;

#[derive(Default)]
pub struct Dictionary(HashMap<String, String>);

impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        self.0
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| DictionaryError::NotFound(key.to_string()))
    }
}

impl<const N: usize> From<[(String, String); N]> for Dictionary {
    fn from(entries: [(String, String); N]) -> Self {
        Dictionary(HashMap::from(entries))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DictionaryError {
    #[error("the key '{0}' was not found")]
    NotFound(String),
}

#[cfg(test)]
mod specs_for_dictionary_search {
    use super::Dictionary;
    use super::DictionaryError;

    #[test]
    fn sut_returns_ok_with_value_if_key_exists_correctly() {
        // Arrange
        let dictionary = Dictionary::from([("test".to_string(), "value".to_string())]);

        // Act
        let actual = dictionary.search("test").unwrap();

        // Assert
        assert_eq!(actual, "value");
    }

    #[test]
    fn sut_returns_not_found_error_if_key_does_not_exists() {
        // Arrange
        let dictionary = Dictionary::new();

        // Act
        let actual = dictionary.search("test").unwrap_err();

        // Assert
        assert!(matches!(actual, DictionaryError::NotFound(_)));
        assert_eq!(actual.to_string(), "the key 'test' was not found");
    }
}
