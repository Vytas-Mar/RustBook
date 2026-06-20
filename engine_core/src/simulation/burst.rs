use super::config::SimOrderKind;
use super::generator::Simulator;
use crate::MatchingEngine;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct BurstMetrics {
    pub orders_placed: u64,
    pub trades_executed: u64, // delta during the burst not cumulative
}

pub fn run_burst(engine: &mut MatchingEngine, sim: &mut Simulator, n: u64) -> BurstMetrics {
    let trades_before = engine.trades.len() as u64;

    for _ in 0..n {
        let sim_event = sim.next();
        let qty = sim_event.order.qty;
        let side = sim_event.order.side;

        match sim_event.order.kind {
            SimOrderKind::Limit { price } => {
                engine.place_limit_order(price, qty, side).unwrap();
            }
            SimOrderKind::Market => {
                engine.place_market_order(qty, side).unwrap();
            }
        }
    }

    let trades_after = engine.trades.len() as u64;

    BurstMetrics {
        orders_placed: n,
        trades_executed: trades_after - trades_before,
    }
}
