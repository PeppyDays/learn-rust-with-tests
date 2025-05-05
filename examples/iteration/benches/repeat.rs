use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;

use iteration::repeat;

pub fn bench_repeat(c: &mut Criterion) {
    c.bench_function("repeat a", |b| {
        b.iter(|| {
            let _ = repeat(black_box("a"));
        })
    });
}

criterion_group!(benches, bench_repeat);
criterion_main!(benches);
