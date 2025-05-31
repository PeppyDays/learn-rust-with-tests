pub fn convert_to_roman(mut arabic: usize) -> String {
    let mut roman = String::new();
    while arabic > 0 {
        match arabic {
            n if n >= 5 => {
                roman.push('V');
                arabic -= 5;
            }
            n if n >= 4 => {
                roman.push_str("IV");
                break;
            }
            _ => {
                roman.push('I');
                arabic -= 1;
            }
        }
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
    #[case(4, "IV")]
    #[case(5, "V")]
    fn sut_converts_arabic_to_roman_correctly(#[case] arabic: usize, #[case] expected: &str) {
        // Act
        let actual = convert_to_roman(arabic);

        // Assert
        assert_eq!(expected, actual);
    }
}
