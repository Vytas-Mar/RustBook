mod common;

use common::assert_book_invariants;
use engine_core::simulation::{SimConfig, Simulator, run_burst};
use engine_core::{MatchingEngine, Orderbook};

fn default_config(seed: u64) -> SimConfig {
    SimConfig {
        seed,
        mid_price: 100,
        price_spread: 10,
        min_qty: 1,
        max_qty: 100,
        market_order_prob: 0.1,
        lambda_per_sec: 1000.0,
    }
}

#[test]
fn burst_with_same_seed_produces_identical_engine_state() {
    let mut e1 = MatchingEngine::new(Orderbook::new());
    let mut e2 = MatchingEngine::new(Orderbook::new());
    let mut s1 = Simulator::new(default_config(42));
    let mut s2 = Simulator::new(default_config(42));

    let m1 = run_burst(&mut e1, &mut s1, 1000);
    let m2 = run_burst(&mut e2, &mut s2, 1000);

    assert_eq!(m1.orders_placed, m2.orders_placed);
    assert_eq!(m1.trades_executed, m2.trades_executed);
    assert_eq!(e1.trades.len(), e2.trades.len());

    for (t1, t2) in e1.trades.iter().zip(e2.trades.iter()) {
        assert_eq!(t1.maker_id, t2.maker_id);
        assert_eq!(t1.taker_id, t2.taker_id);
        assert_eq!(t1.taker_side, t2.taker_side);
        assert_eq!(t1.price, t2.price);
        assert_eq!(t1.qty, t2.qty);
    }

    assert_eq!(
        e1.book.bids.len(),
        e2.book.bids.len(),
        "bid level counts diverged"
    );
    assert_eq!(
        e1.book.asks.len(),
        e2.book.asks.len(),
        "ask level counts diverged"
    );

    for (a, b) in e1.book.bids.iter().zip(e2.book.bids.iter()) {
        assert_eq!(a.0, b.0, "bid price mismatch");
        assert_eq!(a.1.total_qty, b.1.total_qty, "bid total_qty mismatch");
    }

    for (a, b) in e1.book.asks.iter().zip(e2.book.asks.iter()) {
        assert_eq!(a.0, b.0, "ask price mismatch");
        assert_eq!(a.1.total_qty, b.1.total_qty, "ask total_qty mismatch");
    }
}

#[test]
fn burst_preserves_invariants() {
    let mut engine = MatchingEngine::new(Orderbook::new());
    let mut sim = Simulator::new(default_config(99));

    run_burst(&mut engine, &mut sim, 5000);

    assert_book_invariants(&engine.book);
}

#[test]
fn all_market_on_empty_book_produces_no_trades_and_no_resting_orders() {
    let mut cfg = default_config(123);
    cfg.market_order_prob = 1.0;

    let mut engine = MatchingEngine::new(Orderbook::new());
    let mut sim = Simulator::new(cfg);

    let metrics = run_burst(&mut engine, &mut sim, 500);

    assert_eq!(metrics.orders_placed, 500);
    assert_eq!(metrics.trades_executed, 0);
    assert!(engine.book.bids.is_empty());
    assert!(engine.book.asks.is_empty());
    assert!(engine.trades.is_empty());
}
