use chrono::NaiveTime;

use clock::v3::ClockFace;
use clock::v3::Point;

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
