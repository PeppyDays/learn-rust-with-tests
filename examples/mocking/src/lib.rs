pub mod v1;
pub mod v2;
pub mod v3;
pub mod v4;
pub mod v5;

use std::io::Write;
use std::time::Duration;

const COUNTDOWN_START: usize = 3;
const FINAL_WORD: &str = "Go!";

pub trait Sleeper {
    fn sleep(&self);
}

pub struct ConfigurableSleeper {
    duration: Duration,
    sleep_function: Box<dyn Fn(Duration)>,
}

impl ConfigurableSleeper {
    pub fn new(duration: Duration, sleep_function: impl Fn(Duration) + 'static) -> Self {
        ConfigurableSleeper {
            duration,
            sleep_function: Box::new(sleep_function),
        }
    }
}

impl Sleeper for ConfigurableSleeper {
    fn sleep(&self) {
        (self.sleep_function)(self.duration);
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
mod specs_for_configurable_sleeper {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::time::Duration;

    use super::ConfigurableSleeper;
    use super::Sleeper;

    #[test]
    fn sut_delivers_duration_to_sleep_function_correctly() {
        // Arrange
        let recorder = Rc::new(RefCell::new(Duration::from_secs(0)));
        let duration_spy = recorder.clone();
        let sut = ConfigurableSleeper::new(Duration::from_secs(5), move |duration| {
            *duration_spy.borrow_mut() = duration;
        });

        // Act
        sut.sleep();

        // Assert
        let actual = recorder.take();
        let expected = Duration::from_secs(5);
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod specs_for_countdown {
    use std::cell::RefCell;
    use std::io::Write;
    use std::io::stdout;
    use std::thread::sleep;
    use std::time::Duration;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use super::Sleeper;
    use super::countdown;

    struct CountdownOperationsSpy {
        sleep_command: &'static str,
        write_command: &'static str,
        calls: RefCell<Vec<(u128, &'static str)>>,
    }

    impl CountdownOperationsSpy {
        fn new() -> Self {
            CountdownOperationsSpy {
                sleep_command: "sleep",
                write_command: "write",
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl Sleeper for CountdownOperationsSpy {
        fn sleep(&self) {
            let mut calls = self.calls.borrow_mut();
            calls.push((get_current_timestamp(), self.sleep_command));
            sleep(Duration::from_millis(1));
        }
    }

    impl Write for CountdownOperationsSpy {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut calls = self.calls.borrow_mut();
            calls.push((get_current_timestamp(), self.write_command));
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn get_current_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }

    #[test]
    fn sut_writes_3_2_1_go() {
        // Arrange
        let mut buffer = Vec::new();
        let sleeper_dummy = CountdownOperationsSpy::new();

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
        let sleeper_spy = CountdownOperationsSpy::new();

        // Act
        countdown(&mut stdout(), &sleeper_spy);

        // Assert
        assert_eq!(sleeper_spy.calls.borrow().len(), 3);
    }

    #[test]
    fn sut_sleeps_after_writing() {
        // Arrange
        let sleeper_spy = CountdownOperationsSpy::new();
        let mut writer_spy = CountdownOperationsSpy::new();

        // Act
        countdown(&mut writer_spy, &sleeper_spy);

        // Assert
        let merge_operations = |spy_1: CountdownOperationsSpy, spy_2: CountdownOperationsSpy| {
            let mut operations = spy_1.calls.borrow_mut().clone();
            operations.extend(spy_2.calls.borrow_mut().clone());
            operations.sort_by_key(|key| key.0);
            operations
        };
        let operations = merge_operations(writer_spy, sleeper_spy);
        let actual: Vec<&str> = operations.into_iter().map(|(_, command)| command).collect();
        let expected = vec![
            "write", "sleep", "write", "sleep", "write", "sleep", "write",
        ];
        assert_eq!(expected, actual);
    }
}
