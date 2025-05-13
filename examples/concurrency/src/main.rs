use std::thread::sleep;
use std::time::Duration;

use concurrency::check_website;

#[tokio::main]
async fn main() {
    let urls = (0..10).map(|_| "http://example.com").collect::<Vec<_>>();
    check_website(&urls, slow_website_checker_stub).await;
}

fn slow_website_checker_stub(_: &str) -> bool {
    println!("start!");
    sleep(Duration::from_secs(1));
    println!("finish!");
    true
}
