mod burst;
mod config;
mod generator;

pub use burst::{BurstMetrics, run_burst};
pub use config::{SimConfig, SimEvent, SimOrder, SimOrderKind};
pub use generator::Simulator;
