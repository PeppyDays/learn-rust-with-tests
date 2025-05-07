pub fn perimeter(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}

#[cfg(test)]
mod specs_for_perimeter {
    use super::perimeter;

    #[test]
    fn sut_returns_perimeter_correctly() {
        // Arrange
        let width = 10.0;
        let height = 10.0;

        // Act
        let actual = perimeter(width, height);

        // Assert
        let expected = 40.0;
        assert_eq!(expected, actual);
    }
}
