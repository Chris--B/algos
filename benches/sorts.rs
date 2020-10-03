use std::time::Instant;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main};

/// Items in sorted order
fn make_sorted_items(size: usize) -> Vec<i32> {
    let size = size as i32;
    (1..size).collect()
}

/// Items in reverse sorted order
fn make_reverse_sorted_items(size: usize) -> Vec<i32> {
    let size = size as i32;
    (1..size).rev().collect()
}

/// Items in random order
fn make_random_items(size: usize) -> Vec<i32> {
    use rand::rngs::SmallRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;

    // Make sure we have the same items, we'll just shuffle the order
    let mut all_items = make_sorted_items(size);

    // We don't want this seed to change between runs, so it must never change between
    // runs in the same process.
    // It could change between independent runs, but currently does not.
    let seed = u64::from_be_bytes(*b" #yolo !");

    // Shuffle all of the items - this will produce the same sorted result as other runs.
    let mut rng = SmallRng::seed_from_u64(seed);
    all_items.shuffle(&mut rng);

    all_items
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

fn sorting_with(c: &mut Criterion, name: &str, mut make_items: impl FnMut(usize) -> Vec<i32>) {
    let mut group = c.benchmark_group(name);

    let items_set: Vec<(usize, Vec<i32>)> = [
        // 0, 1, 2, 3, 4, 5, 10, 100, 1_000, 2_000,
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

        do_sort_bench(&mut group, size, &items, "insertion", |xs: &mut [i32]| {
            algos::insertion_sort(xs);
        });

        do_sort_bench(&mut group, size, &items, "merge", |xs: &mut [i32]| {
            algos::merge_sort(xs);
        });
    }

    group.finish();
}

fn sorting_random_i32s(c: &mut Criterion) {
    sorting_with(c, "n-random-items", make_random_items);
}

fn sorting_sorted_i32s(c: &mut Criterion) {
    sorting_with(c, "n-already-sorted", make_sorted_items);
}

fn sorting_reverse_sorted_i32s(c: &mut Criterion) {
    sorting_with(c, "n-reverse-sorted-items", make_reverse_sorted_items);
}

criterion_group!(
    benches,
    sorting_random_i32s,
    sorting_sorted_i32s,
    sorting_reverse_sorted_i32s
);

criterion_main!(benches);
