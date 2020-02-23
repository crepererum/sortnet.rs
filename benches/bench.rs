use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;
use sortnet::sortnet;

fn sort_baseline(arr: Vec<i64>) {
    let mut arr = arr;
    arr.sort();
}

fn sort_sortnet(arr: Vec<i64>) {
    let mut arr = arr;
    sortnet(&mut arr[..]);
}

fn bench_i64_generic<F>(c: &mut Criterion, name: &str, fgen: F)
where
    F: Fn(i64) -> Vec<i64>,
{
    let mut group = c.benchmark_group(name);
    for size in 0i64..=16 {
        let arr = fgen(size);

        group.bench_with_input(BenchmarkId::new("native", size), &arr, |b, arr| {
            b.iter(|| sort_baseline(arr.clone()))
        });
        group.bench_with_input(BenchmarkId::new("sortnet", size), &arr, |b, arr| {
            b.iter(|| sort_sortnet(arr.clone()))
        });
    }
    group.finish();
}

fn bench_i64_sorted(mut c: &mut Criterion) {
    bench_i64_generic(&mut c, "i64 sorted", |size| (0..size).collect());
}

fn bench_i64_reversed(mut c: &mut Criterion) {
    bench_i64_generic(&mut c, "i64 reversed", |size| (0..size).rev().collect());
}

fn bench_i64_shuffled(mut c: &mut Criterion) {
    bench_i64_generic(&mut c, "i64 shuffle", |size| {
        let mut tmp: Vec<i64> = (0..size).collect();
        let mut rng = thread_rng();
        tmp.shuffle(&mut rng);
        tmp
    });
}

criterion_group!(
    benches,
    bench_i64_sorted,
    bench_i64_reversed,
    bench_i64_shuffled
);
criterion_main!(benches);
