use std::time::SystemTime;

pub mod v1;

#[derive(Debug, Default, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct ClockFace;

impl ClockFace {
    fn second_hand(&self) -> Point {
        Point { x: 150, y: 60 }
    }
}

impl From<SystemTime> for ClockFace {
    fn from(time: SystemTime) -> Self {
        ClockFace
    }
}

#[cfg(test)]
mod specs_for_clock {
    use std::time::Duration;
    use std::time::SystemTime;

    use chrono::NaiveDate;
    use chrono::NaiveDateTime;
    use chrono::NaiveTime;
    use chrono::Timelike;
    use fake::Fake;
    use fake::faker::chrono::en::Date;

    use super::ClockFace;
    use super::Point;

    #[test]
    fn sut_makes_second_hands_to_12_when_midnight() {
        // Arrange
        let time = midnight();

        // Act
        let actual = ClockFace::from(time).second_hand();

        // Assert
        let expected = Point::new(150, 150 - 90);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_makes_second_hands_to_6_when_30_seconds() {
        // Arrange
        let time = midnight() + Duration::from_secs(30);

        // Act
        let actual = ClockFace::from(time).second_hand();

        // Assert
        let expected = Point::new(150, 150 + 90);
        assert_eq!(expected, actual);
    }

    fn arrange_datetime(hour: u32, minute: u32, second: u32) -> NaiveDateTime {
        let date = Date().fake::<NaiveDate>();
        let time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();
        NaiveDateTime::new(date, time)
    }

    fn midnight() -> SystemTime {
        let secs_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let secs_after_midnight = secs_now % 86400;
        SystemTime::UNIX_EPOCH + Duration::from_secs(secs_now - secs_after_midnight)
    }
}
