use serde::{Deserialize, Serialize};
use glam::IVec2;

use crate::measurements::{Distance, Gradient};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(untagged)]
pub enum Relative {
    Vector(IVec2),
    Gradient(Gradient),
    Distance(Distance),
}
