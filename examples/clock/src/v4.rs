use std::f64::consts::PI;

use chrono::NaiveTime;
use chrono::Timelike;

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
    time.second() as f64 * (PI / 30.0)
}

#[cfg(test)]
mod specs_for_seconds_in_radians {
    use std::f64::consts::PI;

    use chrono::NaiveTime;

    use super::seconds_in_radians;

    #[rstest::rstest]
    #[case(0, 0, 0, 0.0)]
    #[case(0, 0, 15, PI / 2.0)]
    #[case(0, 0, 30, PI)]
    #[case(0, 0, 45, PI / 2.0 * 3.0)]
    #[case(0, 0, 7, PI / 30.0 * 7.0)]
    fn sut_converts_seconds_to_radians_correctly(
        #[case] hours: u32,
        #[case] minutes: u32,
        #[case] seconds: u32,
        #[case] expected: f64,
    ) {
        // Arrange
        let time = NaiveTime::from_hms_opt(hours, minutes, seconds).unwrap();

        // Act
        let actual = seconds_in_radians(time);

        // Assert
        assert_eq!(expected, actual);
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

    // #[test]
    // fn sut_indicates_second_hand_correctly_when_thirty_seconds_past_midnight() {
    //     // Arrange
    //     let time = NaiveTime::from_hms_opt(0, 0, 30).unwrap();
    //
    //     // Act
    //     let actual = ClockFace::from(time).second_hand();
    //
    //     // Assert
    //     let expected = Point::new(150.0, 150.0 + 90.0);
    //     assert_eq!(expected, actual);
    // }
}
