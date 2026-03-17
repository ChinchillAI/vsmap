use serde::{Deserialize, Serialize};

use crate::{features::Resource, measurements::Position};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mine {
    pub name: Option<String>,
    pub resources: Vec<Resource>,
    pub pos: Position,
}
