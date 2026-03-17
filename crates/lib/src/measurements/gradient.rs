use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Gradient {
    pub center: i32,
    pub north: i32,
    pub south: i32,
    pub east: i32,
    pub west: i32,
    pub step: i32,
}

impl From<[i32; 6]> for Gradient {
    fn from(arr: [i32; 6]) -> Self {
        Self {
            center: arr[0],
            north: arr[1],
            south: arr[2],
            east: arr[3],
            west: arr[4],
            step: arr[5],
        }
    }
}
