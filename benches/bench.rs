use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sortnet::sortnet;

fn sort_baseline(arr: Vec<i64>) {
    let mut arr = arr;
    arr.sort();
}

fn sort_sortnet(arr: Vec<i64>) {
    let mut arr = arr;
    sortnet(&mut arr[..]);
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort");
    for size in 0i64..=16 {
        // TODO: modes (sorted, reverse, shuffled)
        let arr: Vec<i64> = (0..size).rev().collect();

        group.bench_with_input(BenchmarkId::new("native", size), &arr, |b, arr| {
            b.iter(|| sort_baseline(arr.clone()))
        });
        group.bench_with_input(BenchmarkId::new("sortnet", size), &arr, |b, arr| {
            b.iter(|| sort_sortnet(arr.clone()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
