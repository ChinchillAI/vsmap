mod distance;
pub use crate::measurements::distance::Distance;

mod gradient;
pub use crate::measurements::gradient::Gradient;

mod vector;
pub use crate::measurements::vector::Vector;

mod relative;
pub use crate::measurements::relative::Relative;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Position {
    pub absolute: Option<Vector>,
    #[serde(flatten)]
    pub relative: HashMap<String, Relative>,
}
