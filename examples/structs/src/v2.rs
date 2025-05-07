pub fn perimeter(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}

pub fn area(width: f64, height: f64) -> f64 {
    width * height
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

#[cfg(test)]
mod specs_for_area {
    use super::area;

    #[test]
    fn sut_returns_area_correctly() {
        // Arrange
        let width = 12.0;
        let height = 6.0;

        // Act
        let actual = area(width, height);

        // Assert
        let expected = 72.0;
        assert_eq!(expected, actual);
    }
}
