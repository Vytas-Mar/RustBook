use criterion::BatchSize::SmallInput;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use engine_core::{MatchingEngine, Orderbook, Side};

fn bench_insert_into_empty_book(c: &mut Criterion) {
    c.bench_function("insert_into_empty_book", |b| {
        b.iter_batched(
            || MatchingEngine::new(Orderbook::new()),
            |mut engine| {
                engine
                    .place_limit_order(black_box(100), black_box(10), Side::Buy)
                    .unwrap();
                engine
            },
            SmallInput,
        );
    });
}

fn bench_insert_no_cross_hot_level(c: &mut Criterion) {
    c.bench_function("insert_no_cross_hot_level", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new(Orderbook::new());
                // pre-populate 100 resting buy orders at price 100
                for _ in 0..100 {
                    engine.place_limit_order(100, 1, Side::Buy).unwrap();
                }

                engine
            },
            |mut engine| {
                // measure: insert #101 at the same price (hot level — no new BTreeMap entry)
                engine
                    .place_limit_order(black_box(100), black_box(10), Side::Buy)
                    .unwrap();
                engine
            },
            SmallInput,
        );
    });
}

fn bench_insert_cold_level(c: &mut Criterion) {
    c.bench_function("insert_cold_level", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new(Orderbook::new());
                // pre-populate: 100 orders at 100 distinct prices (each a fresh level)
                for i in 0..100 {
                    engine
                        .place_limit_order(100 + i as u64, 1, Side::Buy)
                        .unwrap();
                }
                engine
            },
            |mut engine| {
                // measure: insert at a price we haven't seen (new BTreeMap entry + new VecDeque)
                engine
                    .place_limit_order(black_box(99), black_box(1), Side::Buy)
                    .unwrap();
                engine
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_insert_into_empty_book,
    bench_insert_no_cross_hot_level,
    bench_insert_cold_level
);
criterion_main!(benches);
