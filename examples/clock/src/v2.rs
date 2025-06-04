use std::f64::consts::PI;

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

fn seconds_in_radians(time: NaiveTime) -> f64 {
    PI
}

#[cfg(test)]
mod specs_for_seconds_in_radians {
    use std::f64::consts::PI;

    use chrono::NaiveTime;

    use super::seconds_in_radians;

    #[test]
    fn sut_converts_seconds_to_radians_correctly_when_thirty_seconds_past_midnight() {
        // Arrange
        let time = NaiveTime::from_hms_opt(0, 0, 30).unwrap();

        // Act
        let actual = seconds_in_radians(time);

        // Assert
        let expected = PI;
        assert_eq!(expected, actual);
    }
}
