pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub fn perimeter(rectangle: Rectangle) -> f64 {
    2.0 * (rectangle.width + rectangle.height)
}

pub fn area(rectangle: Rectangle) -> f64 {
    rectangle.width * rectangle.height
}

#[cfg(test)]
mod specs_for_perimeter {
    use super::Rectangle;
    use super::perimeter;

    #[test]
    fn sut_returns_perimeter_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 10.0,
            height: 10.0,
        };

        // Act
        let actual = perimeter(rectangle);

        // Assert
        let expected = 40.0;
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod specs_for_area {
    use super::Rectangle;
    use super::area;

    #[test]
    fn sut_returns_area_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 12.0,
            height: 6.0,
        };

        // Act
        let actual = area(rectangle);

        // Assert
        let expected = 72.0;
        assert_eq!(expected, actual);
    }
}
