pub fn repeat(c: &str) -> String {
    let mut repeated = String::new();
    for _ in 0..10 {
        repeated = repeated + c;
    }
    repeated
}

#[cfg(test)]
mod specs_for_repeat {
    use super::repeat;

    #[test]
    fn sut_repeats_given_argument_10_times() {
        // Arrange
        let input = "a";

        // Act
        let actual = repeat(input);

        // Assert
        let expected = "aaaaaaaaaa";
        assert_eq!(expected, actual);
    }
}
