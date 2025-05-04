pub fn greet() -> String {
    String::from("Hello, World!")
}

#[cfg(test)]
mod specs_for_greet {
    use super::greet;

    #[test]
    fn sut_returns_hello_world_correctly() {
        // Act
        let actual = greet();

        // Assert
        let expected = "Hello, World!";
        assert_eq!(expected, actual);
    }
}
