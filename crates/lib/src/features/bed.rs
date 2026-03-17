use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Bed {
    None,
    Hay,
    Wooden,
    Aged,
}
