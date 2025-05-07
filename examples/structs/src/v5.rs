use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

pub fn sum_areas(shapes: &[&dyn Shape]) -> f64 {
    let mut total_area = 0.0;
    for shape in shapes {
        total_area += shape.area();
    }
    total_area
}

#[cfg(test)]
mod specs_for_sum_areas {
    use super::Shape;
    use super::sum_areas;

    #[test]
    fn sut_returns_sum_of_areas_if_rectangle_and_circle_are_given() {
        // Arrange
        let rectangle = super::Rectangle {
            width: 10.0,
            height: 10.0,
        };
        let circle = super::Circle { radius: 10.0 };
        let shapes: Vec<&dyn Shape> = vec![&rectangle, &circle];

        // Act
        let actual = sum_areas(&shapes);

        // Assert
        let expected = 414.1592653589793;
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod specs_for_shape {
    use rstest::rstest;

    use super::Circle;
    use super::Rectangle;
    use super::Shape;

    #[rstest]
    #[case(Rectangle {width: 10.0, height: 10.0}, 40.0)]
    #[case(Circle {radius: 10.0}, 62.83185307179586)]
    fn sut_returns_perimeter_of_shape_correctly(#[case] shape: impl Shape, #[case] expected: f64) {
        // Act
        let actual = shape.perimeter();

        // Assert
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(Rectangle {width: 12.0, height: 6.0}, 72.0)]
    #[case(Circle {radius: 10.0}, 314.1592653589793)]
    fn sut_returns_area_of_shape_correctly(#[case] shape: impl Shape, #[case] expected: f64) {
        // Act
        let actual = shape.area();

        // Assert
        assert_eq!(expected, actual);
    }
}
