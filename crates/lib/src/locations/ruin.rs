use serde::{Deserialize, Serialize};

use crate::measurements::Position;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ruin {
    pub name: Option<String>,
    #[serde(default)]
    pub pos: Position,
}
