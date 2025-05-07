use std::f64::consts::PI;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

#[cfg(test)]
mod specs_for_perimeter {
    use super::Circle;
    use super::Rectangle;

    #[test]
    fn sut_returns_perimeter_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 10.0,
            height: 10.0,
        };

        // Act
        let actual = rectangle.perimeter();

        // Assert
        let expected = 40.0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_perimeter_of_circle_correctly() {
        // Arrange
        let circle = Circle { radius: 10.0 };

        // Act
        let actual = circle.perimeter();

        // Assert
        let expected = 62.83185307179586;
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod specs_for_area {
    use super::Circle;
    use super::Rectangle;

    #[test]
    fn sut_returns_area_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 12.0,
            height: 6.0,
        };

        // Act
        let actual = rectangle.area();

        // Assert
        let expected = 72.0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_area_of_circle_correctly() {
        // Arrange
        let circle = Circle { radius: 10.0 };

        // Act
        let actual = circle.area();

        // Assert
        let expected = 314.1592653589793;
        assert_eq!(expected, actual);
    }
}
