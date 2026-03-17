use serde::{Deserialize, Serialize};

use crate::measurements::{Distance, Gradient, Vector};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(untagged)]
pub enum Relative {
    Vector(Vector),
    Gradient(Gradient),
    Distance(Distance),
}
