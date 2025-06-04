use chrono::NaiveTime;

use clock::v5::ClockFace;
use clock::v5::Point;

#[rstest::rstest]
#[case(0, 0, 30, Point::new(0.0, -1.0))]
#[case(0, 0, 45, Point::new(-1.0, 0.0))]
fn sut_indicates_second_hand_coordinate_correctly(
    #[case] hours: u32,
    #[case] minutes: u32,
    #[case] seconds: u32,
    #[case] expected: Point,
) {
    // Arrange
    let time = NaiveTime::from_hms_opt(hours, minutes, seconds).unwrap();

    // Act
    let actual = ClockFace::from(time).second_hand();

    // Assert
    assert_equal_point(expected, actual);
}

fn assert_equal_point(expected: Point, actual: Point) {
    let equality_threshold = 0.00000001;
    assert!(
        (expected.x - actual.x).abs() < equality_threshold
            && (expected.y - actual.y).abs() < equality_threshold
    )
}
