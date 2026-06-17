use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use engine_core::simulation::{SimConfig, Simulator, run_burst};
use engine_core::{MatchingEngine, Orderbook};

fn make_config(lambda: f64) -> SimConfig {
    SimConfig {
        seed: 42,
        mid_price: 10_000,
        price_spread: 50,
        min_qty: 1,
        max_qty: 100,
        market_order_prob: 0.1,
        lambda_per_sec: lambda,
    }
}

fn bench_mixed_burst(c: &mut Criterion, n: u64, lambda: f64) {
    let config = make_config(lambda);
    c.bench_function(&format!("mixed_burst_n{n}_lambda{lambda}"), |b| {
        b.iter_batched(
            || {
                (
                    MatchingEngine::new(Orderbook::new()),
                    Simulator::new(config.clone()),
                )
            },
            |(mut engine, mut sim)| {
                run_burst(&mut engine, &mut sim, black_box(n));
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_burst_1k_lambda_1k(c: &mut Criterion) {
    bench_mixed_burst(c, 1_000, 1_000.0);
}
fn bench_burst_10k_lambda_1k(c: &mut Criterion) {
    bench_mixed_burst(c, 10_000, 1_000.0);
}
fn bench_burst_10k_lambda_100k(c: &mut Criterion) {
    bench_mixed_burst(c, 10_000, 100_000.0);
}

criterion_group!(
    benches,
    bench_burst_1k_lambda_1k,
    bench_burst_10k_lambda_1k,
    bench_burst_10k_lambda_100k,
);
criterion_main!(benches);
