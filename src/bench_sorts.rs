    use test::{black_box, Bencher};

    use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mycrate::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

    fn make_items<'a>() -> impl Iterator<Item = i32> {
        (1..1_000).rev()
    }

    #[bench]
    fn bench_noop(b: &mut Bencher) {
        let items: Vec<i32> = make_items().collect();

        b.iter(|| {
            let items: Vec<_> = items.clone();

            // Do nothing - this is the base line bench

            black_box(items)
        });
    }

    #[bench]
    fn bench_std(b: &mut Bencher) {
        let items: Vec<i32> = make_items().collect();

        b.iter(|| {
            let mut items: Vec<_> = items.clone();

            items.sort();

            black_box(items)
        });
    }
}
