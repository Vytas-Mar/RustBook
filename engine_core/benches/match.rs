use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use engine_core::{MatchingEngine, Orderbook, Side};

fn bench_market_sweep(c: &mut Criterion, n: usize) {
    c.bench_function(&format!("market_sweep_{n}_levels"), |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new(Orderbook::new());
                // Resting asks at prices 100..(100+n), one order per price, qty 1 each
                for i in 0..n {
                    engine.place_limit_order(100 + i as u64, 1, Side::Sell);
                }
                engine
            },
            |mut engine| {
                // Market buy with qty = n sweeps every level exactly
                engine.place_market_order(black_box(n as u64), Side::Buy);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_market_sweep_1(c: &mut Criterion) {
    bench_market_sweep(c, 1);
}
fn bench_market_sweep_10(c: &mut Criterion) {
    bench_market_sweep(c, 10);
}
fn bench_market_sweep_100(c: &mut Criterion) {
    bench_market_sweep(c, 100);
}

criterion_group!(
    benches,
    bench_market_sweep_1,
    bench_market_sweep_10,
    bench_market_sweep_100,
);
criterion_main!(benches);
