use std::time::Instant;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main};

fn make_items(size: usize) -> Vec<i32> {
    let size = size as i32;
    (1..size).rev().collect()
}

fn do_sort_bench(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    size: usize,
    items: &Vec<i32>,
    name: &str,
    mut sort: impl FnMut(&mut [i32]),
) {
    group.throughput(Throughput::Elements(size as u64));
    group.bench_with_input(BenchmarkId::new(name, size), &items, |b, my_items| {
        b.iter_custom(|iters| {
            // Make `iters` copies of our data before we start our timer
            // This way, we can time only the sorting algorithm
            // Because it sorts in place, we must do this.
            let mut items: Vec<Vec<_>> = (0..iters).map(|_| my_items.to_vec()).collect();

            let start = Instant::now();
            for xs in items.iter_mut() {
                sort(xs);
                black_box(xs);
            }

            start.elapsed()
        });
    });
}

fn sorting_i32s(c: &mut Criterion) {
    let mut group = c.benchmark_group("all sorts of sorts");

    let items_set: Vec<(usize, Vec<i32>)> = [
        0, 1, 2, 3, 4, 5, 10, 100, 1_000, 2_000,
        5_000,
        //     1_000_000,
        //     2_000_000,
        //     5_000_000,
        //     1_000_000_000,
        //     2_000_000_000,
        //     5_000_000_000,
    ]
    .iter()
    .copied()
    .map(|size| (size, make_items(size)))
    .collect();

    for (size, items) in &items_set {
        let size: usize = *size;

        // Use the sort from the std library as a baseline
        // We shouldn't expect to out perform this one
        do_sort_bench(&mut group, size, &items, "std", |xs: &mut [i32]| {
            xs.sort();
        });

        do_sort_bench(&mut group, size, &items, "selection", |xs: &mut [i32]| {
            algos::selection_sort(xs);
        });
    }

    group.finish();
}

criterion_group!(benches, sorting_i32s);
criterion_main!(benches);
