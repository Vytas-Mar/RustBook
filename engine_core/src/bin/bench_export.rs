use engine_core::simulation::{SimConfig, SimOrderKind, Simulator};
use engine_core::{MatchingEngine, Orderbook, Side};
use hdrhistogram::Histogram;
use serde::Serialize;
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct BenchSuite {
    commit: String,
    generated_at_unix_secs: u64,
    benches: Vec<BenchResult>,
}

#[derive(Serialize)]
struct BenchResult {
    name: String,
    samples: u64,
    mean_ns: u64,
    p50_ns: u64,
    p99_ns: u64,
    p999_ns: u64,
    p9999_ns: u64,
    max_ns: u64,
}

fn measure_burst(n: u64, lambda: f64) -> Histogram<u64> {
    let config = SimConfig {
        seed: 42,
        mid_price: 10_000,
        price_spread: 50,
        min_qty: 1,
        max_qty: 100,
        market_order_prob: 0.1,
        lambda_per_sec: lambda,
    };

    let mut sim = Simulator::new(config);
    let events: Vec<_> = (0..n).map(|_| sim.next()).collect();

    let mut engine = MatchingEngine::new(Orderbook::new());
    let mut hist = Histogram::<u64>::new_with_bounds(1, 10_000_000, 3).unwrap();

    for ev in &events {
        let start = Instant::now();
        match ev.order.kind {
            SimOrderKind::Limit { price } => {
                engine.place_limit_order(price, ev.order.qty, ev.order.side);
            }
            SimOrderKind::Market => {
                engine.place_market_order(ev.order.qty, ev.order.side);
            }
        }
        let elapsed_ns = start.elapsed().as_nanos() as u64;
        hist.record(elapsed_ns.max(1)).unwrap();
    }

    hist
}

fn measure_insert_hot_level(n: u64) -> Histogram<u64> {
    let mut engine = MatchingEngine::new(Orderbook::new());
    // Pre-populate one hot level
    for _ in 0..100 {
        engine.place_limit_order(100, 1, Side::Buy);
    }

    let mut hist = Histogram::<u64>::new_with_bounds(1, 10_000_000, 3).unwrap();
    for _ in 0..n {
        let start = Instant::now();
        engine.place_limit_order(100, 1, Side::Buy);
        let elapsed_ns = start.elapsed().as_nanos() as u64;
        hist.record(elapsed_ns.max(1)).unwrap();
    }
    hist
}

fn measure_market_sweep(n_levels: usize, repetitions: u64) -> Histogram<u64> {
    let mut hist = Histogram::<u64>::new_with_bounds(1, 100_000_000, 3).unwrap();
    for _ in 0..repetitions {
        let mut engine = MatchingEngine::new(Orderbook::new());
        for i in 0..n_levels {
            engine.place_limit_order(100 + i as u64, 1, Side::Sell);
        }
        let start = Instant::now();
        engine.place_market_order(n_levels as u64, Side::Buy);
        let elapsed_ns = start.elapsed().as_nanos() as u64;
        hist.record(elapsed_ns.max(1)).unwrap();
    }
    hist
}

fn from_hist(name: &str, hist: &Histogram<u64>) -> BenchResult {
    BenchResult {
        name: name.to_string(),
        samples: hist.len(),
        mean_ns: hist.mean().round() as u64,
        p50_ns: hist.value_at_quantile(0.50),
        p99_ns: hist.value_at_quantile(0.99),
        p999_ns: hist.value_at_quantile(0.999),
        p9999_ns: hist.value_at_quantile(0.9999),
        max_ns: hist.max(),
    }
}

fn print_report(result: &BenchResult) {
    println!("\n=== {} ===", result.name);
    println!("  samples: {}", result.samples);
    println!("  mean:    {} ns", result.mean_ns);
    println!("  p50:     {} ns", result.p50_ns);
    println!("  p99:     {} ns", result.p99_ns);
    println!("  p99.9:   {} ns", result.p999_ns);
    println!("  p99.99:  {} ns", result.p9999_ns);
    println!("  max:     {} ns", result.max_ns);
}

fn git_short_hash() -> String {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

fn main() {
    let benches = vec![
        from_hist(
            "insert_hot_level",
            &measure_insert_hot_level(100_000),
        ),
        from_hist(
            "market_sweep_10_levels",
            &measure_market_sweep(10, 10_000),
        ),
        from_hist(
            "burst_n100k_lambda_1k",
            &measure_burst(100_000, 1_000.0),
        ),
        from_hist(
            "burst_n100k_lambda_100k",
            &measure_burst(100_000, 100_000.0),
        ),
    ];

    for b in &benches {
        print_report(b);
    }

    let suite = BenchSuite {
        commit: git_short_hash(),
        generated_at_unix_secs: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
        benches,
    };

    let path = "ui/orderbook-ui/public/bench-results.json";
    let json = serde_json::to_string_pretty(&suite).expect("failed to serialize");
    std::fs::write(path, json).unwrap_or_else(|e| {
        eprintln!("\n⚠️  Failed to write {path}: {e}");
        eprintln!("   (Run from the workspace root.)");
        std::process::exit(1);
    });

    println!("\n✓ Wrote {}", path);
}
