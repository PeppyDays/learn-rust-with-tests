use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use mocking::ConfigurableSleeper;
use mocking::countdown;

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
