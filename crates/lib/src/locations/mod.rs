mod mine;
mod ruin;
mod trader;
mod translocator;

pub use crate::locations::{
    mine::Mine,
    ruin::Ruin,
    trader::Trader,
    translocator::{Translocator, TranslocatorInToml, TranslocatorSide},
};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::measurements::{Relative, Vector};

macro_rules! define_location {
    (
        $( $location:ident,)+
    ) => {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub enum Location {
            $( $location($location), )+
        }

        impl Location {
            pub fn set_absolute(&mut self, p: Vector) {
                match self {
                    $(Self::$location(l) => {
                        l.pos.absolute = Some(p);
                    },)+
                }
            }

            pub fn get_absolute(&self) -> Option<Vector> {
                match self {
                    $(Self::$location(l) => l.pos.absolute,)+
                }
            }

            pub fn get_pos(&self) -> HashMap<String, Relative> {
                match self {
                    $(Self::$location(l) => l.pos.relative.clone(),)+
                }
            }
        }
    }
}

define_location! {
    Ruin, Trader, Translocator, Mine,
}
