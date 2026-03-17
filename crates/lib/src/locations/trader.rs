use serde::{Deserialize, Serialize};

use crate::{
    features::{Bed, Direction, Trade, TraderSeal},
    measurements::Position,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trader {
    pub name: String,
    pub facing: Direction,
    pub trade: Trade,
    pub seal: TraderSeal,
    pub bed: Bed,
    #[serde(default)]
    pub pos: Position,
}
