mod renderer;
mod solver;

pub use crate::map::renderer::{MapRenderer, render};
pub use crate::map::solver::solve;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::locations::{
    Location, Mine, Ruin, Trader, Translocator, TranslocatorInToml, TranslocatorSide,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawMap {
    #[serde(default)]
    pub ruin: Vec<Ruin>,
    #[serde(default)]
    pub trader: Vec<Trader>,
    #[serde(default)]
    pub translocator: Vec<TranslocatorInToml>,
    #[serde(default)]
    pub mine: Vec<Mine>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "RawMap")]
pub struct Map {
    pub locations: HashMap<String, Location>,
}

impl From<RawMap> for Map {
    fn from(raw: RawMap) -> Self {
        let mut locations = HashMap::<String, Location>::new();

        for (i, ruin) in raw.ruin.into_iter().enumerate() {
            let id = match ruin.name {
                Some(ref name) => {
                    let default_id = name.to_lowercase();
                    if locations.contains_key(&default_id) {
                        format!("{}-{}", default_id, i)
                    } else {
                        format!("{}", default_id)
                    }
                }
                None => {
                    format!("ruin-{i}")
                }
            };

            locations.insert(id, Location::Ruin(ruin));
        }

        for (i, trader) in raw.trader.into_iter().enumerate() {
            let default_id = trader.name.to_lowercase();
            let id = if locations.contains_key(&default_id) {
                format!("{}-{}", default_id, i)
            } else {
                format!("{}", default_id)
            };

            locations.insert(id, Location::Trader(trader));
        }

        for (i, translocator) in raw.translocator.into_iter().enumerate() {
            let id = match translocator.name {
                Some(ref name) => {
                    let default_id = name.to_lowercase();
                    if locations.contains_key(&default_id) {
                        format!("{}-{}", default_id, i)
                    } else {
                        format!("{}", default_id)
                    }
                }
                None => {
                    format!("translocator-{i}")
                }
            };

            let enter_id = format!("{id}-enter");
            let exit_id = format!("{id}-exit");

            let enter = Location::Translocator(Translocator {
                name: enter_id.to_owned(),
                pos: translocator.enter,
                side: TranslocatorSide::Enter,
                other_id: exit_id.to_owned(),
            });

            let exit = Location::Translocator(Translocator {
                name: exit_id.to_owned(),
                pos: translocator.exit,
                side: TranslocatorSide::Exit,
                other_id: enter_id.to_owned(),
            });

            locations.insert(enter_id, enter);
            locations.insert(exit_id, exit);
        }

        Self {
            locations: solve(locations),
        }
    }
}
