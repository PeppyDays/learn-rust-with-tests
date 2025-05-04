const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";

pub fn greet(name: &str) -> String {
    format!("{}{}!", GREETING_PREFIX_FOR_ENGLISH, name)
}

#[cfg(test)]
mod specs_for_greet {
    use super::greet;

    #[test]
    fn sut_returns_hello_with_given_name_correctly() {
        // Act
        let actual = greet("Chris");

        // Assert
        let expected = "Hello, Chris!";
        assert_eq!(expected, actual);
    }
}
