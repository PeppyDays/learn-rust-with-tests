use std::collections::HashMap;

pub struct Dictionary(HashMap<String, String>);

impl Dictionary {
    pub fn search(&self, key: &str) -> &str {
        match self.0.get(key) {
            Some(value) => value,
            None => panic!("not defined!"),
        }
    }
}

impl<const N: usize> From<[(String, String); N]> for Dictionary {
    fn from(entries: [(String, String); N]) -> Self {
        Dictionary(HashMap::from(entries))
    }
}

#[cfg(test)]
mod specs_for_dictionary_search {
    use super::Dictionary;

    #[test]
    fn sut_returns_value_if_key_exists_correctly() {
        // Arrange
        let dictionary = Dictionary::from([("test".to_string(), "value".to_string())]);

        // Act
        let actual = dictionary.search("test");

        // Assert
        assert_eq!(actual, "value");
    }
}
