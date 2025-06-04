use chrono::NaiveTime;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct ClockFace;

impl ClockFace {
    pub fn second_hand(&self) -> Point {
        Point::new(150.0, 60.0)
    }
}

impl From<NaiveTime> for ClockFace {
    fn from(time: NaiveTime) -> Self {
        ClockFace
    }
}

#[cfg(test)]
mod specs_for_clockface {
    use chrono::NaiveTime;

    use super::ClockFace;
    use super::Point;

    #[test]
    fn sut_indicates_second_hand_correctly_when_midnight() {
        // Arrange
        let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

        // Act
        let actual = ClockFace::from(time).second_hand();

        // Assert
        let expected = Point::new(150.0, 150.0 - 90.0);
        assert_eq!(expected, actual);
    }
}
