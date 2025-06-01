use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use mocking::v5::ConfigurableSleeper;
use mocking::v5::countdown;

fn main() {
    countdown(
        &mut stdout(),
        &ConfigurableSleeper::new(
            Duration::from_secs(1),
            Box::new(|duration| {
                sleep(duration);
            }),
        ),
    );
}
