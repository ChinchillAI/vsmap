use std::ops::{Add, Div, Mul, Sub};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Vector {
    pub x: i32,
    pub z: i32,
}

impl Vector {
    pub fn magnitude(&self) -> f32 {
        ((self.x.pow(2) + self.z.pow(2)) as f32).sqrt()
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            z: self.z * other.z,
        }
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            z: self.z / other.z,
        }
    }
}

impl From<[i32; 2]> for Vector {
    fn from(arr: [i32; 2]) -> Self {
        Self {
            x: arr[0],
            z: arr[1],
        }
    }
}
