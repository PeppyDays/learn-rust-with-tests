use std::f64::consts::PI;

use chrono::NaiveTime;
use chrono::Timelike;

const SECOND_HAND_LENGTH: f64 = 90.0;
const CLOCK_CENTER_X: f64 = 150.0;
const CLOCK_CENTER_Y: f64 = 150.0;

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
pub struct ClockFace {
    time: NaiveTime,
}

impl ClockFace {
    pub fn second_hand(&self) -> Point {
        let angle = self.radian_of_second_hand();
        let x = angle.sin();
        let y = angle.cos();
        Point::new(
            SECOND_HAND_LENGTH * x + CLOCK_CENTER_X,
            -SECOND_HAND_LENGTH * y + CLOCK_CENTER_Y,
        )
    }

    fn radian_of_second_hand(&self) -> f64 {
        self.time.second() as f64 * (PI / 30.0)
    }
}

impl From<NaiveTime> for ClockFace {
    fn from(time: NaiveTime) -> Self {
        ClockFace { time }
    }
}
