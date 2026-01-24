use serde::{Deserialize, Serialize};

pub mod melee;
pub mod primary;
pub mod secondary;
pub mod warframe;

pub mod arch_gun;
pub mod arch_melee;
pub mod archwing;

pub mod arcane;
pub mod mods;

pub type UniqueName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Named {
    pub name: String,
    #[serde(rename = "uniqueName")]
    pub unique_name: UniqueName,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimalItem {
    #[serde(flatten)]
    pub named: Named,

    #[serde(rename = "type")]
    pub type_: String, // TODO: extract downward category wise
    pub category: Option<Category>, // TODO: downcast

    pub tradable: bool,
    #[serde(rename = "excludeFromCodex")]
    pub exclude_from_codex: Option<bool>,
    pub masterable: bool,

    #[serde(rename = "imageName")]
    pub image_name: Option<String>,

    #[serde(rename = "patchlogs")]
    pub patch_log: Option<Vec<PatchLog>>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Droppable {
    pub rarity: Option<Rarity>,
    pub drops: Option<Vec<DropChance>>,
    pub probability: Option<f32>,
    pub tradable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseItem {
    #[serde(flatten)]
    pub minimal: MinimalItem,

    #[serde(flatten)]
    pub drop: Droppable,

    #[serde(rename = "showInInventory")]
    pub show_in_inventory: Option<bool>,
    pub conclave: Option<bool>,
    #[serde(rename = "productCategory")]
    pub product_category: Option<ProductCategory>,
    #[serde(rename = "rewardName")]
    pub reward_name: Option<String>,
    #[serde(rename = "primeOmegaAttenuation")]
    pub prime_omega_attenuation: Option<f32>,
    #[serde(rename = "isPrime")]
    pub is_prime: Option<bool>,
    pub vaulted: Option<bool>,
    #[serde(rename = "vaultDate")]
    pub vault_date: Option<String>, // TODO: repr as chrono date
    #[serde(rename = "estimatedVaultDate")]
    pub estimated_vault_date: Option<String>,
    pub sentinel: Option<bool>,
    pub parents: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equippable {
    pub polarities: Option<Vec<Polarity>>,
    pub slot: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buildable {
    #[serde(rename = "masteryReq")]
    pub mastery_req: Option<u8>,
    #[serde(rename = "buildPrice")]
    pub build_price: Option<u32>,
    #[serde(rename = "buildQuantity")]
    pub build_quantity: Option<u32>,
    #[serde(rename = "buildTime")]
    pub build_time: Option<u32>,
    #[serde(rename = "skipBuildTimePrice")]
    pub skip_build_time_price: Option<u32>,
    #[serde(rename = "consumeOnBuild")]
    pub consume_on_build: Option<bool>,
    #[serde(rename = "components")]
    pub components: Option<Vec<Component>>,
    #[serde(rename = "marketCost")]
    pub market_cost: Option<u32>,
    #[serde(rename = "bpCost")]
    pub bp_cost: Option<u32>,
    #[serde(rename = "itemCount")]
    pub item_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiaItem {
    #[serde(rename = "wikiaThumbnail")]
    pub wikia_thumbnail: Option<String>,
    #[serde(rename = "wikiaUrl")]
    pub wikia_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub introduced: Option<Update>,
    #[serde(rename = "wikiAvailable")]
    pub wiki_available: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    #[serde(flatten)]
    pub named: Named,

    #[serde(rename = "itemCount")]
    pub item_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Category {
    All,
    Arcanes,
    Archwing,
    Fish,
    Gear,
    Glyphs,
    Misc,
    Mods,
    Node,
    Pets,
    Quests,
    Relics,
    Resources,
    Sentinels,
    Sigils,
    Skins,
    Warframes,
    Weapon(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductCategory {
    #[serde(rename = "CrewShipWeapons")]
    CrewShipWeapons,
    #[serde(rename = "KubrowPets")]
    KubrowPets,
    #[serde(rename = "LongGuns")]
    LongGuns,
    #[serde(rename = "MechSuits")]
    MechSuits,
    #[serde(rename = "Melee")]
    Melee,
    #[serde(rename = "OperatorAmps")]
    OperatorAmps,
    #[serde(rename = "Pistols")]
    Pistols,
    #[serde(rename = "SentinelWeapons")]
    SentinelWeapons,
    #[serde(rename = "Sentinels")]
    Sentinels,
    #[serde(rename = "SpaceGuns")]
    SpaceGuns,
    #[serde(rename = "SpaceMelee")]
    SpaceMelee,
    #[serde(rename = "SpaceSuits")]
    SpaceSuits,
    #[serde(rename = "SpecialItems")]
    SpecialItems,
    #[serde(rename = "Suits")]
    Suits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(u8)]

pub enum Disposition {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Noise {
    #[serde(rename = "Alarming")]
    Alarming,
    #[serde(rename = "Silent")]
    Silent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Polarity {
    #[serde(rename = "aura")]
    Aura,
    #[serde(rename = "madurai")]
    Madurai,
    #[serde(rename = "naramon")]
    Naramon,
    #[serde(rename = "penjaga")]
    Penjaga,
    #[serde(rename = "umbra")]
    Umbra,
    #[serde(rename = "unairu")]
    Unairu,
    #[serde(rename = "universal")]
    Universal,
    #[serde(rename = "vazarin")]
    Vazarin,
    #[serde(rename = "zenurik")]
    Zenurik,
    #[serde(rename = "any")]
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trigger {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Auto")]
    Auto,
    #[serde(rename = "Auto Burst")]
    AutoBurst,
    #[serde(rename = "Burst")]
    Burst,
    #[serde(rename = "Charge")]
    Charge,
    #[serde(rename = "Duplex")]
    Duplex,
    #[serde(rename = "Held")]
    Held,
    #[serde(rename = "Melee")]
    Melee,
    #[serde(rename = "Semi")]
    Semi,
    #[serde(rename = "")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    #[serde(rename = "Common")]
    Common,
    #[serde(rename = "Uncommon")]
    Uncommon,
    #[serde(rename = "Rare")]
    Rare,
    #[serde(rename = "Legendary")]
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct PatchLog {
    pub name: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub url: String,
    #[serde(rename = "imgUrl")]
    pub img_url: Option<String>,
    pub additions: String,
    pub changes: String,
    pub fixes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Update {
    name: String,
    url: String,
    aliases: Vec<String>,
    parent: String,
    date: String, // TODO: to chrono deserialize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropChance {
    pub chance: Option<f32>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelStats {
    pub stats: Vec<String>,
}

#[rustfmt::skip]
// #[cfg(feature = "test_with_wf_items")]
#[cfg(test)]
mod tests;
