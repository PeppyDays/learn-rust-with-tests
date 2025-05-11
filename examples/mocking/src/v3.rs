use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

const COUNTDOWN_START: usize = 3;
const FINAL_WORD: &str = "Go!";

pub trait Sleeper {
    fn sleep(&self);
}

pub struct DefaultSleeper;

impl Sleeper for DefaultSleeper {
    fn sleep(&self) {
        sleep(Duration::from_secs(1));
    }
}

pub fn countdown(out: &mut dyn Write, sleeper: &dyn Sleeper) {
    for i in (1..=COUNTDOWN_START).rev() {
        out.write_all(format!("{}\n", i).as_bytes()).unwrap();
        sleeper.sleep();
    }
    out.write_all(FINAL_WORD.as_bytes()).unwrap();
}

#[cfg(test)]
mod specs_for_countdown {
    use std::cell::RefCell;
    use std::io::stdout;

    use super::Sleeper;
    use super::countdown;

    struct SleeprSpy {
        calls: RefCell<usize>,
    }

    impl SleeprSpy {
        fn new() -> Self {
            SleeprSpy {
                calls: RefCell::new(0),
            }
        }
    }

    impl Sleeper for SleeprSpy {
        fn sleep(&self) {
            *self.calls.borrow_mut() += 1;
        }
    }

    #[test]
    fn sut_writes_3_2_1_go() {
        // Arrange
        let mut buffer = Vec::new();
        let sleeper_dummy = SleeprSpy::new();

        // Act
        countdown(&mut buffer, &sleeper_dummy);

        // Assert
        let actual = String::from_utf8(buffer).unwrap();
        let expected = "3\n2\n1\nGo!";
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_calls_sleep_3_times() {
        // Arrange
        let sleeper_spy = SleeprSpy::new();

        // Act
        countdown(&mut stdout(), &sleeper_spy);

        // Assert
        assert_eq!(*sleeper_spy.calls.borrow(), 3);
    }
}
