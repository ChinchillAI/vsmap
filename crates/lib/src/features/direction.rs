use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
