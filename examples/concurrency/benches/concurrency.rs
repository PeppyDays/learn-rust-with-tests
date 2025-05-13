use std::thread::sleep;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

use concurrency::check_website;
use tokio::runtime::Runtime;

pub fn bench_website_checker(c: &mut Criterion) {
    let urls = (0..10).map(|_| "http://example.com").collect::<Vec<_>>();

    c.bench_function("check_website", |b| {
        b.to_async(Runtime::new().unwrap()).iter(async || {
            check_website(&urls, slow_website_checker_stub).await;
        })
    });
}

criterion_group!(benches, bench_website_checker);
criterion_main!(benches);

fn slow_website_checker_stub(_: &str) -> bool {
    sleep(Duration::from_millis(20));
    true
}
