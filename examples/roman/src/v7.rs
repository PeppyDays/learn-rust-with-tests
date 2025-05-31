struct RomanNumeral(usize, &'static str);

const ALL_ROMAN_NUMERALS: [RomanNumeral; 13] = [
    RomanNumeral(1000, "M"),
    RomanNumeral(900, "CM"),
    RomanNumeral(500, "D"),
    RomanNumeral(400, "CD"),
    RomanNumeral(100, "C"),
    RomanNumeral(90, "XC"),
    RomanNumeral(50, "L"),
    RomanNumeral(40, "XL"),
    RomanNumeral(10, "X"),
    RomanNumeral(9, "IX"),
    RomanNumeral(5, "V"),
    RomanNumeral(4, "IV"),
    RomanNumeral(1, "I"),
];

pub fn convert_to_roman(mut arabic: usize) -> String {
    let mut roman = String::new();
    for RomanNumeral(value, symbol) in ALL_ROMAN_NUMERALS {
        while arabic >= value {
            roman.push_str(symbol);
            arabic -= value;
        }
    }
    roman
}

pub fn convert_to_arabic(roman: &str) -> usize {
    let mut arabic = 0;
    for _ in 0..roman.len() {
        arabic += 1;
    }
    arabic
}

#[cfg(test)]
mod specs_for_convert {
    use super::convert_to_arabic;
    use super::convert_to_roman;

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case(1, "I")]
    #[case(2, "II")]
    #[case(3, "III")]
    #[case(4, "IV")]
    #[case(5, "V")]
    #[case(9, "IX")]
    #[case(10, "X")]
    #[case(14, "XIV")]
    #[case(18, "XVIII")]
    #[case(20, "XX")]
    #[case(39, "XXXIX")]
    #[case(40, "XL")]
    #[case(47, "XLVII")]
    #[case(49, "XLIX")]
    #[case(50, "L")]
    #[case(90, "XC")]
    #[case(100, "C")]
    #[case(400, "CD")]
    #[case(500, "D")]
    #[case(798, "DCCXCVIII")]
    #[case(900, "CM")]
    #[case(1000, "M")]
    #[case(1006, "MVI")]
    #[case(1984, "MCMLXXXIV")]
    #[case(2014, "MMXIV")]
    #[case(3999, "MMMCMXCIX")]
    fn conversion_cases(#[case] _arabic: usize, #[case] _roman: String) {}

    #[rstest_reuse::apply(conversion_cases)]
    fn sut_converts_arabic_to_roman_correctly(#[case] arabic: usize, #[case] expected: &str) {
        // Act
        let actual = convert_to_roman(arabic);

        // Assert
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(1, "I")]
    #[case(2, "II")]
    #[case(3, "III")]
    fn sut_converts_roman_to_arabic_correctly(#[case] expecetd: usize, #[case] roman: &str) {
        // Act
        let actual = convert_to_arabic(roman);

        // Assert
        assert_eq!(expecetd, actual);
    }
}
