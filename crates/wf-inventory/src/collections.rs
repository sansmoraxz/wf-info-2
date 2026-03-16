use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ObjectId;

/// Represents a fusion treasure (relic) with socket info.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FusionTreasure {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemCount")]
    pub item_count: i64,

    #[serde(rename = "Sockets")]
    pub sockets: Option<i64>,
}

/// Represents a quest key with completion state.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestKey {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "Completed")]
    pub completed: Option<bool>,

    #[serde(rename = "Progress")]
    pub progress: Option<Vec<Value>>,

    pub unlock: Option<bool>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents an active booster.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Booster {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ExpiryDate")]
    pub expiry_date: Option<i64>,
}

/// Represents mastery XP info for an item.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XPInfoEntry {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "XP")]
    pub xp: i64,
}

/// Represents a Railjack salvaged weapon.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrewShipSalvagedWeapon {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: Option<String>,

    #[serde(rename = "UpgradeType")]
    pub upgrade_type: Option<String>,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "IsNew")]
    pub is_new: Option<bool>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a Railjack component skin (engines, shields, etc.).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrewShipWeaponSkin {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: Option<String>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a Kubrow/Kavat genetic imprint.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KubrowPetPrint {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "IsMale")]
    pub is_male: Option<bool>,

    #[serde(rename = "DominantTraits")]
    pub dominant_traits: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents an evolution progress entry (incarnon).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionProgress {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "Progress")]
    pub progress: Option<i64>,

    #[serde(rename = "Rank")]
    pub rank: Option<i64>,
}

