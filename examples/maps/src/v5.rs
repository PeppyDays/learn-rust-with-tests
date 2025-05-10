use std::collections::HashMap;
use std::collections::hash_map::Entry;

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

    pub fn add(&mut self, key: String, value: String) -> Result<(), DictionaryError> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(_) => Err(DictionaryError::AlreadyExists(key)),
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }

    pub fn update(&mut self, key: String, value: String) -> Result<(), DictionaryError> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                Ok(())
            }
            Entry::Vacant(_) => Err(DictionaryError::NotFound(key)),
        }
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

    #[error("the key '{0}' already exists")]
    AlreadyExists(String),
}

#[cfg(test)]
mod specs_for_dictionary_update {
    use super::Dictionary;
    use super::DictionaryError;

    #[test]
    fn sut_returns_ok_and_the_value_is_updated_correctly() {
        // Arrange
        let mut dictionary = Dictionary::from([("test".to_string(), "value1".to_string())]);

        // Act
        dictionary
            .update("test".to_string(), "value2".to_string())
            .unwrap();

        // Assert
        let actual = dictionary.search("test").unwrap();
        assert_eq!("value2", actual);
    }

    #[test]
    fn sut_returns_not_found_error_if_key_does_not_exists() {
        // Arrange
        let mut dictionary = Dictionary::new();

        // Act
        let actual = dictionary
            .update("test".to_string(), "value".to_string())
            .unwrap_err();

        // Assert
        assert!(matches!(actual, DictionaryError::NotFound(_)));
        assert_eq!(actual.to_string(), "the key 'test' was not found");
    }
}

#[cfg(test)]
mod specs_for_dictionary_add {
    use super::Dictionary;
    use super::DictionaryError;

    #[test]
    fn sut_returns_ok_and_able_to_search_the_entry() {
        // Arrange
        let mut dictionary = Dictionary::new();

        // Act
        dictionary
            .add("test".to_string(), "value".to_string())
            .unwrap();

        // Assert
        let actual = dictionary.search("test").unwrap();
        assert_eq!("value", actual);
    }

    #[test]
    fn sut_raises_already_exists_error_if_entry_already_exists() {
        // Arrange
        let mut dictionary = Dictionary::from([("test".to_string(), "value1".to_string())]);

        // Act
        let actual = dictionary
            .add("test".to_string(), "value2".to_string())
            .unwrap_err();

        // Assert
        assert!(matches!(actual, DictionaryError::AlreadyExists(_)));
        assert_eq!(actual.to_string(), "the key 'test' already exists");
    }
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

