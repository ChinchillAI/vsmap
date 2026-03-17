use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TraderSeal {
    None,
    Gate,
    Door,
}
