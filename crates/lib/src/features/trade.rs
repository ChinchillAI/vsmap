use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Trade {
    Agriculture,
    Artisan,
    BuildingMaterials,
    Clothing,
    Commodities,
    Furniture,
    Luxuries,
    SurvivalGoods,
    TreasureHunter,
}
