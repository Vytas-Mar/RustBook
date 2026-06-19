use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}
