pub fn convert_to_roman(arabic: usize) -> String {
    let mut roman = String::new();
    for _ in 0..arabic {
        roman.push('I');
    }
    roman
}

#[cfg(test)]
mod specs_for_convert_to_roman {
    use super::convert_to_roman;

    #[rstest::rstest]
    #[case(1, "I")]
    #[case(2, "II")]
    #[case(3, "III")]
    fn sut_converts_arabic_to_roman_correctly(#[case] arabic: usize, #[case] expected: &str) {
        // Act
        let actual = convert_to_roman(arabic);

        // Assert
        assert_eq!(expected, actual);
    }
}
