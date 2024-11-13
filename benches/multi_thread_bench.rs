use criterion::{criterion_group, criterion_main, Criterion};
use multithreading::{converting, multi_thread, multi_thread_rayon, multi_thread_tokio};

fn multi_thread_tokio_bench(c: &mut Criterion) {
    let nums = (1..1000).collect::<Vec<u32>>();
    // let nums = vec![1, 2, 3, 100];
    c.bench_function("multi_thread_tokio_bench", |b| {
        b.iter(|| async {
            std::hint::black_box(multi_thread_tokio(nums.clone(), converting).await);
        });
    });
}

fn multi_thread_bench(c: &mut Criterion) {
    let nums = (1..1000).collect::<Vec<u32>>();
    // let nums = vec![1, 2, 3, 100];
    c.bench_function("multi_thread_bench", |b| {
        b.iter(|| {
            std::hint::black_box(multi_thread(nums.clone(), converting));
        });
    });
}

fn multi_thread_rayon_bench(c: &mut Criterion) {
    let nums = (1..1000).collect::<Vec<u32>>();
    // let nums = vec![1, 2, 3, 100];
    c.bench_function("multi_thread_rayon_bench", |b| {
        b.iter(|| {
            std::hint::black_box(multi_thread_rayon(nums.clone(), converting));
        });
    });
}

criterion_group!(
    benches,
    multi_thread_bench,
    multi_thread_tokio_bench,
    multi_thread_rayon_bench
);
criterion_main!(benches);
