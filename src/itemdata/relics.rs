use serde::{Deserialize, Serialize};

use serde_json::Value;

pub type Root = Vec<Relic>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    pub category: String,
    pub description: String,
    pub image_name: String,
    pub locations: Vec<Value>, // observed to be empty array
    pub masterable: bool,
    pub name: String,
    pub rewards: Vec<Value>, // observed to be empty array
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub exclude_from_codex: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drop {
    pub chance: f64,
    pub location: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patchlog {
    pub name: String,
    pub date: String,
    pub url: String,
    pub additions: String,
    pub changes: String,
    pub fixes: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_relic() {
        let json_data = r#"
{
  "category": "Relics",
  "description": "An artifact containing Orokin secrets. It can only be opened through the power of the Void.",
  "drops": [
    {
      "chance": 0.0029,
      "location": "Void/Mithra (Interception), Rotation C",
      "rarity": "Legendary",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0029,
      "location": "Void/Mot (Survival), Rotation C",
      "rarity": "Legendary",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0042,
      "location": "Earth/Cetus (Level 15 - 25 Plague Star), Rotation A",
      "rarity": "Legendary",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0058,
      "location": "Earth/Cetus (Level 15 - 25 Plague Star), Rotation A",
      "rarity": "Legendary",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0067,
      "location": "Earth/Cetus (Level 15 - 25 Plague Star), Rotation A",
      "rarity": "Legendary",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0114,
      "location": "Earth/Cetus (Level 15 - 25 Plague Star), Rotation A",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0201,
      "location": "Elite Sanctuary Onslaught, Rotation C",
      "rarity": "Rare",
      "type": "Axi P8 Relic (Radiant)"
    },
    {
      "chance": 0.0241,
      "location": "Veil/Calabash (Skirmish)",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0241,
      "location": "Veil/Numina (Skirmish)",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.025,
      "location": "Void Storm (Veil Proxima)",
      "rarity": "Rare",
      "type": "Axi P8 Relic (Radiant)"
    },
    {
      "chance": 0.0282,
      "location": "Void/Belenus (Defense), Rotation C",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0429,
      "location": "Veil/Arc Silver (Skirmish), Rotation A",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0429,
      "location": "Veil/Lu-Yan (Skirmish), Rotation A",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0476,
      "location": "Veil/Erato (Skirmish), Rotation A",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.05,
      "location": "Jupiter/Ganymede (Disruption), Rotation C",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0612,
      "location": "Lua/Circulus (Survival), Rotation C",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0625,
      "location": "Void/Aten (Mobile Defense)",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0625,
      "location": "Void/Marduk (Sabotage)",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Eris/Ixodes (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Eris/Kala-Azar (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Eris/Oestrus (Infested Salvage), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Kuva Fortress/Nabuk (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Lua/StöFler (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Neptune/Proteus (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Pluto/Outer Terminus (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Sedna/Hydron (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Uranus/Bianca (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Uranus/Miranda (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0667,
      "location": "Uranus/Stephano (Defense), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.079,
      "location": "Mars/Tyana Pass (Defense), Rotation C",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.08,
      "location": "Kuva Fortress/Taveuni (Survival), Rotation C",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0818,
      "location": "Höllvania/Solstice Square (Defense), Rotation C",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0833,
      "location": "Faceoff: Single Squad (Steel Path), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0833,
      "location": "Faceoff: Single Squad, Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0833,
      "location": "Faceoff: Squad VS Squad (Steel Path), Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0833,
      "location": "Faceoff: Squad VS Squad, Rotation B",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.0968,
      "location": "Uranus/Caelus (Interception), Rotation C",
      "rarity": "Rare",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1,
      "location": "Deimos/Armatus (Disruption), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1,
      "location": "Neptune/Yursa (Defection), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1,
      "location": "Veil/Sabmir Cloud (Skirmish), Rotation B",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.102,
      "location": "Sedna/Kappa (Disruption), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1084,
      "location": "Void/Mithra (Interception), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1084,
      "location": "Void/Mot (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Deimos/Terrorem (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Eris/Hymeno (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Eris/Ixodes (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Eris/Kala-Azar (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Eris/Nimus (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Europa/Cholistan (Excavation), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Kuva Fortress/Nabuk (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Lua/StöFler (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Lua/Tycho (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Neptune/Despina (Excavation), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Neptune/Kelashin (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Neptune/Proteus (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Pluto/Hieracon (Excavation), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Pluto/Outer Terminus (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Pluto/Palus (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Sedna/Amarna (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Sedna/Hydron (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Sedna/Scylla (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Sedna/Selkie (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Sedna/Yemaja (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Uranus/Assur (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Uranus/Bianca (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Uranus/Cupid (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Uranus/Miranda (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Uranus/Ophelia (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1106,
      "location": "Uranus/Stephano (Defense), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1136,
      "location": "Lua/Yuvarium (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1242,
      "location": "Lua/Apollo (Disruption), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.125,
      "location": "Eris/Zabala (Survival), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Eris/Phalan (Interception), Rotation B",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Eris/Phalan (Interception), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Eris/Sporid (Interception), Rotation B",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Eris/Sporid (Interception), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Eris/Xini (Interception), Rotation B",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Eris/Xini (Interception), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Lua/Apollo (Disruption), Rotation B",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Pluto/Cerberus (Interception), Rotation B",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Pluto/Cerberus (Interception), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Sedna/Berehynia (Interception), Rotation B",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    },
    {
      "chance": 0.1429,
      "location": "Sedna/Berehynia (Interception), Rotation C",
      "rarity": "Uncommon",
      "type": "Axi P8 Relic"
    }
  ],
  "imageName": "axi-exceptional.png",
  "locations": [],
  "masterable": false,
  "name": "Axi P8 Exceptional",
  "rewards": [],
  "tradable": true,
  "type": "Relic",
  "uniqueName": "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeASilver"
}
"#;

        let rec: Relic = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeASilver"
        );
    }
}
