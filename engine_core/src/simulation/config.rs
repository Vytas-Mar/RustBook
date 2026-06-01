use crate::Side;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SimConfig {
    pub seed: u64,
    pub mid_price: u64,
    pub price_spread: u64,
    pub min_qty: u64,
    pub max_qty: u64,
    pub market_order_prob: f64,
    pub lambda_per_sec: f64,
}

#[derive(Debug, Clone)]
pub enum SimOrderKind {
    Limit { price: u64 },
    Market,
}
#[derive(Debug, Clone)]
pub struct SimOrder {
    pub side: Side,
    pub kind: SimOrderKind,
    pub qty: u64,
}
#[derive(Debug, Clone)]
pub struct SimEvent {
    pub dt_nanos: u64,
    pub order: SimOrder,
}
