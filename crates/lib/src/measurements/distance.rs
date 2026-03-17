use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Distance {
    pub center: i32,
}

impl From<[i32; 1]> for Distance {
    fn from(arr: [i32; 1]) -> Self {
        Self { center: arr[0] }
    }
}
