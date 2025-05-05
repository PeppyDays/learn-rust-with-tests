/// `add` takes two integers and returns the sum of them.
///
/// ```
/// use integers::add;
///
/// let sum = add(1, 5);
/// assert_eq!(6, sum);
/// ```
pub fn add(x: i64, y: i64) -> i64 {
    x + y
}

#[cfg(test)]
mod specs_for_add {
    use super::add;

    #[test]
    fn sut_returns_4_if_arguments_are_2_and_2() {
        // Arrange
        let x = 2;
        let y = 2;

        // Act
        let actual = add(x, y);

        // Assert
        let expected = 4;
        assert_eq!(expected, actual);
    }
}
