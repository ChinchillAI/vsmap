mod distance;
pub use crate::measurements::distance::Distance;

mod gradient;
pub use crate::measurements::gradient::Gradient;

mod relative;
pub use crate::measurements::relative::Relative;

use serde::{Deserialize, Serialize};
use glam::IVec2;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Position {
    pub absolute: Option<IVec2>,
    #[serde(flatten)]
    pub relative: HashMap<String, Relative>,
}
