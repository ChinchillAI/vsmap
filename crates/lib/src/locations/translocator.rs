use serde::{Deserialize, Serialize};

use crate::measurements::Position;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TranslocatorSide {
    Enter,
    Exit,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslocatorInToml {
    pub name: Option<String>,
    pub enter: Position,
    pub exit: Position,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Translocator {
    pub name: String,
    pub pos: Position,
    pub other_id: String,
    pub side: TranslocatorSide,
}
