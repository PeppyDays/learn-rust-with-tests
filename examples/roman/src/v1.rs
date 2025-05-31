pub fn convert_to_roman(arabic: usize) -> String {
    "I".to_string()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod specs_for_convert_to_roman {
    use super::convert_to_roman;

    #[test]
    fn sut_converts_1_to_I() {
        // Arrange
        let arabic = 1;

        // Act
        let actual = convert_to_roman(arabic);

        // Assert
        let expected = "I";
        assert_eq!(expected, actual);
    }
}
