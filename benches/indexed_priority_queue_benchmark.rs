use criterion::{black_box, criterion_group, criterion_main, Criterion};
use indexed_priority_queue::MinIndexedPriorityQueue;
use std::ops::Range;

fn ten_million_sequential_integers_benchmark(c: &mut Criterion) {
    c.bench_function("10M reversely sequential i32's bench", |b| {
        b.iter(|| {
            let mut v = Range {
                start: 0,
                end: 10i32.pow(7),
            }
                .rev()
                .map(|i| i)
                .collect::<Vec<i32>>();
            MinIndexedPriorityQueue::from_existent_vec(black_box(&mut v));
        });
    });
}

criterion_group!(benches, ten_million_sequential_integers_benchmark);
criterion_main!(benches);
