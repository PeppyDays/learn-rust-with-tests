const REPEAT_COUNT: usize = 10;

pub fn repeat(c: &str) -> String {
    let mut repeated = String::with_capacity(REPEAT_COUNT * c.len());
    for _ in 0..REPEAT_COUNT {
        repeated.push_str(c);
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
