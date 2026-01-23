use multi_index_map::MultiIndexMap;
use serde::{Deserialize, Serialize};

use crate::itemdata::{DropChance, LevelStats, PatchLog, Rarity};

#[derive(Debug, MultiIndexMap, Serialize, Deserialize)]
#[multi_index_derive(Debug)]
#[multi_index_hash(rustc_hash::FxBuildHasher)]
pub struct Arcane {
    #[multi_index(hashed_non_unique)]
    #[serde(rename = "name")]
    pub name: String,

    #[multi_index(hashed_unique)]
    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[multi_index(hashed_non_unique)]
    #[serde(rename = "type")]
    pub type_: String,

    pub rarity: Option<Rarity>,
    pub drops: Option<Vec<DropChance>>,

    #[serde(rename = "imageName")]
    pub image_name: String,

    #[serde(rename = "levelStats")]
    pub level_stats: Option<Vec<LevelStats>>,

    #[serde(rename = "patchlogs")]
    pub patch_log: Option<Vec<PatchLog>>,

    #[serde(rename = "tradable")]
    pub tradable: bool,

    #[serde(rename = "masterable")]
    pub masterable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_arcane() {
        let json_data = r#"
{
  "category": "Arcanes",
  "drops": [
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Consequence"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Ice"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Momentum"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Nullifier"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Tempo"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Warmth"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Deflection"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Healing"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Resistance"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Victory"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Acceleration"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Awakening"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Eruption"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Phantasm"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Strike"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Precision"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Pulse"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Ultimatum"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Trickery"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Velocity"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Arachne"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Avenger"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Fury"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Rage"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Consequence"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Ice"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Momentum"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Nullifier"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Tempo"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Warmth"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Deflection"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Healing"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Resistance"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Victory"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Acceleration"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Awakening"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Eruption"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Phantasm"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Strike"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Precision"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Pulse"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Ultimatum"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Trickery"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Velocity"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Arachne"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Avenger"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Fury"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Rage"
    },
    {
      "chance": 0.0141,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Grace"
    },
    {
      "chance": 0.0141,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Energize"
    },
    {
      "chance": 0.0141,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Barrier"
    },
    {
      "chance": 0.0278,
      "location": "Duviri Static Undercroft Portal (Steel Path)",
      "rarity": "Rare",
      "type": "Arcane Reaper"
    },
    {
      "chance": 0.0282,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Aegis"
    },
    {
      "chance": 0.0282,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Precision"
    },
    {
      "chance": 0.0282,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Pulse"
    },
    {
      "chance": 0.0282,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Ultimatum"
    },
    {
      "chance": 0.0282,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Rage"
    },
    {
      "chance": 0.0282,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Fury"
    },
    {
      "chance": 0.0282,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Avenger"
    },
    {
      "chance": 0.0282,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Arachne"
    },
    {
      "chance": 0.0333,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Avenger"
    },
    {
      "chance": 0.0333,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Arachne"
    },
    {
      "chance": 0.0345,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Ultimatum"
    },
    {
      "chance": 0.0345,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Rage"
    },
    {
      "chance": 0.0345,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Fury"
    },
    {
      "chance": 0.0345,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Aegis"
    },
    {
      "chance": 0.0345,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Precision"
    },
    {
      "chance": 0.0345,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Pulse"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Consequence"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Ice"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Momentum"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Nullifier"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Tempo"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Warmth"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Deflection"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Healing"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Resistance"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Victory"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Acceleration"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Awakening"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Eruption"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Phantasm"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Strike"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Precision"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Pulse"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Ultimatum"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Trickery"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Velocity"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Arachne"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Avenger"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Fury"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Rage"
    },
    {
      "chance": 0.0443,
      "location": "Duviri/Endless: Tier 1 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Intention"
    },
    {
      "chance": 0.0443,
      "location": "Duviri/Endless: Tier 1 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Power Ramp"
    },
    {
      "chance": 0.0443,
      "location": "Duviri/Endless: Tier 3 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Intention"
    },
    {
      "chance": 0.0443,
      "location": "Duviri/Endless: Tier 3 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Power Ramp"
    },
    {
      "chance": 0.0443,
      "location": "Duviri/Endless: Tier 4 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Intention"
    },
    {
      "chance": 0.0443,
      "location": "Duviri/Endless: Tier 4 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Power Ramp"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation A",
      "rarity": "Rare",
      "type": "Arcane Pistoleer"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation A",
      "rarity": "Rare",
      "type": "Arcane Tanker"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation A",
      "rarity": "Rare",
      "type": "Arcane Bodyguard"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation B",
      "rarity": "Rare",
      "type": "Arcane Bodyguard"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation B",
      "rarity": "Rare",
      "type": "Arcane Blade Charger"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation B",
      "rarity": "Rare",
      "type": "Arcane Primary Charger"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation C",
      "rarity": "Rare",
      "type": "Arcane Pistoleer"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation C",
      "rarity": "Rare",
      "type": "Arcane Tanker"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation C",
      "rarity": "Rare",
      "type": "Arcane Blade Charger"
    },
    {
      "chance": 0.05,
      "location": "Arbitrations, Rotation C",
      "rarity": "Rare",
      "type": "Arcane Primary Charger"
    },
    {
      "chance": 0.05,
      "location": "Eidolon Hydrolyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Grace"
    },
    {
      "chance": 0.05,
      "location": "Eidolon Hydrolyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Energize"
    },
    {
      "chance": 0.05,
      "location": "Eidolon Hydrolyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Barrier"
    },
    {
      "chance": 0.0529,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Ultimatum"
    },
    {
      "chance": 0.0529,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Pulse"
    },
    {
      "chance": 0.0529,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Aegis"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Resistance"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Healing"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Deflection"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Victory"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Strike"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Awakening"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Guardian"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Phantasm"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Eruption"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Acceleration"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Trickery"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Velocity"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.057,
      "location": "Mars/Tyana Pass (Defense), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Double Back"
    },
    {
      "chance": 0.057,
      "location": "Mars/Tyana Pass (Defense), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Steadfast"
    },
    {
      "chance": 0.0592,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Precision"
    },
    {
      "chance": 0.06,
      "location": "Duviri/Endless: Tier 6 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Reaper"
    },
    {
      "chance": 0.06,
      "location": "Duviri/Endless: Tier 6 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Power Ramp"
    },
    {
      "chance": 0.06,
      "location": "Duviri/Endless: Tier 7 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Reaper"
    },
    {
      "chance": 0.06,
      "location": "Duviri/Endless: Tier 7 (Hard)",
      "rarity": "Rare",
      "type": "Arcane Power Ramp"
    },
    {
      "chance": 0.0602,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Resistance"
    },
    {
      "chance": 0.0602,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Healing"
    },
    {
      "chance": 0.0602,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Deflection"
    },
    {
      "chance": 0.0602,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Victory"
    },
    {
      "chance": 0.066,
      "location": "Sister Of Parvos (Ascension Mode)",
      "rarity": "Uncommon",
      "type": "Arcane Ice Storm"
    },
    {
      "chance": 0.066,
      "location": "Sister Of Parvos (Ascension Mode)",
      "rarity": "Uncommon",
      "type": "Arcane Battery"
    },
    {
      "chance": 0.0667,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Deflection"
    },
    {
      "chance": 0.0667,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Phantasm"
    },
    {
      "chance": 0.0667,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Eruption"
    },
    {
      "chance": 0.0667,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Velocity"
    },
    {
      "chance": 0.0667,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.0678,
      "location": "Eidolon Teralyst",
      "rarity": "Rare",
      "type": "Arcane Resistance"
    },
    {
      "chance": 0.069,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Healing"
    },
    {
      "chance": 0.069,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Awakening"
    },
    {
      "chance": 0.069,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Guardian"
    },
    {
      "chance": 0.069,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Trickery"
    },
    {
      "chance": 0.069,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Resistance"
    },
    {
      "chance": 0.069,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Victory"
    },
    {
      "chance": 0.069,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Strike"
    },
    {
      "chance": 0.069,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Acceleration"
    },
    {
      "chance": 0.0702,
      "location": "Eidolon Gantulyst",
      "rarity": "Rare",
      "type": "Arcane Precision"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Ice"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Nullifier"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Warmth"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Deflection"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Healing"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Resistance"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Eruption"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Phantasm"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Pulse"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Ultimatum"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Trickery"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Consequence"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Arachne"
    },
    {
      "chance": 0.0794,
      "location": "Lua/Circulus (Survival), Rotation B",
      "rarity": "Rare",
      "type": "Arcane Rise"
    },
    {
      "chance": 0.0851,
      "location": "Temporal Archimedea Gold Rewards",
      "rarity": "Rare",
      "type": "Arcane Escapist"
    },
    {
      "chance": 0.0851,
      "location": "Temporal Archimedea Gold Rewards",
      "rarity": "Rare",
      "type": "Arcane Hot Shot"
    },
    {
      "chance": 0.0851,
      "location": "Temporal Archimedea Gold Rewards",
      "rarity": "Rare",
      "type": "Arcane Universal Fallout"
    },
    {
      "chance": 0.0876,
      "location": "Temporal Archimedea Silver Rewards",
      "rarity": "Rare",
      "type": "Arcane Escapist"
    },
    {
      "chance": 0.0876,
      "location": "Temporal Archimedea Silver Rewards",
      "rarity": "Rare",
      "type": "Arcane Hot Shot"
    },
    {
      "chance": 0.0876,
      "location": "Temporal Archimedea Silver Rewards",
      "rarity": "Rare",
      "type": "Arcane Universal Fallout"
    },
    {
      "chance": 0.0909,
      "location": "Duviri/Endless: Repeated Rewards (Hard)",
      "rarity": "Rare",
      "type": "Arcane Power Ramp"
    },
    {
      "chance": 0.0909,
      "location": "Duviri/Endless: Repeated Rewards (Hard)",
      "rarity": "Rare",
      "type": "Arcane Intention"
    },
    {
      "chance": 0.0925,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Acceleration"
    },
    {
      "chance": 0.0925,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Eruption"
    },
    {
      "chance": 0.0925,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Phantasm"
    },
    {
      "chance": 0.0925,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.1,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Tempo"
    },
    {
      "chance": 0.1,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Consequence"
    },
    {
      "chance": 0.1,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Momentum"
    },
    {
      "chance": 0.1,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Ice"
    },
    {
      "chance": 0.1,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Nullifier"
    },
    {
      "chance": 0.1,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Warmth"
    },
    {
      "chance": 0.1019,
      "location": "Duviri Static Undercroft Portal (Steel Path)",
      "rarity": "Uncommon",
      "type": "Arcane Intention"
    },
    {
      "chance": 0.1019,
      "location": "Duviri Static Undercroft Portal (Steel Path)",
      "rarity": "Uncommon",
      "type": "Arcane Power Ramp"
    },
    {
      "chance": 0.102,
      "location": "Eidolon Gantulyst",
      "rarity": "Uncommon",
      "type": "Arcane Acceleration"
    },
    {
      "chance": 0.102,
      "location": "Eidolon Gantulyst",
      "rarity": "Uncommon",
      "type": "Arcane Eruption"
    },
    {
      "chance": 0.102,
      "location": "Eidolon Gantulyst",
      "rarity": "Uncommon",
      "type": "Arcane Phantasm"
    },
    {
      "chance": 0.102,
      "location": "Eidolon Gantulyst",
      "rarity": "Uncommon",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.102,
      "location": "Lua/Circulus (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Blessing"
    },
    {
      "chance": 0.1034,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Tempo"
    },
    {
      "chance": 0.1034,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Consequence"
    },
    {
      "chance": 0.1034,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Momentum"
    },
    {
      "chance": 0.1034,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Ice"
    },
    {
      "chance": 0.1034,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Nullifier"
    },
    {
      "chance": 0.1034,
      "location": "Neptune/Mammon's Prospect (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Warmth"
    },
    {
      "chance": 0.1034,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Tempo"
    },
    {
      "chance": 0.1034,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Consequence"
    },
    {
      "chance": 0.1034,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Momentum"
    },
    {
      "chance": 0.1034,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Ice"
    },
    {
      "chance": 0.1034,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Nullifier"
    },
    {
      "chance": 0.1034,
      "location": "Venus/Vesper Strait (Skirmish), Rotation C",
      "rarity": "Uncommon",
      "type": "Arcane Warmth"
    },
    {
      "chance": 0.1111,
      "location": "The Descendia: Infernum 6 and 13 Arcane Rewards (Steel Path)",
      "rarity": "Uncommon",
      "type": "Arcane Expertise"
    },
    {
      "chance": 0.1111,
      "location": "The Descendia: Infernum 6 and 13 Arcane Rewards (Steel Path)",
      "rarity": "Uncommon",
      "type": "Arcane Persistence"
    },
    {
      "chance": 0.1111,
      "location": "The Descendia: Infernum 6 and 13 Arcane Rewards (Steel Path)",
      "rarity": "Uncommon",
      "type": "Arcane Circumvent"
    },
    {
      "chance": 0.1111,
      "location": "The Descendia: Infernum 6 and 13 Arcane Rewards (Steel Path)",
      "rarity": "Uncommon",
      "type": "Arcane Concentration"
    },
    {
      "chance": 0.1124,
      "location": "Eidolon Hydrolyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Fury"
    },
    {
      "chance": 0.1124,
      "location": "Eidolon Hydrolyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Avenger"
    },
    {
      "chance": 0.117,
      "location": "Eidolon Hydrolyst",
      "rarity": "Uncommon",
      "type": "Arcane Avenger"
    },
    {
      "chance": 0.1192,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Guardian"
    },
    {
      "chance": 0.1205,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Warmth"
    },
    {
      "chance": 0.1205,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Momentum"
    },
    {
      "chance": 0.1205,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Nullifier"
    },
    {
      "chance": 0.1205,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Consequence"
    },
    {
      "chance": 0.1205,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Ice"
    },
    {
      "chance": 0.125,
      "location": "H-09 Apex",
      "rarity": "Uncommon",
      "type": "Arcane Camisado"
    },
    {
      "chance": 0.125,
      "location": "H-09 Apex",
      "rarity": "Uncommon",
      "type": "Arcane Bellicose"
    },
    {
      "chance": 0.125,
      "location": "H-09 Apex",
      "rarity": "Uncommon",
      "type": "Arcane Impetus"
    },
    {
      "chance": 0.125,
      "location": "H-09 Apex",
      "rarity": "Uncommon",
      "type": "Arcane Truculence"
    },
    {
      "chance": 0.125,
      "location": "H-09 Apex",
      "rarity": "Uncommon",
      "type": "Arcane Crepuscular"
    },
    {
      "chance": 0.125,
      "location": "H-09 Efervon Tank",
      "rarity": "Uncommon",
      "type": "Arcane Camisado"
    },
    {
      "chance": 0.125,
      "location": "H-09 Efervon Tank",
      "rarity": "Uncommon",
      "type": "Arcane Bellicose"
    },
    {
      "chance": 0.125,
      "location": "H-09 Efervon Tank",
      "rarity": "Uncommon",
      "type": "Arcane Impetus"
    },
    {
      "chance": 0.125,
      "location": "H-09 Efervon Tank",
      "rarity": "Uncommon",
      "type": "Arcane Truculence"
    },
    {
      "chance": 0.125,
      "location": "H-09 Efervon Tank",
      "rarity": "Uncommon",
      "type": "Arcane Crepuscular"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Antivirus Bounty (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Bellicose"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Antivirus Bounty (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Truculence"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Antivirus Bounty (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Impetus"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Antivirus Bounty (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Crepuscular"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Antivirus Bounty (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Camisado"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Scaldra (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Bellicose"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Scaldra (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Truculence"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Scaldra (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Impetus"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Scaldra (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Crepuscular"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Scaldra (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Camisado"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Techrot (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Bellicose"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Techrot (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Truculence"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Techrot (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Impetus"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Techrot (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Crepuscular"
    },
    {
      "chance": 0.125,
      "location": "Höllvania/Exterminate: Techrot (Caches)",
      "rarity": "Uncommon",
      "type": "Arcane Camisado"
    },
    {
      "chance": 0.125,
      "location": "Scaldra Screamer",
      "rarity": "Uncommon",
      "type": "Arcane Camisado"
    },
    {
      "chance": 0.125,
      "location": "Scaldra Screamer",
      "rarity": "Uncommon",
      "type": "Arcane Bellicose"
    },
    {
      "chance": 0.125,
      "location": "Scaldra Screamer",
      "rarity": "Uncommon",
      "type": "Arcane Impetus"
    },
    {
      "chance": 0.125,
      "location": "Scaldra Screamer",
      "rarity": "Uncommon",
      "type": "Arcane Truculence"
    },
    {
      "chance": 0.125,
      "location": "Scaldra Screamer",
      "rarity": "Uncommon",
      "type": "Arcane Crepuscular"
    },
    {
      "chance": 0.1274,
      "location": "Eidolon Hydrolyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Rage"
    },
    {
      "chance": 0.1274,
      "location": "Eidolon Hydrolyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Arachne"
    },
    {
      "chance": 0.1366,
      "location": "Temporal Archimedea Legendary Rewards",
      "rarity": "Uncommon",
      "type": "Arcane Escapist"
    },
    {
      "chance": 0.1366,
      "location": "Temporal Archimedea Legendary Rewards",
      "rarity": "Uncommon",
      "type": "Arcane Hot Shot"
    },
    {
      "chance": 0.1366,
      "location": "Temporal Archimedea Legendary Rewards",
      "rarity": "Uncommon",
      "type": "Arcane Universal Fallout"
    },
    {
      "chance": 0.1376,
      "location": "Eidolon Hydrolyst",
      "rarity": "Uncommon",
      "type": "Arcane Fury"
    },
    {
      "chance": 0.1376,
      "location": "Eidolon Hydrolyst",
      "rarity": "Uncommon",
      "type": "Arcane Rage"
    },
    {
      "chance": 0.1376,
      "location": "Eidolon Hydrolyst",
      "rarity": "Uncommon",
      "type": "Arcane Arachne"
    },
    {
      "chance": 0.1463,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Strike"
    },
    {
      "chance": 0.1463,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Awakening"
    },
    {
      "chance": 0.1495,
      "location": "Eidolon Teralyst",
      "rarity": "Uncommon",
      "type": "Arcane Warmth"
    },
    {
      "chance": 0.1495,
      "location": "Eidolon Teralyst",
      "rarity": "Uncommon",
      "type": "Arcane Momentum"
    },
    {
      "chance": 0.1495,
      "location": "Eidolon Teralyst",
      "rarity": "Uncommon",
      "type": "Arcane Nullifier"
    },
    {
      "chance": 0.1495,
      "location": "Eidolon Teralyst",
      "rarity": "Uncommon",
      "type": "Arcane Consequence"
    },
    {
      "chance": 0.1495,
      "location": "Eidolon Teralyst",
      "rarity": "Uncommon",
      "type": "Arcane Ice"
    },
    {
      "chance": 0.1568,
      "location": "Eidolon Gantulyst",
      "rarity": "Uncommon",
      "type": "Arcane Guardian"
    },
    {
      "chance": 0.1568,
      "location": "Eidolon Teralyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Tempo"
    },
    {
      "chance": 0.1679,
      "location": "Eidolon Hydrolyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Velocity"
    },
    {
      "chance": 0.1825,
      "location": "Eidolon Gantulyst",
      "rarity": "Uncommon",
      "type": "Arcane Strike"
    },
    {
      "chance": 0.1825,
      "location": "Eidolon Gantulyst",
      "rarity": "Uncommon",
      "type": "Arcane Awakening"
    },
    {
      "chance": 0.185,
      "location": "Eidolon Teralyst",
      "rarity": "Uncommon",
      "type": "Arcane Tempo"
    },
    {
      "chance": 0.2,
      "location": "Sister Of Parvos (Ascension Hard Mode)",
      "rarity": "Uncommon",
      "type": "Arcane Ice Storm"
    },
    {
      "chance": 0.2,
      "location": "Sister Of Parvos (Ascension Hard Mode)",
      "rarity": "Uncommon",
      "type": "Arcane Battery"
    },
    {
      "chance": 0.2024,
      "location": "Eidolon Hydrolyst (Capture)",
      "rarity": "Uncommon",
      "type": "Arcane Trickery"
    },
    {
      "chance": 0.2223,
      "location": "Eidolon Hydrolyst",
      "rarity": "Uncommon",
      "type": "Arcane Velocity"
    },
    {
      "chance": 0.2478,
      "location": "Eidolon Hydrolyst",
      "rarity": "Uncommon",
      "type": "Arcane Trickery"
    },
    {
      "chance": 0.3333,
      "location": "Duviri/Endless: Tier 2 (Hard)",
      "rarity": "Common",
      "type": "Arcane Reaper"
    },
    {
      "chance": 0.3333,
      "location": "Duviri/Endless: Tier 8 (Hard)",
      "rarity": "Common",
      "type": "Arcane Reaper"
    }
  ],
  "imageName": "arcane.png",
  "masterable": false,
  "name": "Arcane",
  "patchlogs": [
    {
      "name": "Lua’s Prey: Hotfix 32.2.4",
      "date": "2022-12-07T19:04:56Z",
      "url": "https://forums.warframe.com/topic/1333863-lua%E2%80%99s-prey-hotfix-3224/",
      "additions": "",
      "changes": "Added an “Arcanes” shortcut in the pause menu under “Equipment” while in Relays and Open Landscape towns. ",
      "fixes": ""
    },
    {
      "name": "Lua’s Prey: Hotfix 32.2.1",
      "date": "2022-11-30T22:33:07Z",
      "url": "https://forums.warframe.com/topic/1332403-lua%E2%80%99s-prey-hotfix-3221%C2%A0/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Clients visiting Host’s Orbiter getting pulled into the Arcane management screen when Host opens it.  \nFixed the “Arcanes” button in the pause menu using the “Mods” description. "
    },
    {
      "name": "Update 32.2.0: Lua’s Prey",
      "date": "2022-11-30T16:01:46Z",
      "url": "https://forums.warframe.com/topic/1332235-update-3220-lua%E2%80%99s-prey/",
      "additions": "",
      "changes": "Upon login, a waypoint will direct you to its new location between the Mods and Incubator Segments. You may need to do some redecorating as your existing decorations may be obstructing your view of it. For new players, this segment becomes available after getting their first Arcane, a waypoint will flash to direct them to the Segment once they do. The former Arcane button in the Foundry will now direct players to the new Segment.\nPlayers will still be able to access and upgrade their Arcanes in their Arsenal, but this Segment will allow for an extensive look at what Arcanes they have across all equipment types!\nThe Exodia Force Arcane will no longer trigger Saryn’s Toxic Lash.\nCleaned up some FX related to the Exodia Force Arcane.",
      "fixes": "Lua Thrax Plasm can also be traded at Archimedean Yonta in the Chrysalith for the rotation rewards listed above (Voruna Blueprints, Sarofang Blueprint, Perigale Blueprint, and New Arcanes).\nExcalibur Umbra will lose the Health increase from Arcane Blessing after using Transference."
    },
    {
      "name": "Echoes of Veilbreaker: Update 32.1",
      "date": "2022-11-02T15:00:14Z",
      "url": "https://forums.warframe.com/topic/1329684-echoes-of-veilbreaker-update-321/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Energy Motes generated by the Emergence Dissipate Arcane not providing Energy if picked up by Necramech. "
    },
    {
      "name": "Veilbreaker: Revenant Prime: Hotfix 32.0.12",
      "date": "2022-10-12T19:39:20Z",
      "url": "https://forums.warframe.com/topic/1327814-veilbreaker-revenant-prime-hotfix-32012/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Theorem Demulcent Arcane applying its weapon damage increase buff to Wisp’s Breach Surge, Baruuk’s Desolate Hands, and Ember’s Fireball. "
    },
    {
      "name": "Veilbreaker: Revenant Prime: Hotfix 32.0.9",
      "date": "2022-10-05T17:59:06Z",
      "url": "https://forums.warframe.com/topic/1327014-veilbreaker-revenant-prime-hotfix-3209/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed damage types from equipped weapons applying to Mag’s Shards while she is standing in a zone created by Residual Arcanes. \nThe Arcane effects apply only to weapons, her Shards will only apply Slash damage as intended while in these zones. "
    },
    {
      "name": "Veilbreaker: Update 32",
      "date": "2022-09-07T15:00:11Z",
      "url": "https://forums.warframe.com/topic/1321162-veilbreaker-update-32/",
      "additions": "",
      "changes": "Primary/Secondary Merciless Arcane: Removed its +100% Ammo Maximum bonus. \nIn light of the overall speed buff to holster speed, we have made the following changes to the mods and Arcanes that offered holster speed stat increases:\nPrimary/Secondary Dexterity Arcane:",
      "fixes": "Fixed Titania’s Razorflies triggering the effect of the Secondary Deadhead and Pax Seeker Arcanes. \nFixed issues of equipping two Amp Arcanes that convert Amp damage to single type (ie. heat) not doing instances of void damage whatsoever. \nFixed an issue where ‘On Void Sling’ Arcanes were mistakenly applying multiple times. This also applied to: \nFixed losing all your Molt Augmented Arcane stacks after a Host migration. "
    },
    {
      "name": "Khora Prime: Hotfix 31.7.2",
      "date": "2022-08-17T14:59:12Z",
      "url": "https://forums.warframe.com/topic/1319647-khora-prime-hotfix-3172/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Ultimatum not proccing off Ash's Bladestorm."
    },
    {
      "name": "Khora Prime: Hotfix 31.7.1",
      "date": "2022-07-28T17:55:50Z",
      "url": "https://forums.warframe.com/topic/1318073-khora-prime-hotfix-3171/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcanes not having a description when hovered over in Mission Progress and End of Mission screen.\nFixed Necramech Fury's description showing ‘+1 Arcane Revive’."
    },
    {
      "name": "Nora’s Mix Vol.2: Hotfix 31.6.4",
      "date": "2022-07-14T15:00:33Z",
      "url": "https://forums.warframe.com/topic/1316486-nora%E2%80%99s-mix-vol2-hotfix-3164/",
      "additions": "",
      "changes": "",
      "fixes": "Also fixed Arcanes that have decaying stacks losing all their bonus when a stack expires. \nFixed Rank 3 or above Molt Augmented and Molt Reconstruct Arcanes not giving extra Warframe revives.\nAlso fixed the Molt Arcanes and Arcane Pulse not indicating that they will give an extra Warframe revive at Rank 3 and up. \nFixed “Can Refresh” label appearing in the UI for Arcanes that do not have a Duration element to it. \nFixed the Emergence Savior Arcane giving Clients immortality after they die instead of triggering on death.  \nFixed the Cascadia Empowered Arcane buffs applying to Companion or Sentinel weapons.\nFixed the Arcanes in Cavalero’s Offerings appearing under the ‘Mods’ category. As reported here: https://forums.warframe.com/topic/1314743-cavalero-shop-has-the-wrong-category/"
    },
    {
      "name": "Echoes of the Zariman: Hotfix 31.6.1",
      "date": "2022-06-09T21:02:57Z",
      "url": "https://forums.warframe.com/topic/1313629-echoes-of-the-zariman-hotfix-3161/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed accidental trade bans after picking up an Arcane dropped from a Void Angel. As PSA’d here: https://forums.warframe.com/topic/1313615-psa-rectifying-accidental-trade-bans-after-echoes-of-the-zariman/\nFixed the “Requires an Amp Arcane Adapter” error message popping up instead of the Arcane Adapter Slot one for Primary and Secondary weapons if you do not own the corresponding Adapter."
    },
    {
      "name": "Echoes of the Zariman: Update 31.6",
      "date": "2022-06-09T14:49:52Z",
      "url": "https://forums.warframe.com/topic/1313506-echoes-of-the-zariman-update-316/",
      "additions": "",
      "changes": "The Amp Arcane Adapter fuses with an Amp to unlock an additional Arcane Slot. These can be purchased with Holdfast Standing from Cavalero.\nTo install the Amp Arcane Adapter and unlock an additional Slot, go to your Operator Equipment > Select the Amp you wish to unlock an additional Slot > Select the ‘Locked’ slot and hit “ok” to install the Amp Arcane Adapter.\nWith the addition of several new Amp Arcanes with Angels of the Zariman and Echoes of the Zariman, this new Slot allows you to make the most of your growing Arcane collection!\nThis new Arcane slot is only visible in the Operator Equipment screen once the Angels of the Zariman quest has been completed since Cavalero must be visited in the Chrysalith to get the Amp Arcane Adapter.\nFixed Thrax Legatus enemies being able to inflict Magnetic Status procs on Warframes with a Rank 5 Arcane Nullifier equipped (102% chance to resist Magnetic Status Effects entirely).\nWe’ve adjusted the values of older Arcanes with the goal of making lower rank Arcanes more viable, and more desirable while collecting as a result. In short, there are three stats that each Arcane tends to have (speaking generally) -\nCertain Arcanes increase all of these stats per rank (making them scale exponentially), but we are changing it so that only one stat is affected by rank (making it scale linearly). The other stats will now be at the max-rank value regardless of the Arcane Rank, thereby buffing lower-rank Arcanes as a result.\nFor example, using Arcane Fury.\n*Arcane Barrier is the one true exception to the Arcane changes below, since it has two scaling stats, but one of those stats actually becomes worse with rank. Rank 3-4 for example increases the cooldown from 4s to 5s. So if we were to change it to have constant cooldown like the others, we would be increasing the cooldown duration for the lower ranks \nwhich is something we want to avoid! Any other possible changes might have to alter how the Arcane works overall, so we ultimately decided to leave it untouched.\n*As a bonus for the four Residual Arcanes above, we’ve added some extra improvements \nAdded FX backdrop to equipped Arcanes in the Operator Equipment screen.\nChanged color on Amp Arcanes to all use the same green. \nUpdated Arcane icons back to being square.\nThis is always how the Arcane has functioned, we are simply updating the description.",
      "fixes": "The following weapons, Arcanes, and special items have been added to Cavalero’s Offerings in the Chrysalith!\nFixed the Arcane icons being improperly sized when equipped. \nFixed being able to equip multiple Arcanes on Kitguns through chat-link. \nFixed exploding barrels triggering weapon Arcanes. \nFixed camera positioning when hovering over Operator Arcanes."
    },
    {
      "name": "Zephyr & Chroma Prime Vault: Hotfix 31.5.10 + 31.5.10.1",
      "date": "2022-05-17T17:31:55Z",
      "url": "https://forums.warframe.com/topic/1311547-zephyr-chroma-prime-vault-hotfix-31510-315101/",
      "additions": "",
      "changes": "Added in-mission markers to Arcane drops. ",
      "fixes": ""
    },
    {
      "name": "Angels of the Zariman: Hotfix 31.5.4",
      "date": "2022-05-02T20:54:52Z",
      "url": "https://forums.warframe.com/topic/1308448-angels-of-the-zariman-hotfix-3154/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Nekros’ Desecrate and Hydroid’s Pilfering Swarm being able to duplicate Voidplume Pinion and Arcane drops from Void Manifestation fights. \nFixed the new Arcanes from the Angels of the Zariman update all displaying as ‘Common’ in the mission progress screen, regardless of their actual rarity. "
    },
    {
      "name": "Angels of the Zariman: Hotfix 31.5.2",
      "date": "2022-04-28T20:22:09Z",
      "url": "https://forums.warframe.com/topic/1306962-angels-of-the-zariman-hotfix-3152/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Cascadia Overcharge description incorrectly stating that it’s a Warframe Arcane, when it is a Secondary Weapon Arcane. "
    },
    {
      "name": "Update 31.5: Angels of the Zariman",
      "date": "2022-04-27T15:02:06Z",
      "url": "https://forums.warframe.com/topic/1305880-update-315-angels-of-the-zariman/",
      "additions": "",
      "changes": "Incarnon Weapons + New Arcanes \nThe Incarnon Weapon vendor in the Chrysalith also offers 10 new Arcanes in their Offerings! They also have a chance of dropping from defeated Thrax and Void Manifestation enemies.\n\"On Void Blast\" Arcanes now trigger on Void Sling.\nAmp appearance, Arcane, Focus Lens selection conditionally available upon unlocking an Amp.\nArcane descriptions now include what they can be equipped on. \nAdded sounds to Arcane selection in the Mod Screen.\nUpdated Arcane buff FX. ",
      "fixes": "Fixed the Magus Destruct Arcane increasing enemy resistance to Puncture damage, not decreasing like intended. \nFixed selecting Operator Arcane being unreliable/unresponsive on first interaction. \nAlso fixes the UI getting confused between Operator Arcane and Amp Arcane in follow up interactions. "
    },
    {
      "name": "Nora's Mix Volume 1: Update 31.2.0 + 31.2.0.1",
      "date": "2022-03-16T17:58:08Z",
      "url": "https://forums.warframe.com/topic/1302788-noras-mix-volume-1-update-3120-31201/",
      "additions": "",
      "changes": "Arcane Grace ",
      "fixes": ""
    },
    {
      "name": "Echoes of War: Hotfix 31.1.2",
      "date": "2022-02-10T19:28:28Z",
      "url": "https://forums.warframe.com/topic/1299938-echoes-of-war-hotfix-3112/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Relics and Arcanes appearing far too large within the confines of the boxes in the Codex. "
    },
    {
      "name": "Echoes of War: Hotfix 31.1.1",
      "date": "2022-02-09T21:19:47Z",
      "url": "https://forums.warframe.com/topic/1299718-echoes-of-war-hotfix-3111/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane icons appearing stretched in Chat linked loadouts. "
    },
    {
      "name": "Update 31.1.0: Echoes of War",
      "date": "2022-02-09T15:59:43Z",
      "url": "https://forums.warframe.com/topic/1299619-update-3110-echoes-of-war/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed misaligned UI animations in the themed Arcane Manager screen. "
    },
    {
      "name": "The New War: Hotfix 31.0.6",
      "date": "2022-01-04T20:03:51Z",
      "url": "https://forums.warframe.com/topic/1295743-the-new-war-hotfix-3106/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Virtuous Arcanes not functioning for a post-The New War character. "
    },
    {
      "name": "The New War: Hotfix 31.0.3",
      "date": "2021-12-17T21:26:42Z",
      "url": "https://forums.warframe.com/topic/1292266-the-new-war-hotfix-3103/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed moments where Focus/Arcane attributes were being applied to the Operator when they shouldn't during The New War Quest. "
    },
    {
      "name": "Prime Resurgence: Hotfix 30.9.2",
      "date": "2021-11-11T19:53:50Z",
      "url": "https://forums.warframe.com/topic/1286170-prime-resurgence-hotfix-3092/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed livelock that could occur when fusing Operator Arcanes and using all of your remaining unranked in one operation."
    },
    {
      "name": "Update 30.9.0: Prime Resurgence",
      "date": "2021-11-11T13:33:32Z",
      "url": "https://forums.warframe.com/topic/1286070-update-3090-prime-resurgence/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed weapons with 1 bullet in their magazine (such as Vectis and Exergis) bypassing the Fire Rate limit if equipped with a max rank Arcane Pistoleer.\nFixed the Inventory weapon summary not indicating that an Arcane Adapter is installed.\nAn icon will now appear for an installed Arcane Adapter."
    },
    {
      "name": "Nights of Naberus: Hotfix 30.8.1",
      "date": "2021-10-06T19:47:19Z",
      "url": "https://forums.warframe.com/topic/1283279-nights-of-naberus-hotfix-3081/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed having an unranked weapon Arcane left equipped after combining all Arcanes of that type into a new one."
    },
    {
      "name": "Nidus Prime & Plague Star: Hotfix 30.7.6",
      "date": "2021-09-23T20:34:00Z",
      "url": "https://forums.warframe.com/topic/1281923-nidus-prime-plague-star-hotfix-3076/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed certain Arcane HUD elements not having the proper icon scaling. "
    },
    {
      "name": "Update 30.7: Nidus Prime & Plague Star",
      "date": "2021-09-08T16:58:55Z",
      "url": "https://forums.warframe.com/topic/1279845-update-307-nidus-prime-plague-star/",
      "additions": "",
      "changes": "Infested Zaw Arcanes ",
      "fixes": ""
    },
    {
      "name": "Nightwave: Nora’s Choice: Update 30.6.0",
      "date": "2021-08-04T17:32:47Z",
      "url": "https://forums.warframe.com/topic/1275793-nightwave-nora%E2%80%99s-choice-update-3060/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed inability to equip the Secondary Deadhead Arcane on the Athodai. "
    },
    {
      "name": "Sisters of Parvos: Hotfix 30.5.3",
      "date": "2021-07-09T15:07:34Z",
      "url": "https://forums.warframe.com/topic/1270868-sisters-of-parvos-hotfix-3053%C2%A0%C2%A0/",
      "additions": "",
      "changes": "",
      "fixes": "In Update 30.5, a restriction was added to the Exodia Contagion Arcane for Zaws, where it can only activate once while you remain in the air. To use Contagion again you must touch the ground."
    },
    {
      "name": "Sisters of Parvos: Hotfix 30.5.1",
      "date": "2021-07-06T21:24:18Z",
      "url": "https://forums.warframe.com/topic/1269937-sisters-of-parvos-hotfix-3051/",
      "additions": "",
      "changes": "Removed mentions of Arcane Adapter Blueprints in Tehsin’s Steel Honors store ",
      "fixes": "Fixed PHs appearing in Arcane Adapter."
    },
    {
      "name": "Update 30.5: Sisters of Parvos",
      "date": "2021-07-06T15:02:10Z",
      "url": "https://forums.warframe.com/topic/1269749-update-305-sisters-of-parvos/",
      "additions": "",
      "changes": "Acolytes will now drop 1 of 6 New Arcanes on death!\nPrimary and Secondary Weapon Arcanes\nBlood Rush’s maximum value is being lowered.This changes the achievability of consistent Red Crits from just one Mod (on most Melee weapons, some High-Critical exceptions), and now additional help will be needed via Mods, Arcanes, or Warframe abilities to achieve consistent Red Crits.\nnamely, the Steel Path. You’ll be able to get the Arcane Slot Unlockers from Steel Path Honors, whereas the Arcanes themselves drop from Acolytes in the Steel Path! The spawn frequency of Acolytes is also being increased, meaning you’ll get more Steel Essence, as well as more chances for the Arcane you want. All Acolytes will drop them!\nThere are 6 Arcanes (3 Primary, 3 Secondary) that you’ll be able to rank up and put to put into the new Slots:\nThe second Primary Arcane is designed with high precision weapons in mind. Make your Headshots count for greater performance (excluding AOE headshots)!:\nThe final Primary Arcane is designed with using your full loadout in mind, enter Melee synergy:\nThe stats are the same for the Secondary versions of the New Arcanes:\nUltimately each Arcane stacks up to the same amount of damage, but it’s how you choose to get there that counts. Precision? Spray and Pray? Melee synergy? Your choice!\nThe “Arcane Adapters themselves are an item you’ll need to install on the weapons you want to take further, and you can find the Primary and Secondary Arcane Adapters in the Steel Path Honors. The Arcanes themselves drop from Acolytes on The Steel Path.",
      "fixes": "Fixed buff notification for Arcane Energize and Grace showing up as a cooldown instead of an upgrade duration. It now appears as a debuff."
    },
    {
      "name": "Gara Prime: Hotfix 30.3.5",
      "date": "2021-06-10T19:47:41Z",
      "url": "https://forums.warframe.com/topic/1267287-gara-prime-hotfix-3035/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcanes not triggering from shooting enemies while riding on a K-Drive."
    },
    {
      "name": "Call of the Tempestarii: TennoGen: Hotfix 30.1.1",
      "date": "2021-05-05T21:12:25Z",
      "url": "https://forums.warframe.com/topic/1262687-call-of-the-tempestarii-tennogen-hotfix-3011/",
      "additions": "",
      "changes": "Fixed not being awarded the final Arcane for completing the 36th Orphix.",
      "fixes": ""
    },
    {
      "name": "Update 29.10.0: Corpus Proxima & The New Railjack",
      "date": "2021-03-19T15:18:34Z",
      "url": "https://forums.warframe.com/topic/1253565-update-29100-corpus-proxima-the-new-railjack/",
      "additions": "",
      "changes": "Warframe Arcanes in Rotation C (i.e every 12 Orphix) of Orphix gamemode in Corpus Railjack (max 36 Orphix). ",
      "fixes": ""
    },
    {
      "name": "Orphix Venom: Hotfix 29.6.4",
      "date": "2021-01-06T16:33:32Z",
      "url": "https://forums.warframe.com/topic/1244110-orphix-venom-hotfix-2964/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Host not replicating lightning bolts FX from Residual Shock Arcane to Clients."
    },
    {
      "name": "Update 29.6.0: Orphix Venom!",
      "date": "2020-12-18T19:25:12Z",
      "url": "https://forums.warframe.com/topic/1241202-update-2960-orphix-venom/",
      "additions": "",
      "changes": "",
      "fixes": "Warframe Arcanes\nVazarin Dash, Trinity’s Blessing, Equinox's Mend, Rejuvenate Aura, and Arcane Pulse no longer heal Necramechs. Out of 21 sources of healing, these five slipped through. "
    },
    {
      "name": "Deimos: Arcana: Prime Vault 29.5.9",
      "date": "2020-12-15T18:58:49Z",
      "url": "https://forums.warframe.com/topic/1240704-deimos-arcana-prime-vault-2959/",
      "additions": "",
      "changes": "Deimos Arcanes",
      "fixes": ""
    },
    {
      "name": "Deimos: Arcana: Hotfix 29.5.6",
      "date": "2020-12-01T22:31:33Z",
      "url": "https://forums.warframe.com/topic/1238756-deimos-arcana-hotfix-2956/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Residual Arcane zone damage numbers not showing up for Clients."
    },
    {
      "name": "Deimos: Arcana: Hotfix 29.5.5 + 29.5.5.1",
      "date": "2020-11-27T20:28:28Z",
      "url": "https://forums.warframe.com/topic/1238119-deimos-arcana-hotfix-2955-29551/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed rare script error when clicking on a Mod Link that contains an Arcane."
    },
    {
      "name": "Update 29.5: Deimos Arcana",
      "date": "2020-11-19T23:39:27Z",
      "url": "https://forums.warframe.com/topic/1236257-update-295-deimos-arcana/",
      "additions": "",
      "changes": "Actualise... synergy... SYNERGISE! Residual Arcanes and Theorem Arcanes are geared towards synergising with each other to provide a different method of imposing elemental damage. Discover these new synergized Kitgun and Warframe Arcanes as rewards from the new Isolation Vault Bounties.\n*Please review changes to these Arcanes at the bottom of the document from our Public Test Cluster!\n20% chance to create a pool of toxic blood for 12s, dealing 40 Toxin Damage/s. Standing in the area applies the Toxin Damage to Theorem Arcanes.\n20% chance to create a frigid mist for 12s, dealing 40 Cold Damage/s. Standing in the area applies the Cold Damage to Theorem Arcanes.\n20% chance to spawn volatile hives for 12s that explode when enemies approach for 80 Heat Damage in 10m. Standing in the area applies Heat Damage type to Theorem Arcanes.\n20% chance to spawn an electrified spike for 12s, dealing 200 Electricity Damage to enemies within 10m. Standing in the area applies Electricity Damage to Theorem Arcanes.\nStanding in a zone created by a Residual Arcane creates a globe that orbits the player every 2s. The globes will strike the nearest enemy within 15m dealing *150 damage *increasing their vulnerability to the globe’s damage type by 200% for 6s. Globes will persist for *30s upon leaving the zone.\n+1 Arcane Revive.\nStanding in a zone created by a Residual Arcane increases weapon damage by *12%/s stacking up to 15x. Effect persists for 20s upon leaving the zone.\n+1 Arcane Revive.\nStanding in a zone created by a Residual Arcane increases damage of Companions and summoned Allies within *90m by *24%/s stacking up to 15x. Effect persists for 20s upon leaving the zone.\n+1 Arcane Revive.\nUpdated Arcane Pulse’s description to include the heal amount and radius. \nGenerally speaking, the major focus this week was split between Weapon tweaks and major Arcane buffs to give another chance at usability. The Arcane readability, numbers, and more have been significantly reviewed and changed for Deimos: Arcana!\nThe Theorem and Residual Arcanes were the most contested items on the Test Cluster over the weekend. We have made a concerted effort to reset how they work in context (mainly where the AoE spawns) and rebalance. The next steps for us are receiving feedback on the newer flow and newer stats for these upon release!\nBefore: Standing in a zone created by a Residual Arcane increases damage of companions and summoned allies within 60m by 4%/s stacking up to 15x. Effect persists for 20s upon leaving the zone.\nBefore: Standing in a zone created by a Residual Arcane creates a globe that orbits the player every 2s. The globes will strike the nearest enemy within 5m dealing 100 damage and applying a Status Effect. Globes will persist for 8s upon leaving the zone.\nBefore: Standing in a zone created by a Residual Arcane increases weapon damage by 4%/s stacking up to 15x. Effect persists for 5s upon leaving the zone.",
      "fixes": "Fixed Arcane buffs deactivating when you switch to/from Archwing in Orb Vallis or Plains of Eidolon.\nKitgun + Weapon + Arcanes"
    },
    {
      "name": "Heart of Deimos: Update 29.1.0",
      "date": "2020-09-17T15:55:25Z",
      "url": "https://forums.warframe.com/topic/1225874-heart-of-deimos-update-2910/",
      "additions": "",
      "changes": "Fixed Marked for Death with Arcane Trickery equipped triggering invisibility almost every time, because each enemy hit by the AoE has its own 15% chance to activate Arcane Trickery.",
      "fixes": ""
    },
    {
      "name": "Derelict Shift: Update 28.3.0",
      "date": "2020-08-12T18:34:52Z",
      "url": "https://forums.warframe.com/topic/1212935-derelict-shift-update-2830/",
      "additions": "",
      "changes": "This Hotfix ensures that Resource Drop Chance Boosters work within The Steel Path for Riven Slivers and Steel Essence. They were initially turned off to ensure the inherent Steel Path Resource Drop Chance Booster worked as intended. Like the initial Arcane Marketplace in Scarlet Spear, we wanted to ensure the automatic ‘100% Steel Path Resource Drop Chance Booster’ didn’t unintentionally scale for Resources specific to the Steel Path (Riven Slivers and Steel Essence). This unfortunately also affected player obtained reward Resource Drop Chance Boosters.\nOperator Amps damage conversion Arcanes now convert to 98% max so they can always deal a bit of Void Damage.",
      "fixes": ""
    },
    {
      "name": "The Deadlock Protocol: Hotfix 28.0.6 + 28.0.6.1",
      "date": "2020-06-24T18:00:22Z",
      "url": "https://forums.warframe.com/topic/1203007-the-deadlock-protocol-hotfix-2806-28061/",
      "additions": "",
      "changes": "Updated a few occurrences of \"Resist a Damage effect\" to more accurate “Resist a Status effect\" for some descriptions of Arcanes.",
      "fixes": "Fixed Arcane Bodyguard not healing Venari."
    },
    {
      "name": "Warframe Revised: Railjack Revisited (Part 1): Hotfix 27.4.1",
      "date": "2020-05-01T20:16:49Z",
      "url": "https://forums.warframe.com/topic/1189425-warframe-revised-railjack-revisited-part-1-hotfix-2741/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed a server script error when changing Arcanes."
    },
    {
      "name": "Warframe Revised: Railjack Revisited (Part 1): Update 27.4",
      "date": "2020-05-01T13:21:46Z",
      "url": "https://forums.warframe.com/topic/1189247-warframe-revised-railjack-revisited-part-1-update-274/",
      "additions": "",
      "changes": "Avionics, Mods, and Arcanes that are not owned are marked as ‘preview’ in this category and those that you own but have not ranked to Max are also in this category.\nArcane Pulse\nThe Trading screen will now prompt you with a warning when you’re about to Trade an Arcane that is currently equipped.",
      "fixes": "Fixed Arcane tooltip in your Inventory showing more information than necessary. The tooltip of the selected Arcane should only show the Rank information."
    },
    {
      "name": "Scarlet Spear: 27.3.13",
      "date": "2020-04-14T19:52:12Z",
      "url": "https://forums.warframe.com/topic/1185141-scarlet-spear-27313/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed \"on hit\" Arcanes triggering when using Railjack weapons."
    },
    {
      "name": "Scarlet Spear: 27.3.12",
      "date": "2020-04-14T13:32:16Z",
      "url": "https://forums.warframe.com/topic/1185014-scarlet-spear-27312/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Momentum not activating for Critical shots on the Condrix.\nFixed UI overlapping when viewing Arcanes and then opening a Chat link."
    },
    {
      "name": "Scarlet Spear: 27.3.11",
      "date": "2020-04-09T19:32:11Z",
      "url": "https://forums.warframe.com/topic/1183684-scarlet-spear-27311/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane \"Upgraded\" counter including equipped Arcanes even if they are not at max rank."
    },
    {
      "name": "Scarlet Spear: TennoGen 27.3.10",
      "date": "2020-04-08T15:02:18Z",
      "url": "https://forums.warframe.com/topic/1183272-scarlet-spear-tennogen-27310/",
      "additions": "",
      "changes": "Fixed Operator Magus Arcanes not functioning in consecutive Murex Raids.",
      "fixes": ""
    },
    {
      "name": "Operation: Scarlet Spear: TennoGen 27.3.8 + 27.3.8.1 + 27.3.8.2",
      "date": "2020-04-02T16:33:08Z",
      "url": "https://forums.warframe.com/topic/1181462-operation-scarlet-spear-tennogen-2738-27381-27382/",
      "additions": "",
      "changes": "We have added the Rare and Legendary Arcanes to Little Duck’s Trade for 2000 Credits Per Rare and 4000 Credits Per Legendary.",
      "fixes": ""
    },
    {
      "name": "Operation: Scarlet Spear: Hotfix 27.3.1",
      "date": "2020-03-24T19:21:39Z",
      "url": "https://forums.warframe.com/topic/1178166-operation-scarlet-spear-hotfix-2731/",
      "additions": "",
      "changes": "We are Adding more Arcanes to complete the collection.",
      "fixes": ""
    },
    {
      "name": "Operation: Scarlet Spear: 27.3.0",
      "date": "2020-03-24T17:03:07Z",
      "url": "https://forums.warframe.com/topic/1178095-operation-scarlet-spear-2730/",
      "additions": "",
      "changes": "Once again your favourite cross-armed lady, Little Duck, has taken her interest of Exotic Goods even further to aid you in Operation Scarlet Spear. Keep in mind that you won’t see all Arcanes in Little Duck's Trade Offerings \nWe are making changes to Magus Lockdown due to it being virtually one of the best CC and Damage combinations in the game. In some cases you can clear waves of Elite Sanctuary Onslaught without even using a Warframe or Weapon, and simply spam Magus Lockdown to victory. This goes well beyond the intended use of it and we are changing it to a CC only Arcane \nMagus Lockdown no longer affects the Golden Maw due to the Arcane killing the Maw and preventing Quest completion. \nUpdated the Arcane ‘DISTILL’ term to ‘BREAK DOWN’ to acquaint new Arcane Management mechanics.\nAdded tips to the Arcane screen for ‘BREAK DOWN’, Arcanes that refresh, and how to obtain respective Arcanes. \nArcanes are now displayed with their respective Icon in the HUD, instead of the generic Arcane icon.\nThe Rank-Up callout is now displayed to the left of the next Arcane Rank in the Arcane Upgrade screen when using a controller.\nArcanes that are sold in Vendor Offerings will now only show Max Rank tooltip information, and UI indication that the Arcane you’re purchasing is Unranked. \nThis fixes Arcane tooltips expanding off screen when viewed in Vendor Offerings. \nClarified Arcane requirements for Arcane Trickery, Arcane Ultimatum, and Exodia Might by adding ‘Kill’ to the ‘On Finisher’ line.",
      "fixes": "Fixed two Callouts for the same button, Sort and Break Down, in the Arcane Manager Screen when using a controller.\nFixed Rank Up button not appearing outside of the Arcane Equip screen when opened via Foundry.\nFixed black squares appearing in the Arcane Manager screen and Relic Manager screen with Deferred Rendering enabled.\nFixed the Arcane Manager ‘Incomplete’ section displaying Arcanes that have been max Ranked."
    },
    {
      "name": "Warframe Revised: Hotfix 27.2.2",
      "date": "2020-03-06T20:01:27Z",
      "url": "https://forums.warframe.com/topic/1173118-warframe-revised-hotfix-2722/",
      "additions": "Added an ‘Incomplete’ category to Arcane management screen when accessed via Foundry.",
      "changes": "",
      "fixes": "Fixed inboxed Avionics and Arcanes not showing up properly (Prime Time bug!).\nFixed the Arcane Manager screen incorrectly showing you how many Max Rank Arcanes you own."
    },
    {
      "name": "Warframe Revised: Hotfix 27.2.1",
      "date": "2020-03-05T23:06:21Z",
      "url": "https://forums.warframe.com/topic/1172684-warframe-revised-hotfix-2721/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Arcane Manager ‘UPGRADED’ value not counting Arcanes that are Rank 3+.\nFixed cases where you could equip two of the same Operator Arcane simultaneously.\nFixed a script error that would occur when opening the Warframe Arcane Manager screen.\nFixed unlocalized ‘Rank Up’ button in the Arcane Manager screen."
    },
    {
      "name": "Warframe Revised: Update 27.2.0",
      "date": "2020-03-05T15:32:32Z",
      "url": "https://forums.warframe.com/topic/1172454-warframe-revised-update-2720/",
      "additions": "",
      "changes": "Increasing the maximum Rank of Warframe and Operator Arcanes to 5, up from 3. Arcane Revives are a bonus that begins on Rank 3. \nAdjusting the power of Arcanes at Rank 5 to generally behave as if you had 1.5 equipped, list as follows:\nArcane Acceleration:\nArcane Aegis: \nArcane Agility:\nArcane Arachne: \nArcane Avenger:\nArcane Awakening:\nArcane Barrier:\nArcane Blade Charger:\nArcane Bodyguard: \nArcane Consequence:\nArcane Deflection:\nArcane Energize:\nArcane Energize will still give Energy to toggled Warframe Abilities \nArcane Eruption:\nArcane Fury:\nArcane Grace\nArcane Guardian\nArcane Healing\nArcane Ice\nArcane Momentum\nArcane Nullifier\nArcane Phantasm\nArcane Pistoleer\nArcane Precision\nArcane Primary Charger\nArcane Pulse \nArcane Rage\nArcane Resistance\nArcane Strike\nArcane Tanker\nArcane Tempo\nArcane Trickery\nArcane Ultimatum \nArcane Velocity\nArcane Victory\nArcane Warmth\nWhy: The reasoning here is mainly toward the ability to equip two of the same Arcane. This reasoning is one of past inconsistency and time determining intent. There are a lot of builds that specialize the use of two Arcanes, but we want to encourage a variety instead of duplication. Arcanes are the only Upgrade system in the game that allows two of the exact same upgrade to be equipped \nand we would rather players have variety than duplications. In the same way you can’t equip Amalgam Serration and regular Serration, you can’t equip multiple Rivens per weapon, or any duplication of Mods at all, Arcanes will follow. But we are making major changes to the Ranking (up from 3 to 5 with power changes). Instead of having 2 of the same Arcane with a double effect, you can now choose between 2 different Arcanes that behave (generally) at 1.5x efficacy than before.\nWhy: The conversation surrounding Arcane Guardian led to a significant review of Armor stats on Warfarmes. The Majority of Warframes received an increase in the Armor stat to increase survivability. Compounded with Shield Gating and the numerous other changes covered, we expect a much more fair feeling playing field for all Warframes.",
      "fixes": "A player who has not yet completed 'Rising Tide' will find building their Railjack a lot easier. A player with an extensive Arcane collection will see most Arcanes can now achieve 5 Ranks, and duplicates can no longer be equipped together. A player using the Kuva Bramma will notice Self Damage is gone. A player with Multishot mods equipped should see some changes in their Upgrade Menus with the addition of a new Multishot Stat.\nFixed missing controller button callout to Rank up an Arcane in the Arcane Manager screen"
    },
    {
      "name": "Empyrean: Ivara Prime 27.0.7",
      "date": "2019-12-19T22:17:20Z",
      "url": "https://forums.warframe.com/topic/1155166-empyrean-ivara-prime-2707/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Fulmin (only when reduced to below 10 rounds on Semi-Auto firing mode), Flux Rifle, Kitguns with the 'Gaze' Chamber type and Pax Charge Arcane, Imperator/Vandal, Cycron, and Larkspur not recharging correctly."
    },
    {
      "name": "Update 26: The Old Blood",
      "date": "2019-10-31T20:07:22Z",
      "url": "https://forums.warframe.com/topic/1136784-update-26-the-old-blood/",
      "additions": "",
      "changes": "This update will also see some changes to the following Weapon, Arcane and Focus School buffs:",
      "fixes": ""
    },
    {
      "name": "Atlas Prime: Hotfix 25.8.1 + 25.8.1.1",
      "date": "2019-10-09T15:29:13Z",
      "url": "https://forums.warframe.com/topic/1133158-atlas-prime-hotfix-2581-25811/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Pistoleer not giving infinite Ammo at 100% efficiency for weapons that fire more than one bullet at a time."
    },
    {
      "name": "TennoGen & Nightwave Episode 5: Hotfix 25.7.8",
      "date": "2019-09-26T21:00:27Z",
      "url": "https://forums.warframe.com/topic/1130720-tennogen-nightwave-episode-5-hotfix-2578/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Blade Charger displaying the damage increase factor (2%) instead of as a percentage (200%). The Arcane still behaved correctly and applied 200%. As reported here: https://forums.warframe.com/topic/1130656-tennogen-nightwave-episode-5-hotfix-2577/page/5/?tab=comments#comment-11055621 "
    },
    {
      "name": "TennoGen & Nightwave Episode 5: Hotfix 25.7.7",
      "date": "2019-09-26T16:10:33Z",
      "url": "https://forums.warframe.com/topic/1130656-tennogen-nightwave-episode-5-hotfix-2577/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Primary Charger, Arcane Blade Charger, Arcane Pistoleer, and Arcane Bodyguard not listing “+1 Arcane Revive” at max Rank."
    },
    {
      "name": "Arbitrations Revisited: Hotfix 25.7.6",
      "date": "2019-09-18T19:13:08Z",
      "url": "https://forums.warframe.com/topic/1129207-arbitrations-revisited-hotfix-2576/",
      "additions": "",
      "changes": "Last week we released a Dev Workshop (link on those words: https://forums.warframe.com/topic/1128051-arbitrations-revisited-part-2/) outlining our plans to improve Arbitrations, revisiting the droptables and improving reward frequency to be more consistent between modes. From new players who just completed the Star Chart, to veterans returning for new Arcanes and Aura mods, we hope these changes result in a more satisfying gameplay experience overall.",
      "fixes": ""
    },
    {
      "name": "Prime Vault: Hotfix 25.7.5",
      "date": "2019-09-09T19:05:50Z",
      "url": "https://forums.warframe.com/topic/1127427-prime-vault-hotfix-2575/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed ability to equip Arcanes you do not own through Mod Link. As reported here: https://forums.warframe.com/topic/1126678-stumble-upon-duplicating-glitchyesnomaybe/"
    },
    {
      "name": "Wukong Deluxe: Hotfix 25.5.1 + 25.5.2",
      "date": "2019-08-02T20:04:13Z",
      "url": "https://forums.warframe.com/topic/1117121-wukong-deluxe-hotfix-2551-2552/",
      "additions": "",
      "changes": "Possible item drops / rank stats are now directly indicated on Relic and Arcane description text.",
      "fixes": "Fixed inability to install any Arcane in the second slot on your Warframe, as it would be auto removed after backing out of the screen."
    },
    {
      "name": "The Jovian Concord: Hotfix 25.0.8",
      "date": "2019-05-31T20:15:29Z",
      "url": "https://forums.warframe.com/topic/1099109-the-jovian-concord-hotfix-2508/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcanes awarded through Bounties showing as Mods in the UI."
    },
    {
      "name": "The Jovian Concord: Hotfix 25.0.4",
      "date": "2019-05-28T18:32:28Z",
      "url": "https://forums.warframe.com/topic/1098019-the-jovian-concord-hotfix-2504/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcanes displaying that they can be traded for 100 Ducats."
    },
    {
      "name": "The Jovian Concord: Hotfix 25.0.3",
      "date": "2019-05-24T22:08:21Z",
      "url": "https://forums.warframe.com/topic/1096333-the-jovian-concord-hotfix-2503/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed crashes when going to the Arcane selection screen for a Kitgun while playing in Japanese."
    },
    {
      "name": "Plains of Eidolon Remaster: Hotfix 24.6.2 + 24.6.2.1",
      "date": "2019-04-05T20:50:12Z",
      "url": "https://forums.warframe.com/topic/1080295-plains-of-eidolon-remaster-hotfix-2462-24621/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane ranks displaying with wrong formatting as reported here: https://forums.warframe.com/topic/1080065-that-moment-when-you-finally-earn-nightwave-arcane-energize/ "
    },
    {
      "name": "Plains of Eidolon Remaster: Update 24.6.0",
      "date": "2019-04-04T23:14:14Z",
      "url": "https://forums.warframe.com/topic/1079621-plains-of-eidolon-remaster-update-2460/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Dispatch Overdrive not applying to Garuda’s Talons when equipped (this time without breaking all Arcanes).\nFixed the Pax Soar Arcane buffs taking effect even when your Kitgun is unequipped."
    },
    {
      "name": "Buried Debts: Hotfix 24.5.7",
      "date": "2019-03-28T20:36:19Z",
      "url": "https://forums.warframe.com/topic/1077145-buried-debts-hotfix-2457/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed a script error when accessing the Arcane Manager screen."
    },
    {
      "name": "Buried Debts: Update 24.5.0",
      "date": "2019-03-14T21:06:12Z",
      "url": "https://forums.warframe.com/topic/1070992-buried-debts-update-2450/",
      "additions": "",
      "changes": "1) We have Restored Arcanes not triggering from Exalted weapons across the board by reverting the change made in Update 24.4.0:",
      "fixes": "Fixed the Arcane slot in the Upgrade screen remaining highlighted after install.\nFixed a script error when accessing the Arcane Manager screen."
    },
    {
      "name": "Buried Debts: Update 24.4.0",
      "date": "2019-03-08T02:01:23Z",
      "url": "https://forums.warframe.com/topic/1067397-buried-debts-update-2440/",
      "additions": "",
      "changes": "The Arcane Manager has received a visual overhaul to match the chosen UI Theme! Along with the new visual appeal, the Manager now also:\nLists which Arcanes you do not own (hovering over will display information)\nArcane Ranks displayed on the right-side are now selectable, allowing for a more direct Rank Up mechanic.\nRight clicking an Arcane in the Mod Upgrade screen now unequips it, similar to Mods.",
      "fixes": "Fixed a localization issue with Arcane Fury and Arcane Awakening.\nFixed Magus Firewall Arcane’s FX not appearing for Clients. \nFixed a script error when attempting to equip a Magus Arcane on the Operator.\nMesa's Peacemaker will no longer trigger Arcane Velocity. Arcane Velocity can still be triggered outside of Peacemaker and will apply if Peacemaker is used during its duration."
    },
    {
      "name": "Fortuna: Hotfix 24.3.3 - Nightwave",
      "date": "2019-03-01T16:14:18Z",
      "url": "https://forums.warframe.com/topic/1065360-fortuna-hotfix-2433-nightwave/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed missing Heat icon in the Arcane Manager screen."
    },
    {
      "name": "Fortuna: Hotfix 24.2.11",
      "date": "2019-01-29T19:01:28Z",
      "url": "https://forums.warframe.com/topic/1057949-fortuna-hotfix-24211/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed missing Korean LOC translations for Arcanes. \nFixed some spotloading caused by Arcanes on Operators."
    },
    {
      "name": "Fortuna: Hotfix 24.2.7 + 24.2.7.1",
      "date": "2019-01-09T18:37:18Z",
      "url": "https://forums.warframe.com/topic/1052727-fortuna-hotfix-2427-24271/",
      "additions": "Added a new sound for the Pax Charge Arcane.",
      "changes": "You can purchase multiple Arcanes from the respective Vendors (Rude Zuud, etc). ",
      "fixes": "Fixed Magus Anomaly Arcane pulling Kuva Siphons, Eidolons, Lephantis, and other “boss” type enemies as reported here:\nFixed duplicate ‘UNRANKED’ text on the Arcane Trade screen. \nFixed Little Duck’s Operator Arcanes Tab being mislabeled as Mods.\nFixed some remaining Arcanes still having ALL CAPS descriptions.\nFixed Magus Repair Operator Arcane not functioning for Clients."
    },
    {
      "name": "Fortuna: Hotfix 24.2.6",
      "date": "2018-12-20T22:05:56Z",
      "url": "https://forums.warframe.com/topic/1046637-fortuna-hotfix-2426/",
      "additions": "",
      "changes": "Updated all Arcane Descriptions to not be all CAPS.",
      "fixes": ""
    },
    {
      "name": "Fortuna: The Profit-Taker - Update 24.2",
      "date": "2018-12-18T13:59:14Z",
      "url": "https://forums.warframe.com/topic/1044890-fortuna-the-profit-taker-update-242/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed awkward animations playing when certain Arcanes triggered while reloading your weapon. "
    },
    {
      "name": "Fortuna: Hotfix 24.1.4",
      "date": "2018-12-10T20:09:14Z",
      "url": "https://forums.warframe.com/topic/1042413-fortuna-hotfix-2414/",
      "additions": "",
      "changes": "Updated Arcane Rage description to read \"Primary Weapons\" instead of \"Rifles\".",
      "fixes": ""
    },
    {
      "name": "Fortuna: Hotfix 24.1.1 + 24.1.1.1",
      "date": "2018-11-23T20:52:30Z",
      "url": "https://forums.warframe.com/topic/1036429-fortuna-hotfix-2411-24111/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Victory not triggering when using a Primary weapon. "
    },
    {
      "name": "Fortuna: Hotfix 24.0.7",
      "date": "2018-11-15T20:08:27Z",
      "url": "https://forums.warframe.com/topic/1032058-fortuna-hotfix-2407/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed KitGuns not applying negative damage modifiers for projectile based Chambers and Zaws not applying negative damage modifiers to projectiles launched from the Exodia Contagion Arcane."
    },
    {
      "name": "Fortuna: Hotfix 24.0.4",
      "date": "2018-11-12T22:14:40Z",
      "url": "https://forums.warframe.com/topic/1030143-fortuna-hotfix-2404/",
      "additions": "",
      "changes": "Tooltips for Arcanes now display bonuses in uppercase like the Arcane manager screen.",
      "fixes": ""
    },
    {
      "name": "Fortuna: Update 24.0",
      "date": "2018-11-08T20:46:20Z",
      "url": "https://forums.warframe.com/topic/1025679-fortuna-update-240/",
      "additions": "",
      "changes": "Zuud also offers enhancements for your Kitguns known as Pax Arcanes! Similar to Exodias for Zaw Melee weapons, these Arcanes can be installed on your Kitgun in your Arsenal once they have been Gilded:",
      "fixes": ""
    },
    {
      "name": "Chimera: Update 23.10",
      "date": "2018-10-12T13:56:43Z",
      "url": "https://forums.warframe.com/topic/1016610-chimera-update-2310/",
      "additions": "",
      "changes": "We no longer force double-click to equip Arcanes when playing on a controller.",
      "fixes": "Fixed Operator Arcane customization screen not actually saving Arcane changes.\nFixed deselecting an Arcane not removing the complete Credit value in the Inventory sell screen. "
    },
    {
      "name": "Mask of the Revenant: Update 23.8.0 + 23.8.0.1 + 23.8.0.2",
      "date": "2018-09-12T18:41:12Z",
      "url": "https://forums.warframe.com/topic/1008095-mask-of-the-revenant-update-2380-23801-23802/",
      "additions": "",
      "changes": "",
      "fixes": "With 90% damage resistance, Nezha is still very capable of tanking, but encouraged to rely on his other tools to avoid getting overwhelmed. Taking minimal health damage allows for synergy with Blazing Chakram’s health orbs, not to mention new modding avenues like Equilibrium, Health Conversion and various Arcanes. The change also allows us to improve survivability in other ways, such as the increased health pool, and major Warding Halo quality-of-life buffs listed below.\nFixed ability to trigger Exodia Arcanes on Exalted Melee weapons. Exodia Arcanes still apply to Zaws, just not in Exalted Melee (Excalibur, Valkyr, etc) form."
    },
    {
      "name": "The Sacrifice: Update 23.1.0: TennoGen Round 13",
      "date": "2018-07-18T17:30:11Z",
      "url": "https://forums.warframe.com/topic/984990-the-sacrifice-update-2310-tennogen-round-13/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an issue where attempting to sell a ranked Arcane enhancement through your inventory would fail. "
    },
    {
      "name": "The Sacrifice: Hotfix 23.0.4",
      "date": "2018-06-20T20:44:05Z",
      "url": "https://forums.warframe.com/topic/971991-the-sacrifice-hotfix-2304/",
      "additions": "",
      "changes": "Specters can no longer trigger Operator Virtuos Arcanes.\nFixed Umbra triggering Operator Virtuos Arcanes.",
      "fixes": "Fixed trading partner’s Arcane Rank Icons not displaying on the trading UI screen.\nFixed all ‘on damage’ Arcanes not triggering upon taking damage."
    },
    {
      "name": "The Sacrifice: Hotfix 23.0.3 - Limbo Prime",
      "date": "2018-06-19T17:58:18Z",
      "url": "https://forums.warframe.com/topic/971211-the-sacrifice-hotfix-2303-limbo-prime/",
      "additions": "",
      "changes": "",
      "fixes": "Improved visibility of Arcane Rank indication icons due to large white text field on the Trading screen."
    },
    {
      "name": "Beasts of the Sanctuary: Update 22.20.0",
      "date": "2018-05-17T14:55:31Z",
      "url": "https://forums.warframe.com/topic/957066-beasts-of-the-sanctuary-update-22200/",
      "additions": "",
      "changes": "Arcane ranks are now on the side of the icon rather than in the 'name' field.\nYou can now purchase multiple copies of Arcane Blueprints in Hok and Quills Offerings.",
      "fixes": "Fixed max Ranked Arcanes appearing grayed out in the Codex if you didn’t own a separate Unranked one."
    },
    {
      "name": "Shrine of the Eidolon: Hotfix 22.17.1 - 22.17.1.1",
      "date": "2018-04-05T18:31:09Z",
      "url": "https://forums.warframe.com/topic/942090-shrine-of-the-eidolon-hotfix-22171-221711/",
      "additions": "",
      "changes": "100k Credits for Unranked Legendary Arcane\n300k Credits for Rank 1 Legendary Arcane\n600k Credits for Rank 2 Legendary Arcane\n1M Credits for Rank 3 Legendary Arcane",
      "fixes": ""
    },
    {
      "name": "Shrine of the Eidolon: Update 22.17.0 + 22.17.0.1",
      "date": "2018-03-28T18:49:46Z",
      "url": "https://forums.warframe.com/topic/939097-shrine-of-the-eidolon-update-22170-221701/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane description being partially cut off on the last line if it extends past the text area.\nFixed line breaks in the wrong places for Arcane descriptions in some non-English languages."
    },
    {
      "name": "Shrine of the Eidolon: Hotfix 22.16.3",
      "date": "2018-03-16T21:21:48Z",
      "url": "https://forums.warframe.com/topic/934023-shrine-of-the-eidolon-hotfix-22163/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed grammatical text error in the Arcane Manager where it should be singular 'Warframe' instead of plural 'Warframes'."
    },
    {
      "name": "Shrine of the Eidolon: Update 22.16.0",
      "date": "2018-03-15T20:27:52Z",
      "url": "https://forums.warframe.com/topic/933296-shrine-of-the-eidolon-update-22160/",
      "additions": "",
      "changes": "Removed the ability to purchase multiple of the same Helmet/Syandana from the Market. With the changes to how Arcanes are equipped, buying multiple of the same Helmet/Syandana is not necessary. \nIncreased textbox width for Arcane text in Arcane Manager screen.",
      "fixes": "Fixed the Warframe Arcane in the second slot not triggering if the Excalibur Dex Helmet is equipped."
    },
    {
      "name": "Shrine of the Eidolon: Hotfix 22.15.1",
      "date": "2018-03-08T20:29:51Z",
      "url": "https://forums.warframe.com/topic/931023-shrine-of-the-eidolon-hotfix-22151/",
      "additions": "",
      "changes": "the healing on the Operator will now be instantaneous according to the % in the description of the Arcane.",
      "fixes": "Fixed Arcane Acceleration applying to all Primaries when it should just be Rifles."
    },
    {
      "name": "Shrine of the Eidolon: Update 22.15.0",
      "date": "2018-03-07T21:04:32Z",
      "url": "https://forums.warframe.com/topic/930667-shrine-of-the-eidolon-update-22150/",
      "additions": "",
      "changes": "Arcanes will now appear as \"EQUIPPED\" if already equipped on the Warframe/Operator/Amp/Zaw that is currently being modified.\nThe Arcane name and Rank will appear on the same text line in the Trade confirmation prompt.\nRemoved Arcane slots when in the Conclave Mode of the Arsenal.\nFixed the Arcane Manager in Relays appearing empty. \nFixed not being able to trade the same Arcane at different Ranks at the same time.",
      "fixes": "Fixed a script error when attempting to replace a Mod with an Arcane in the Trading screen."
    },
    {
      "name": "Shrine of the Eidolon: Hotfix 22.14.2",
      "date": "2018-03-02T20:17:40Z",
      "url": "https://forums.warframe.com/topic/928851-shrine-of-the-eidolon-hotfix-22142/",
      "additions": "",
      "changes": "When Upgrading an Arcane, you can now specify to what level you want to Upgrade it to.\nWhen Upgrading an Arcane, you can now do multiple Ranks in a single operation.\nThe ‘NEXT RANK’ button will appear grayed out if you do not have the required amount of Arcanes to proceed to the next rank.\nChanged the Arcane Rank indicators to be Arcane icons instead of numbers.",
      "fixes": "Fixed a crash that occurred when returning to Cetus after acquiring an Arcane.\nFixed being able to equip a single Magus into both Arcane slots on your Operator. You must own more than one copy of the Magus in order to do so.\nFixed a case where you could equip 3 Arcanes on your Warframe.\nFixed the UI locking up when Upgrading your Warframe and then changing your currently equipped Arcane to \"none\"."
    },
    {
      "name": "Shrine of the Eidolon: Update 22.14.0",
      "date": "2018-03-01T18:20:15Z",
      "url": "https://forums.warframe.com/topic/928296-shrine-of-the-eidolon-update-22140/",
      "additions": "",
      "changes": "If you need an Arcane refresher, this spoiler is for you!\nUpon logging in, all Arcanes installed on Cosmetics/Helmets will be Distilled because Arcanes can no longer be installed on Cosmetics/Helmets.\nOld Legacy Arcane Helmets will keep their Arcane installed but you can not equip the Legacy Arcane Helmet on a Warframe with 2 Arcanes already installed.\nArcanes can only be installed on your Warframe, Operator, Operator Amp, or Zaw.\nYou can install 2 Arcanes per Warframe and per Operator.\nYou can install 1 Arcane on Operator Amps and Zaws.\nYou can now trade Upgraded Arcanes!\nArcanes have received a visual overhaul! Arcane Rarity now matches the style we use on Mods and Relics. Hopefully this is a much clearer representation of their Rarity than before: \nYou can now search/sort within the Arcane Manager screen.\nYou must be at least Mastery Rank 5 or have an Arcane owned if below Mastery Rank 5 to install an Arcane.\nFixed Arcanes not listing Teralysts (+ variants) as drop locations, and vice versa. ",
      "fixes": ""
    },
    {
      "name": "Shrine of the Eidolon: Hotfix 22.13.4",
      "date": "2018-02-22T22:25:01Z",
      "url": "https://forums.warframe.com/topic/925435-shrine-of-the-eidolon-hotfix-22134/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcanes working inconsistently on Exalted Abilities (Excalibur’s Exalted Blade, etc) or Weapon Abilities (Titania’s Razorwing, etc)."
    },
    {
      "name": "Shrine of the Eidolon: Hotfix 22.13.2",
      "date": "2018-02-16T19:48:19Z",
      "url": "https://forums.warframe.com/topic/922693-shrine-of-the-eidolon-hotfix-22132/",
      "additions": "",
      "changes": "Teralyst, Gantulyst and Hydrolyst now all drop Arcanes! \nAll 3 Teralysts have a 100% chance of dropping an Arcane.\nThe variety of Arcanes have been spread out across all 3 variants and weighted according to the rarity of the Arcane and difficulty of the Teralyst variant. \nHow you choose to defeat the Teralyst variants also attributes to the Arcane type/chance.\nUpdated Arcane Resistance to read Toxin instead of Electricity and updated its icon to match the effect.",
      "fixes": ""
    },
    {
      "name": "Shrine of the Eidolon: Hotfix 22.13.1",
      "date": "2018-02-15T21:48:54Z",
      "url": "https://forums.warframe.com/topic/922278-shrine-of-the-eidolon-hotfix-22131/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Momentum and Arcane Consequence reflecting incorrect text descriptions since their recent changes in U22.12.0."
    },
    {
      "name": "Shrine of the Eidolon: Hotfix 22.12.3",
      "date": "2018-02-12T22:56:11Z",
      "url": "https://forums.warframe.com/topic/920621-shrine-of-the-eidolon-hotfix-22123/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Arcane Arachne damage increase not resetting when shooting the ground"
    },
    {
      "name": "Shrine of the Eidolon: Update 22.12.0",
      "date": "2018-02-09T23:37:46Z",
      "url": "https://forums.warframe.com/topic/918482-shrine-of-the-eidolon-update-22120/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed not being able to Chat link Operator Magus, Exodia, or Virtuos Arcanes."
    },
    {
      "name": "Plains of Eidolon: Hotfix 22.11.1",
      "date": "2018-02-06T18:58:57Z",
      "url": "https://forums.warframe.com/topic/916858-plains-of-eidolon-hotfix-22111/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Trinity Knightess and Excalibur Corpra Helmet being Arcaned. "
    },
    {
      "name": "Plains of Eidolon: Hotfix 22.2.5",
      "date": "2017-11-11T02:19:26Z",
      "url": "https://forums.warframe.com/topic/877028-plains-of-eidolon-hotfix-2225/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed a black box HUD icon appearing when using a Zaw with the Exodia Triumph Arcane upgrade. This was an unintended Channeling Efficiency upgrade on top of its intended Channeling damage."
    },
    {
      "name": "Plains of Eidolon: Update 22.1.0",
      "date": "2017-10-25T20:57:43Z",
      "url": "https://forums.warframe.com/topic/867056-plains-of-eidolon-update-2210/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed inconsistent Arcane descriptions between vendor and Arcane menu."
    },
    {
      "name": "Plains of Eidolon: Hotfix 22.0.8",
      "date": "2017-10-20T21:44:33Z",
      "url": "https://forums.warframe.com/topic/863518-plains-of-eidolon-hotfix-2208/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed a soft lock caused by equipping a Virtuos Arcane on an Amp."
    },
    {
      "name": "Plains of Eidolon: Hotfix 22.0.2",
      "date": "2017-10-13T19:57:07Z",
      "url": "https://forums.warframe.com/topic/855756-plains-of-eidolon-hotfix-2202/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed a progression stopping issue in the Stolen Dreams where you were unable to ‘Find the Arcane Machine’. "
    },
    {
      "name": "Warframe Update 22: Plains of Eidolon",
      "date": "2017-10-12T22:00:20Z",
      "url": "https://forums.warframe.com/topic/854253-warframe-update-22-plains-of-eidolon/",
      "additions": "Once you have acquired your blueprint and crafted your Cutter, enter the Plains. The ancient battle has left traces of energy on those Plains, and this affects the Landscape, including the very rock formations! As a result, you will be able to mine interesting and strange mineral deposits with interesting and strange rewards. ‘Arcane Gems’ will be available from this system. Once you find a deposit, and have your mining laser, simply trace the pattern on the rock to successfully extract your reward!",
      "changes": "",
      "fixes": "Fixed Arcane Helmets not being listed when selecting an Arcane to Normal Helmet conversion blueprint. "
    },
    {
      "name": "Hydroid Prime: Update 21.7.0",
      "date": "2017-09-07T18:44:11Z",
      "url": "https://forums.warframe.com/topic/840010-hydroid-prime-update-2170/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed various issues with Arcanes and some Conditional Mods not having proper line breaks. "
    },
    {
      "name": "Chains of Harrow: Update 21.2.0 + 21.2.0.1",
      "date": "2017-07-26T21:16:57Z",
      "url": "https://forums.warframe.com/topic/823165-chains-of-harrow-update-2120-21201/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed grammar when selecting an Arcane Helmet to convert in Foundry."
    },
    {
      "name": "Oberon Prime: Update 20.7.0",
      "date": "2017-06-07T19:08:59Z",
      "url": "https://forums.warframe.com/topic/804548-oberon-prime-update-2070/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed switching back and forth from the \"Uninstalled\" to the \"Installed\" Arcanes tab breaking the Arcane UI. "
    },
    {
      "name": "Octavia’s Anthem: Update 20.4.0",
      "date": "2017-05-04T13:58:50Z",
      "url": "https://forums.warframe.com/topic/793570-octavia%E2%80%99s-anthem-update-2040/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Arcane Aegis Shield recharge being delayed by a couple seconds instead of being instant. "
    },
    {
      "name": "The Glast Gambit: Hotfix 19.11.3",
      "date": "2017-02-17T22:37:15Z",
      "url": "https://forums.warframe.com/topic/763984-the-glast-gambit-hotfix-19113/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed opening the Main Menu while in the Arcanes Menu opening the Main Menu in the background."
    },
    {
      "name": "The Glast Gambit: Hotfix 19.6.1",
      "date": "2017-01-13T16:41:08Z",
      "url": "https://forums.warframe.com/topic/749385-the-glast-gambit-hotfix-1961/",
      "additions": "",
      "changes": "Reverted the inability to click and view Relic/Arcanes in the Codex that you do not own. You can now view them as you could prior to Update 19.6.0. ",
      "fixes": ""
    },
    {
      "name": "The Glast Gambit: Update 19.6.0 + 19.6.0.1",
      "date": "2017-01-11T23:47:27Z",
      "url": "https://forums.warframe.com/topic/748631-the-glast-gambit-update-1960-19601/",
      "additions": "",
      "changes": "Removed ability to purchase multiple Arcane Distillers from Syndicates since they are now no longer consumed on use.",
      "fixes": ""
    },
    {
      "name": "The Glast Gambit: Update 19.5",
      "date": "2016-12-22T14:13:09Z",
      "url": "https://forums.warframe.com/topic/738152-the-glast-gambit-update-195/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Ash Carabid & Banshee Echo helmets not being improved by Arcanes "
    },
    {
      "name": "The War Within: Update 19.4.2 + 19.4.2.1",
      "date": "2016-12-20T23:37:56Z",
      "url": "https://forums.warframe.com/topic/737306-the-war-within-update-1942-19421/",
      "additions": "",
      "changes": "Distilling Arcanes no longer consumes a distiller",
      "fixes": ""
    },
    {
      "name": "Hotfix 19.0.5",
      "date": "2016-11-18T16:09:29Z",
      "url": "https://forums.warframe.com/topic/721283-hotfix-1905/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcanes being listed in a random order when viewed in the Foundry.\nFixed Arcane buffs not appearing on the HUD. "
    },
    {
      "name": "Update 19: The War Within",
      "date": "2016-11-12T04:02:28Z",
      "url": "https://forums.warframe.com/topic/715768-update-19-the-war-within/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Status resist Arcanes (Arcane Resistance, Deflection, Ice, Warmth, Nullifier, and Healing) not displaying that they offer an extra Arcane Revive when maxed. \nFixed an issue where disconnecting and reconnecting during a Trial mission could result in players receiving different Arcanes."
    },
    {
      "name": "Specters of the Rail: U2.1",
      "date": "2016-08-10T17:57:04Z",
      "url": "https://forums.warframe.com/topic/685418-specters-of-the-rail-u21/",
      "additions": "",
      "changes": "The Codex section for Relics and Arcanes will now show items you've discovered but don't own (like Mods do so you can see the drop sources to farm some more).",
      "fixes": ""
    },
    {
      "name": "Specters of the Rail: Update 2",
      "date": "2016-08-03T21:43:21Z",
      "url": "https://forums.warframe.com/topic/682884-specters-of-the-rail-update-2/",
      "additions": "Relics & Arcanes now appear in the Codex in their own section, which reveal drop locations for Relics and Arcanes! Please note that as a result of this change, the Oddities section has been moved to Objects and renamed 'Kuria'.",
      "changes": "",
      "fixes": ""
    },
    {
      "name": "Update: Specters of the Rail",
      "date": "2016-07-08T14:12:48Z",
      "url": "https://forums.warframe.com/topic/668132-update-specters-of-the-rail/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed crafting an original Arcane helmet when you already have one in your inventory, consuming the BluePrint but not giving you the completed Helmet."
    },
    {
      "name": "Hotfix 18.13.2",
      "date": "2016-06-01T22:37:51Z",
      "url": "https://forums.warframe.com/topic/655270-hotfix-18132/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Ash’s Arcane Locust and Arcane Scorpion Helmet not being tradable. "
    },
    {
      "name": "Hotfix 18.13.1",
      "date": "2016-05-28T02:45:03Z",
      "url": "https://forums.warframe.com/topic/652966-hotfix-18131/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed crashing in the Arcane installment screen."
    },
    {
      "name": "Update 18.11.0 & 18.11.1",
      "date": "2016-05-11T20:30:34Z",
      "url": "https://forums.warframe.com/topic/647209-update-18110-18111/",
      "additions": "",
      "changes": "",
      "fixes": "•    Fixed being visible while Ivara’s Prowl is active with an Arcane Trickery installed. "
    },
    {
      "name": "Hotfix 18.10.5",
      "date": "2016-05-04T21:33:02Z",
      "url": "https://forums.warframe.com/topic/645092-hotfix-18105/",
      "additions": "",
      "changes": "",
      "fixes": "•    Fixed becoming permanently invisible by casting Prowl and having Arcane Trickery."
    },
    {
      "name": "Update 18.5: Sands of Inaros",
      "date": "2016-03-04T18:20:53Z",
      "url": "https://forums.warframe.com/topic/618642-update-185-sands-of-inaros-%C2%A0/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Arachne Arcane not producing a HUD icon when proccing."
    },
    {
      "name": "Hotfix 18.4.9",
      "date": "2016-02-08T22:15:39Z",
      "url": "https://forums.warframe.com/topic/607236-hotfix-1849/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an issue where Arcane enhancement descriptions were using improper text."
    },
    {
      "name": "Hotfix 18.4.6",
      "date": "2016-02-01T18:17:25Z",
      "url": "https://forums.warframe.com/topic/603669-hotfix-1846/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an error preventing multiple visual triggers on proc-based Mods/Arcanes."
    },
    {
      "name": "Update 18: The Second Dream",
      "date": "2015-12-03T23:46:40Z",
      "url": "https://forums.warframe.com/topic/568455-update-18-the-second-dream/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Bullet Jump and Aim Glide improperly causing the Arachne Arcane to proc."
    },
    {
      "name": "Hotfix 17.12.1",
      "date": "2015-11-25T22:04:24Z",
      "url": "https://forums.warframe.com/topic/564757-hotfix-17121/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Steam Tennogen items displaying an improper label when players install an Arcane."
    },
    {
      "name": "Update 17.10.0",
      "date": "2015-11-04T23:21:00Z",
      "url": "https://forums.warframe.com/topic/556384-update-17100/",
      "additions": "",
      "changes": "Reduced the required Codex scans for the Arcane Machine, Arcane Boiler, Cascade Bomb, Darvo, and Infested Mesa to one.",
      "fixes": "Fixed a duplicate entry for the Arcane Machine that would appear in the Codex."
    },
    {
      "name": "Update 17.9.1",
      "date": "2015-10-29T20:54:29Z",
      "url": "https://forums.warframe.com/topic/553319-update-1791/",
      "additions": "",
      "changes": "",
      "fixes": "New Arcanes are here! Beat The Jordas Verdict to find them all!"
    },
    {
      "name": "Update 17.9.0",
      "date": "2015-10-28T15:12:27Z",
      "url": "https://forums.warframe.com/topic/552544-update-1790/",
      "additions": "",
      "changes": "Audio FX will now play on the successful installation of an Arcane.",
      "fixes": "Fixed no items being listed under the “Select Items to Enhance” options of the Arcanes sub-menu."
    },
    {
      "name": "Hotfix 17.5.5",
      "date": "2015-10-05T22:15:15Z",
      "url": "https://forums.warframe.com/topic/539640-hotfix-1755/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an error preventing players from claiming pending Arcanes in their Foundry.  This fix works by reimbursing any pending recipes. You'll need to start the Arcane craft again after this hotfix."
    },
    {
      "name": "Hotfix 17.5.3",
      "date": "2015-10-02T16:07:50Z",
      "url": "https://forums.warframe.com/topic/536870-hotfix-1753/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Arcane UI incorrectly showing 2 Arcanes in your inventory after Distilling an Arcane."
    },
    {
      "name": "Update 17.5: The Jordas Precept. + Hotfix 17.5.1",
      "date": "2015-10-02T02:05:33Z",
      "url": "https://forums.warframe.com/topic/536207-update-175-the-jordas-precept-hotfix-1751/",
      "additions": "A new button for Arcanes has been added to the Foundry, next to the Components button.  This menu will allow players to sort through their installed and uninstalled Arcanes in a much more organized fashion.\nArcane Distiller cost reduced to 50,000 standing.\nInstalling and distilling Arcanes is now done instantaneously.",
      "changes": "",
      "fixes": "Fixed an issue preventing Arcanes from activating during Archwing Missions."
    },
    {
      "name": "Hotfix 17.4.2",
      "date": "2015-09-11T20:40:57Z",
      "url": "https://forums.warframe.com/topic/526883-hotfix-1742/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Revives not being visible in the Revive UI of Archwing Missions."
    },
    {
      "name": "Hotfix 17.3.1",
      "date": "2015-09-04T15:59:08Z",
      "url": "https://forums.warframe.com/topic/522855-hotfix-1731/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Proto Excalibur Helmet not being able to take Arcane enhancements."
    },
    {
      "name": "Update 17.2.4",
      "date": "2015-08-26T19:41:39Z",
      "url": "https://forums.warframe.com/topic/518275-update-1724/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an issue with Arcane Distillers still appearing in inventories even after being consumed."
    },
    {
      "name": "Hotfix 17.2.3",
      "date": "2015-08-24T21:01:33Z",
      "url": "https://forums.warframe.com/topic/517129-hotfix-1723/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an issue preventing Spectres and Arcane Helmets not appearing in inventory after crafting without relogging."
    },
    {
      "name": "Hotfix 17.0.4",
      "date": "2015-08-06T14:48:33Z",
      "url": "https://forums.warframe.com/topic/504855-hotfix-1704/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed players not being able to equip Arcane Enhancements Equinox’s Helmet.  This fix only corrects on newly crafted helmets -"
    },
    {
      "name": "Hotfix 17.0.3",
      "date": "2015-08-04T22:21:45Z",
      "url": "https://forums.warframe.com/topic/503583-hotfix-1703/",
      "additions": "",
      "changes": "",
      "fixes": "Unlisted U 17.0 Addition:Nightmare Trial Rewards now guarantee an Arcane, with a chance of a Resource Blueprint or other reward in addition to the guaranteed Arcane."
    },
    {
      "name": "Update 17: Echoes Of The Sentient",
      "date": "2015-07-31T15:09:25Z",
      "url": "https://forums.warframe.com/topic/498793-update-17-echoes-of-the-sentient/",
      "additions": "",
      "changes": "All Alternate Helmets that had Stamina-related effects have been given new abilities:Arcane Chorus Helmet \nArcane Flux Helmet \nArcane Storm Helmet \nArcane Gambit Helmet \nArcane Scorpion Helmet \nArcane Menticide Helmet ",
      "fixes": ""
    },
    {
      "name": "Hotfix 16.11.3",
      "date": "2015-07-10T15:47:25Z",
      "url": "https://forums.warframe.com/topic/488018-hotfix-16113/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed players being unable to put Arcanes on Ash Prime’s helmet (for real this time \nthis fix resolves anyone who had purchased Ash Prime vs crafting!) This should also fix some users with un-Arcaneable Volt Prime helmets."
    },
    {
      "name": "Hotfix 16.11.2",
      "date": "2015-07-08T15:21:23Z",
      "url": "https://forums.warframe.com/topic/486780-hotfix-16112/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Ash Prime Helmet not available for Arcane Enhancements."
    },
    {
      "name": "Update 16.10.0",
      "date": "2015-06-25T21:50:17Z",
      "url": "https://forums.warframe.com/topic/480637-update-16100/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an issue preventing player from selecting cosmetic items after choosing a cosmetic and cancelling an Arcane upgrade."
    },
    {
      "name": "Update 16.9.0",
      "date": "2015-06-17T21:50:27Z",
      "url": "https://forums.warframe.com/topic/476490-update-1690/",
      "additions": "",
      "changes": "Players will now see a quantity listed next to the Arcane Enhancements in their inventory.",
      "fixes": "Fixed players being unable to sell Arcanes."
    },
    {
      "name": "Hotfix 16.5.3",
      "date": "2015-05-13T22:21:59Z",
      "url": "https://forums.warframe.com/topic/457163-hotfix-1653/",
      "additions": "",
      "changes": "The Removal Of Arcane Enhancements On Nekros’ Mortos Syandana And Valkyr’S Bonds Is Now Live, And Any Attached Arcanes Have Been Returned To Your Inventory. As Stated In Hotfix 16.5.2, This Change Is Due To Nekros And Valkyr Having An Unfair Advantage Over Other Warframes By Having One Additional Arcane Slot.:",
      "fixes": ""
    },
    {
      "name": "Hotfix 16.5.2",
      "date": "2015-05-13T16:10:12Z",
      "url": "https://forums.warframe.com/topic/456875-hotfix-1652/",
      "additions": "",
      "changes": "The Nekros Mortos Syandana and Valkyr Bonds can no longer be Enhanced. This change is due to Nekros and Valkyr having an unfair advantage over other Warframes by having one additional Arcane slot. We're going to be removing Arcanes off these two attachments, and running a script to compensate players so the Enhancements can be used on intended Syandanas. We will notify everyone when this script has started.",
      "fixes": ""
    },
    {
      "name": "Update 16.4",
      "date": "2015-04-23T21:03:45Z",
      "url": "https://forums.warframe.com/topic/445957-update-164/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed players losing duplicate Arcane Helmets after trading one of the helmets.\nFixed Arcane HUD indicators from showing as though they were debuffs from a Dragon Key."
    },
    {
      "name": "Hotfix 16.1.3",
      "date": "2015-03-26T22:31:46Z",
      "url": "https://forums.warframe.com/topic/427542-hotfix-1613/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane enhancements showing visible in the Foundry when players are unable to craft one.\nFixed typo that would appear when selecting an Arcane."
    },
    {
      "name": "Hotfix 16.1.1",
      "date": "2015-03-24T23:35:27Z",
      "url": "https://forums.warframe.com/topic/425719-hotfix-1611/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane enhancements being set to base rank on Clients."
    },
    {
      "name": "Hotfix 16.0.4",
      "date": "2015-03-23T22:18:24Z",
      "url": "https://forums.warframe.com/topic/424657-hotfix-1604/",
      "additions": "",
      "changes": "",
      "fixes": "Arcane Helmets have been removed from PvP.\nBuilding Arcane Enhancements now costs 200k credits."
    },
    {
      "name": "Hotfix 16.0.2",
      "date": "2015-03-20T23:22:15Z",
      "url": "https://forums.warframe.com/topic/421728-hotfix-1602/",
      "additions": "",
      "changes": "",
      "fixes": "·         Fixed Arcane Enhancements appearing with Prime part in the Inventory."
    },
    {
      "name": "Hotfix 16.0.1",
      "date": "2015-03-20T16:33:47Z",
      "url": "https://forums.warframe.com/topic/421286-hotfix-1601/",
      "additions": "",
      "changes": "Added sound effect to procs trigged from Arcane enhancements.",
      "fixes": "Fixed Excalibur skins showing up as fusible items in Arcane Foundry."
    },
    {
      "name": "Warframe: Sanctuary",
      "date": "2015-03-20T02:21:54Z",
      "url": "https://forums.warframe.com/topic/420448-warframe-sanctuary/",
      "additions": "",
      "changes": "",
      "fixes": "Earn Arcane enhancements from defeating Vay Hek, each of which provide a unique bonus Warframes can unleash during combat.In order to equip an Arcane enhancement players must construct it in their Foundry under the Miscellaneous tab.\nPlayers will then need to select what Helmet or Syandana they would like the Arcane to be attached to.\nAfter construction the Arcane Helmet or Arcane Syandana will appear in their Arsenal under Attachments."
    },
    {
      "name": "Update 15.14.0",
      "date": "2015-02-11T22:44:03Z",
      "url": "https://forums.warframe.com/topic/400568-update-15140/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Boiler appearing in lower case in the Stolen Dreams Quest details."
    },
    {
      "name": "Update 15.13",
      "date": "2015-02-05T21:08:01Z",
      "url": "https://forums.warframe.com/topic/396379-update-1513/",
      "additions": "",
      "changes": "What are the Arcane Codices?\nThe Grineer have put out a capture order for a thief named Maroo – they claim she double crossed them and has sold precious Arcane Codices to the Corpus. Concerned for Maroo’s safety – and excited at the idea of a new Arcane Codex – the Lotus is requesting that you bring Maroo in.\nThink fast, Tenno! You must plan your actions quickly and accordingly in this Quest. Failure to do so may alert the enemy to your presence and the Arcane Codex data may be destroyed.\nAttempting to convert an Arcane Helmet will now display a warning message to confirm your decision.",
      "fixes": ""
    },
    {
      "name": "Hotfix 15.5.8",
      "date": "2014-12-05T23:17:49Z",
      "url": "https://forums.warframe.com/topic/359419-hotfix-1558/",
      "additions": "",
      "changes": "Fixed players not being able to trade Arcane Helmets.",
      "fixes": ""
    },
    {
      "name": "Update 15.1.0+15.1.01",
      "date": "2014-11-05T22:46:16Z",
      "url": "https://forums.warframe.com/topic/340143-update-151015101/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an issue with Arcane Helmet descriptions not showing in the Arsenal."
    },
    {
      "name": "Update 14.10.0",
      "date": "2014-10-08T22:00:00Z",
      "url": "https://forums.warframe.com/topic/321746-update-14100/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an issue with the Harlequin Mirage Helmet Blueprint incorrectly appearing as the Arcane Scorpion Ash Helmet Blueprint. Please note that we will be re-running a functioning Harlequin Helmet alert at a similar time as the broken one, but with a doubled duration."
    },
    {
      "name": "Hotfix 14.0.9",
      "date": "2014-07-26T01:03:34Z",
      "url": "https://forums.warframe.com/topic/273702-hotfix-1409/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Arcane Helmets being untradable."
    },
    {
      "name": "Hotfix 13.5.2 + 13.5.3",
      "date": "2014-05-30T23:33:48Z",
      "url": "https://forums.warframe.com/topic/239114-hotfix-1352-1353/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed an issue with the Volt Arcane Pulse helmet having an incorrect tooltip."
    },
    {
      "name": "Update 13.5.0: Edo Armor & Nami Solo",
      "date": "2014-05-28T23:05:58Z",
      "url": "https://forums.warframe.com/topic/238069-update-1350-edo-armor-nami-solo/",
      "additions": "",
      "changes": "Fixed a missing description for the Arcane Pulse Helmet.",
      "fixes": ""
    },
    {
      "name": "Hotfix 13.2.4",
      "date": "2014-05-09T19:14:10Z",
      "url": "https://forums.warframe.com/topic/229195-hotfix-1324/",
      "additions": "",
      "changes": "",
      "fixes": "• Fixed an issue with Helmet Conversion blueprints switching your currently equipped helmet from an Arcane helmet to the default helmet.\n• Fixed an issue with Helmets with Stats (Arcane) still being available in certain Market Bundles."
    }
  ],
  "tradable": true,
  "type": "Arcane",
  "uniqueName": "/Lotus/Upgrades/CosmeticEnhancers/Offensive/PermaComboOnFinisherSub"
}
"#;

        let rec: Arcane = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Upgrades/CosmeticEnhancers/Offensive/PermaComboOnFinisherSub"
        );
    }
}