/// Represents a spectral loadout.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpectreLoadout {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "Suits")]
    pub suits: Option<String>,

    #[serde(rename = "LongGuns")]
    pub long_guns: Option<String>,

    #[serde(rename = "Pistols")]
    pub pistols: Option<String>,

    #[serde(rename = "Melee")]
    pub melee: Option<String>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a challenge instance state.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChallengeInstanceState {
    pub id: Option<Value>,

    #[serde(rename = "Progress")]
    pub progress: Option<Value>,

    #[serde(rename = "IsRewardCollected")]
    pub is_reward_collected: Option<bool>,

    pub params: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a collectible series (Kuria, fragments, etc.).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectibleSeries {
    #[serde(rename = "CollectibleType")]
    pub collectible_type: Option<String>,

    #[serde(rename = "Count")]
    pub count: Option<i64>,

    #[serde(rename = "ReqScans")]
    pub req_scans: Option<i64>,

    #[serde(rename = "Tracking")]
    pub tracking: Option<Value>,

    #[serde(rename = "IncentiveStates")]
    pub incentive_states: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a completed job chain.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletedJobChain {
    #[serde(rename = "LocationTag")]
    pub location_tag: Option<String>,

    #[serde(rename = "Jobs")]
    pub jobs: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a completed job.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletedJob {
    #[serde(rename = "JobId")]
    pub job_id: Option<String>,

    #[serde(rename = "StageCompletions")]
    pub stage_completions: Option<Vec<i64>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a discovered marker.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredMarker {
    pub tag: Option<String>,

    #[serde(rename = "discoveryState")]
    pub discovery_state: Option<Vec<i64>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a Descent (The Circuit) reward state.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescentReward {
    #[serde(rename = "Seed")]
    pub seed: Option<i64>,

    #[serde(rename = "Category")]
    pub category: Option<String>,

    #[serde(rename = "FloorClaimed")]
    pub floor_claimed: Option<i64>,

    #[serde(rename = "Expiry")]
    pub expiry: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents an Endless XP (Steel Path) entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndlessXPEntry {
    #[serde(rename = "Category")]
    pub category: Option<String>,

    #[serde(rename = "Choices")]
    pub choices: Option<Vec<String>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents focus loadout preset.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FocusLoadout {
    #[serde(rename = "FocusAbility")]
    pub focus_ability: Option<String>,

    #[serde(rename = "Preset")]
    pub preset: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents hub NPC customization.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HubNpcCustomization {
    #[serde(rename = "Tag")]
    pub tag: Option<String>,

    #[serde(rename = "Colors")]
    pub colors: Option<Value>,

    #[serde(rename = "Pattern")]
    pub pattern: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a Kahl loadout.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KahlLoadOut {
    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "Skins")]
    pub skins: Option<Vec<String>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents Codex library scan progress.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryPersonalProgress {
    #[serde(rename = "TargetType")]
    pub target_type: Option<String>,

    #[serde(rename = "Scans")]
    pub scans: Option<i64>,

    #[serde(rename = "Completed")]
    pub completed: Option<bool>,
}

/// Represents a login milestone reward.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SortieRewardEntry {
    #[serde(rename = "Manifest")]
    pub manifest: Option<String>,

    #[serde(rename = "SortieId")]
    pub sortie_id: Option<Value>,

    #[serde(rename = "StoreItem")]
    pub store_item: Option<String>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a nemesis history entry (Lich/Sister).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NemesisHistory {
    #[serde(rename = "AgentIdx")]
    pub agent_idx: Option<i64>,

    #[serde(rename = "BirthNode")]
    pub birth_node: Option<String>,

    #[serde(rename = "Rank")]
    pub rank: Option<i64>,

    #[serde(rename = "KillingSuit")]
    pub killing_suit: Option<String>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a pending trade.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingTrade {
    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "Getting")]
    pub getting: Option<Value>,

    #[serde(rename = "Giving")]
    pub giving: Option<Value>,

    #[serde(rename = "ClanTax")]
    pub clan_tax: Option<i64>,

    #[serde(rename = "BuddyReady")]
    pub buddy_ready: Option<bool>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents periodic mission completion data.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeriodicMissionCompletion {
    pub tag: Option<String>,
    pub date: Option<Value>,
    pub count: Option<i64>,
}

/// Represents personal goal/event progress.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalGoalProgress {
    #[serde(rename = "Tag")]
    pub tag: Option<String>,

    #[serde(rename = "Count")]
    pub count: Option<i64>,

    #[serde(rename = "Best")]
    pub best: Option<f64>,

    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a dojo research project.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalTechProject {
    #[serde(rename = "ItemType")]
    pub item_type: Option<String>,

    #[serde(rename = "ItemId")]
    pub item_id: Option<ObjectId>,

    #[serde(rename = "ReqCredits")]
    pub req_credits: Option<i64>,

    #[serde(rename = "ReqItems")]
    pub req_items: Option<Vec<Value>>,

    #[serde(rename = "CompletionDate")]
    pub completion_date: Option<Value>,

    #[serde(rename = "HasContributions")]
    pub has_contributions: Option<bool>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a recent vendor purchase history.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentVendorPurchase {
    #[serde(rename = "VendorType")]
    pub vendor_type: Option<String>,

    #[serde(rename = "PurchaseHistory")]
    pub purchase_history: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a Nightwave season challenge history entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonChallengeHistory {
    pub id: Option<String>,
    pub challenge: Option<String>,
}

/// Represents a song challenge (Shawzin).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SongChallenge {
    #[serde(rename = "Song")]
    pub song: Option<String>,

    #[serde(rename = "Difficulties")]
    pub difficulties: Option<Vec<i64>>,
}

/// Represents sortie reward attenuation.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RewardAttenuation {
    #[serde(rename = "Tag")]
    pub tag: Option<String>,

    #[serde(rename = "Atten")]
    pub atten: Option<f64>,
}

/// Represents a step sequencer (Mandachord).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepSequencer {
    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "FingerPrint")]
    pub finger_print: Option<String>,

    #[serde(rename = "NotePacks")]
    pub note_packs: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents a taunt history entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TauntHistory {
    pub node: Option<String>,
    pub state: Option<String>,
}

/// Represents a library daily task info.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryDailyTaskInfo {
    #[serde(rename = "EnemyIcon")]
    pub enemy_icon: Option<String>,

    #[serde(rename = "EnemyLocTag")]
    pub enemy_loc_tag: Option<String>,

    #[serde(rename = "EnemyTypes")]
    pub enemy_types: Option<Vec<String>>,

    #[serde(rename = "RewardStanding")]
    pub reward_standing: Option<i64>,

    #[serde(rename = "RewardStoreItem")]
    pub reward_store_item: Option<String>,

    #[serde(rename = "RewardQuantity")]
    pub reward_quantity: Option<i64>,

    #[serde(flatten)]
    pub other: Option<Value>,
}
