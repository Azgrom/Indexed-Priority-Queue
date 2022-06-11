use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use indexed_priority_queue::MinIndexedPriorityQueue;
use std::collections::BinaryHeap;

fn ten_million_sequential_integers_benchmark(c: &mut Criterion) {
    let mut v = (10i32.pow(3)..0).map(|i| i).collect::<Vec<i32>>();
    let mut group = c.benchmark_group("Binary Heaps Comparison");

    group.bench_with_input(
        BenchmarkId::new("Custom Heap", "Thousand inverse sequential elements"),
        &v,
        |b, v| {
            let mut c = v.clone();
            MinIndexedPriorityQueue::from(&mut c);
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Original Heap", "Thousand inverse sequential elements"),
        &v,
        |b, v| {
            BinaryHeap::from(v.clone());
        },
    );

    group.finish();
}

criterion_group!(benches, ten_million_sequential_integers_benchmark);
criterion_main!(benches);
