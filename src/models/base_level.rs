use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BaseLevel {
    #[serde(rename = "maxPopulation")]
    pub max_population: u32, // number of sustainable bits
    #[serde(rename = "upgradeCost")]
    pub upgrade_cost: u32, // bits required to unlock this upgrade
    #[serde(rename = "spawnRate")]
    pub spawn_rate: u32, // number uf bits spawned per tick
}

