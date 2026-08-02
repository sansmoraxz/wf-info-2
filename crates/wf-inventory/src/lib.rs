use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// Shared types for inventory items
pub mod common;

/// Warframe frame module
pub mod suit;

/// Warframe primary weapon module
pub mod long_gun;

/// Warframe secondary weapon module
pub mod pistol;

/// Warframe melee module
pub mod melee;

/// Warframe upgrade (mods) module
pub mod upgrades;

/// Warframe archwing module
pub mod space_suit;

/// Warframe archgun module
pub mod space_gun;

/// Warframe archmelee module
pub mod space_melee;

/// Blueprints
pub mod recipe;

/// Necramechs
pub mod mech_suit;

/// Sentinel companions
pub mod sentinel;

/// Sentinel weapons
pub mod sentinel_weapon;

/// Kubrow/Kavat pets
pub mod kubrow_pet;

/// MOA companions
pub mod moa_pet;

/// Operator Amps
pub mod operator_amp;

/// Operator suits
pub mod operator_suit;

/// Operator/Drifter loadouts
pub mod operator_loadout;

/// K-Drives
pub mod hoverboard;

/// Kaithe mounts
pub mod horse;

/// Motorcycles
pub mod motorcycle;

/// Railjack ships
pub mod crew_ship;

/// Railjack weapons
pub mod crew_ship_weapon;

/// Railjack harnesses/reactors
pub mod crew_ship_harness;

/// Crew members
pub mod crew_member;

/// Resource drones
pub mod drone;

/// Special items (exalted weapons, etc.)
pub mod special_item;

/// Landing craft
pub mod ship;

/// Drifter melee weapons
pub mod drifter_melee;

/// Data knives
pub mod data_knife;

/// Scoops
pub mod scoop;

/// Antiques (Operator weapons)
pub mod antique;

/// Simple collection and item types
pub mod collections;

/// Progress and standing data
pub mod progress;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FractionSyndicates {
    SteelMeridianSyndicate,
    ArbitersSyndicate,
    CephalonSudaSyndicate,
    PerrinSyndicate,
    RedVeilSyndicate,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Display,
    derive_more::From,
    derive_more::AsRef,
)]
#[display("{oid}")]
#[as_ref(str)]
pub struct ObjectId {
    #[serde(rename = "$oid")]
    oid: String,
}

/// An item's canonical `ItemType` type path, e.g. `/Lotus/Powersuits/...`.
#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Display,
    derive_more::From,
    derive_more::AsRef,
)]
#[serde(transparent)]
#[from(forward)]
#[as_ref(str)]
pub struct ItemType(String);

impl std::borrow::Borrow<str> for ItemType {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl PartialEq<&str> for ItemType {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Polarity {
    #[serde(rename = "Value")]
    pub value: Option<String>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateWrapper {
    #[serde(rename = "$date")]
    #[serde(deserialize_with = "deserialize_mongo_date_option")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
}

pub fn deserialize_mongo_date_option<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;
    if v.is_null() {
        return Ok(None);
    }

    if let Value::Number(n) = &v
        && let Some(ms) = n.as_i64()
    {
        if let Some(dt) = ms_to_dt(ms) {
            return Ok(Some(dt));
        } else {
            return Err(serde::de::Error::custom("invalid timestamp"));
        }
    }

    if let Value::String(s) = &v {
        if let Ok(ms) = s.parse::<i64>() {
            if let Some(dt) = ms_to_dt(ms) {
                return Ok(Some(dt));
            } else {
                return Err(serde::de::Error::custom("invalid timestamp"));
            }
        }
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(Some(dt.with_timezone(&Utc)));
        }
    }

    if let Value::Object(map) = &v {
        if let Some(Value::String(num_s)) = map.get("$numberLong")
            && let Ok(ms) = num_s.parse::<i64>()
        {
            if let Some(dt) = ms_to_dt(ms) {
                return Ok(Some(dt));
            } else {
                return Err(serde::de::Error::custom("invalid timestamp"));
            }
        }
        if let Some(Value::Number(num)) = map.get("$numberLong")
            && let Some(ms) = num.as_i64()
        {
            if let Some(dt) = ms_to_dt(ms) {
                return Ok(Some(dt));
            } else {
                return Err(serde::de::Error::custom("invalid timestamp"));
            }
        }
        if let Some(Value::String(s)) = map.get("$date") {
            if let Ok(ms) = s.parse::<i64>() {
                if let Some(dt) = ms_to_dt(ms) {
                    return Ok(Some(dt));
                } else {
                    return Err(serde::de::Error::custom("invalid timestamp"));
                }
            }
            if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                return Ok(Some(dt.with_timezone(&Utc)));
            }
        }
    }

    Err(serde::de::Error::custom("unsupported date format"))
}

fn ms_to_dt(ms: i64) -> Option<DateTime<Utc>> {
    Utc.timestamp_millis_opt(ms).single()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    // ── Equipment ──
    /// Warframes
    #[serde(rename = "Suits")]
    pub suits: Vec<suit::Suit>,

    /// Primary Weapons
    #[serde(rename = "LongGuns")]
    pub long_guns: Vec<long_gun::LongGun>,

    /// Secondary Weapons
    #[serde(rename = "Pistols")]
    pub pistols: Vec<pistol::Pistol>,

    /// Melee
    #[serde(rename = "Melee")]
    pub melee: Vec<melee::Melee>,

    /// Archwing
    #[serde(rename = "SpaceSuits")]
    pub space_suits: Vec<space_suit::SpaceSuit>,

    /// Archgun
    #[serde(rename = "SpaceGuns")]
    pub space_guns: Vec<space_gun::SpaceGun>,

    /// ArchMelee
    #[serde(rename = "SpaceMelee")]
    pub space_melee: Vec<space_melee::SpaceMelee>,

    /// Necramechs
    #[serde(rename = "MechSuits")]
    pub mech_suits: Option<Vec<mech_suit::MechSuit>>,

    /// Sentinel companions
    #[serde(rename = "Sentinels")]
    pub sentinels: Option<Vec<sentinel::Sentinel>>,

    /// Sentinel weapons
    #[serde(rename = "SentinelWeapons")]
    pub sentinel_weapons: Option<Vec<sentinel_weapon::SentinelWeapon>>,

    /// Kubrow/Kavat pets
    #[serde(rename = "KubrowPets")]
    pub kubrow_pets: Option<Vec<kubrow_pet::KubrowPet>>,

    /// MOA companions
    #[serde(rename = "MoaPets")]
    pub moa_pets: Option<Vec<moa_pet::MoaPet>>,

    /// Operator Amps
    #[serde(rename = "OperatorAmps")]
    pub operator_amps: Option<Vec<operator_amp::OperatorAmp>>,

    /// Operator suits
    #[serde(rename = "OperatorSuits")]
    pub operator_suits: Option<Vec<operator_suit::OperatorSuit>>,

    /// Operator loadouts
    #[serde(rename = "OperatorLoadOuts")]
    pub operator_load_outs: Option<Vec<operator_loadout::OperatorLoadOut>>,

    /// Adult Operator (Drifter) loadouts
    #[serde(rename = "AdultOperatorLoadOuts")]
    pub adult_operator_load_outs: Option<Vec<operator_loadout::AdultOperatorLoadOut>>,

    /// K-Drives
    #[serde(rename = "Hoverboards")]
    pub hoverboards: Option<Vec<hoverboard::Hoverboard>>,

    /// Kaithe mounts
    #[serde(rename = "Horses")]
    pub horses: Option<Vec<horse::Horse>>,

    /// Motorcycles
    #[serde(rename = "Motorcycles")]
    pub motorcycles: Option<Vec<motorcycle::Motorcycle>>,

    /// Railjack ships
    #[serde(rename = "CrewShips")]
    pub crew_ships: Option<Vec<crew_ship::CrewShip>>,

    /// Railjack weapons
    #[serde(rename = "CrewShipWeapons")]
    pub crew_ship_weapons: Option<Vec<crew_ship_weapon::CrewShipWeapon>>,

    /// Railjack harnesses/reactors
    #[serde(rename = "CrewShipHarnesses")]
    pub crew_ship_harnesses: Option<Vec<crew_ship_harness::CrewShipHarness>>,

    /// Railjack salvaged weapons
    #[serde(rename = "CrewShipSalvagedWeapons")]
    pub crew_ship_salvaged_weapons: Option<Vec<collections::CrewShipSalvagedWeapon>>,

    /// Railjack component skins (engines, shields, etc.)
    #[serde(rename = "CrewShipWeaponSkins")]
    pub crew_ship_weapon_skins: Option<Vec<collections::CrewShipWeaponSkin>>,

    /// Railjack ammo
    #[serde(rename = "CrewShipAmmo")]
    pub crew_ship_ammo: Option<Vec<common::CountableItem>>,

    /// Railjack raw salvage
    #[serde(rename = "CrewShipRawSalvage")]
    pub crew_ship_raw_salvage: Option<Vec<common::CountableItem>>,

    /// Crew members
    #[serde(rename = "CrewMembers")]
    pub crew_members: Option<Vec<crew_member::CrewMember>>,

    /// Resource drones
    #[serde(rename = "Drones")]
    pub drones: Option<Vec<drone::Drone>>,

    /// Special items (exalted weapons, etc.)
    #[serde(rename = "SpecialItems")]
    pub special_items: Option<Vec<special_item::SpecialItem>>,

    /// Landing craft
    #[serde(rename = "Ships")]
    pub ships: Option<Vec<ship::Ship>>,

    /// Drifter melee weapons
    #[serde(rename = "DrifterMelee")]
    pub drifter_melee: Option<Vec<drifter_melee::DrifterMelee>>,

    /// Data knives
    #[serde(rename = "DataKnives")]
    pub data_knives: Option<Vec<data_knife::DataKnife>>,

    /// Scoops
    #[serde(rename = "Scoops")]
    pub scoops: Option<Vec<scoop::Scoop>>,

    /// Antiques (Operator weapons)
    #[serde(rename = "Antiques")]
    pub antiques: Option<Vec<antique::Antique>>,

    // ── Mods & Upgrades ──
    /// Mods + Arcanes (unupgraded)
    #[serde(rename = "RawUpgrades")]
    pub raw_upgrades: Vec<upgrades::RawUpgrade>,

    /// Mods + Arcanes (upgraded)
    #[serde(rename = "Upgrades")]
    pub upgrades: Vec<upgrades::Upgrade>,

    // ── Blueprints ──
    /// Blueprints
    #[serde(rename = "Recipes")]
    pub recipes: Vec<recipe::Recipe>,

    /// Blueprint build in progress
    #[serde(rename = "PendingRecipes")]
    pub pending_recipes: Vec<recipe::PendingRecipe>,

    // ── Collections ──
    /// Consumable items (ciphers, restores, etc.)
    #[serde(rename = "Consumables")]
    pub consumables: Option<Vec<common::CountableItem>>,

    /// Miscellaneous items (resources, etc.)
    #[serde(rename = "MiscItems")]
    pub misc_items: Option<Vec<common::CountableItem>>,

    /// Ship decorations
    #[serde(rename = "ShipDecorations")]
    pub ship_decorations: Option<Vec<common::CountableItem>>,

    /// Level keys (derelict, etc.)
    #[serde(rename = "LevelKeys")]
    pub level_keys: Option<Vec<common::CountableItem>>,

    /// Email items
    #[serde(rename = "EmailItems")]
    pub email_items: Option<Vec<common::CountableItem>>,

    /// Flavour items (profile icons, etc.)
    #[serde(rename = "FlavourItems")]
    pub flavour_items: Option<Vec<common::TypeOnlyItem>>,

    /// Focus upgrades
    #[serde(rename = "FocusUpgrades")]
    pub focus_upgrades: Option<Vec<common::TypeOnlyItem>>,

    /// Weapon skins
    #[serde(rename = "WeaponSkins")]
    pub weapon_skins: Option<Vec<common::WeaponSkin>>,

    /// Fusion treasures (relics)
    #[serde(rename = "FusionTreasures")]
    pub fusion_treasures: Option<Vec<collections::FusionTreasure>>,

    /// Quest keys
    #[serde(rename = "QuestKeys")]
    pub quest_keys: Option<Vec<collections::QuestKey>>,

    /// Active boosters
    #[serde(rename = "Boosters")]
    pub boosters: Option<Vec<collections::Booster>>,

    /// Mastery XP info per item
    #[serde(rename = "XPInfo")]
    pub xp_info: Option<Vec<collections::XPInfoEntry>>,

    /// Kubrow/Kavat genetic imprints
    #[serde(rename = "KubrowPetPrints")]
    pub kubrow_pet_prints: Option<Vec<collections::KubrowPetPrint>>,

    /// Incarnon evolution progress
    #[serde(rename = "EvolutionProgress")]
    pub evolution_progress: Option<Vec<collections::EvolutionProgress>>,

    /// Spectre loadouts
    #[serde(rename = "SpectreLoadouts")]
    pub spectre_loadouts: Option<Vec<collections::SpectreLoadout>>,

    // ── Progress & Standing ──
    /// Syndicate/faction standings
    #[serde(rename = "Affiliations")]
    pub affiliations: Option<Vec<progress::Affiliation>>,

    /// Mission completion data
    #[serde(rename = "Missions")]
    pub missions: Option<Vec<progress::Mission>>,

    /// Challenge progress (Nightwave, etc.)
    #[serde(rename = "ChallengeProgress")]
    pub challenge_progress: Option<Vec<progress::ChallengeProgressEntry>>,

    /// Lore fragment scans
    #[serde(rename = "LoreFragmentScans")]
    pub lore_fragment_scans: Option<Vec<progress::LoreFragmentScan>>,

    // ── Scalar fields ──
    /// Player remaining trades for the day
    #[serde(rename = "TradesRemaining")]
    pub trades_remaining: Option<i64>,

    /// Syndicate
    #[serde(rename = "SupportedSyndicate")]
    pub supported_syndicates: Option<FractionSyndicates>,

    /// Regular credits balance
    #[serde(rename = "RegularCredits")]
    pub regular_credits: Option<i64>,

    /// Platinum balance
    #[serde(rename = "PremiumCredits")]
    pub premium_credits: Option<i64>,

    /// Free platinum balance
    #[serde(rename = "PremiumCreditsFree")]
    pub premium_credits_free: Option<i64>,

    /// Endo balance
    #[serde(rename = "FusionPoints")]
    pub fusion_points: Option<i64>,

    /// Mastery rank
    #[serde(rename = "PlayerLevel")]
    pub player_level: Option<i64>,

    /// Daily focus cap
    #[serde(rename = "DailyFocus")]
    pub daily_focus: Option<i64>,

    /// Focus XP per school
    #[serde(rename = "FocusXP")]
    pub focus_xp: Option<Value>,

    /// Active focus ability
    #[serde(rename = "FocusAbility")]
    pub focus_ability: Option<String>,

    /// Gifts remaining for the day
    #[serde(rename = "GiftsRemaining")]
    pub gifts_remaining: Option<i64>,

    /// Daily syndicate affiliation cap
    #[serde(rename = "DailyAffiliation")]
    pub daily_affiliation: Option<i64>,

    /// Active avatar image
    #[serde(rename = "ActiveAvatarImageType")]
    pub active_avatar_image_type: Option<String>,

    /// Currently equipped title
    #[serde(rename = "TitleType")]
    pub title_type: Option<String>,

    /// Guild (clan) ID
    #[serde(rename = "GuildId")]
    pub guild_id: Option<ObjectId>,

    /// Whether archwing is enabled
    #[serde(rename = "ArchwingEnabled")]
    pub archwing_enabled: Option<bool>,

    // ── Slot bins ──
    /// Warframe slots
    #[serde(rename = "SuitBin")]
    pub suit_bin: Option<common::SlotBin>,

    /// Weapon slots
    #[serde(rename = "WeaponBin")]
    pub weapon_bin: Option<common::SlotBin>,

    /// Sentinel slots
    #[serde(rename = "SentinelBin")]
    pub sentinel_bin: Option<common::SlotBin>,

    /// Necramech slots
    #[serde(rename = "MechBin")]
    pub mech_bin: Option<common::SlotBin>,

    /// Archwing slots
    #[serde(rename = "SpaceSuitBin")]
    pub space_suit_bin: Option<common::SlotBin>,

    /// Arch-weapon slots
    #[serde(rename = "SpaceWeaponBin")]
    pub space_weapon_bin: Option<common::SlotBin>,

    /// Operator Amp slots
    #[serde(rename = "OperatorAmpBin")]
    pub operator_amp_bin: Option<common::SlotBin>,

    /// Riven mod slots
    #[serde(rename = "RandomModBin")]
    pub random_mod_bin: Option<common::SlotBin>,

    /// Crew member slots
    #[serde(rename = "CrewMemberBin")]
    pub crew_member_bin: Option<common::SlotBin>,

    /// Railjack salvage slots
    #[serde(rename = "CrewShipSalvageBin")]
    pub crew_ship_salvage_bin: Option<common::SlotBin>,

    /// PvE loadout slots
    #[serde(rename = "PveBonusLoadoutBin")]
    pub pve_bonus_loadout_bin: Option<common::SlotBin>,

    /// PvP loadout slots
    #[serde(rename = "PvpBonusLoadoutBin")]
    pub pvp_bonus_loadout_bin: Option<common::SlotBin>,

    // ── Additional list collections ──
    /// Challenge instance states
    #[serde(rename = "ChallengeInstanceStates")]
    pub challenge_instance_states: Option<Vec<collections::ChallengeInstanceState>>,

    /// Collectible series (Kuria, fragments, etc.)
    #[serde(rename = "CollectibleSeries")]
    pub collectible_series: Option<Vec<collections::CollectibleSeries>>,

    /// Completed job chains
    #[serde(rename = "CompletedJobChains")]
    pub completed_job_chains: Option<Vec<collections::CompletedJobChain>>,

    /// Completed jobs
    #[serde(rename = "CompletedJobs")]
    pub completed_jobs: Option<Vec<collections::CompletedJob>>,

    /// Discovered markers
    #[serde(rename = "DiscoveredMarkers")]
    pub discovered_markers: Option<Vec<collections::DiscoveredMarker>>,

    /// Descent (The Circuit) rewards
    #[serde(rename = "DescentRewards")]
    pub descent_rewards: Option<Vec<collections::DescentReward>>,

    /// Endless XP (Steel Path) entries
    #[serde(rename = "EndlessXP")]
    pub endless_xp: Option<Vec<collections::EndlessXPEntry>>,

    /// Focus loadout presets
    #[serde(rename = "FocusLoadouts")]
    pub focus_loadouts: Option<Vec<collections::FocusLoadout>>,

    /// Hub NPC customizations
    #[serde(rename = "HubNpcCustomizations")]
    pub hub_npc_customizations: Option<Vec<collections::HubNpcCustomization>>,

    /// Kahl loadouts
    #[serde(rename = "KahlLoadOuts")]
    pub kahl_load_outs: Option<Vec<collections::KahlLoadOut>>,

    /// Codex library personal progress
    #[serde(rename = "LibraryPersonalProgress")]
    pub library_personal_progress: Option<Vec<collections::LibraryPersonalProgress>>,

    /// Last sortie reward
    #[serde(rename = "LastSortieReward")]
    pub last_sortie_reward: Option<Vec<collections::SortieRewardEntry>>,

    /// Last lite sortie reward
    #[serde(rename = "LastLiteSortieReward")]
    pub last_lite_sortie_reward: Option<Vec<collections::SortieRewardEntry>>,

    /// Nemesis (Lich/Sister) history
    #[serde(rename = "NemesisHistory")]
    pub nemesis_history: Option<Vec<collections::NemesisHistory>>,

    /// Pending trades
    #[serde(rename = "PendingTrades")]
    pub pending_trades: Option<Vec<collections::PendingTrade>>,

    /// Periodic mission completions
    #[serde(rename = "PeriodicMissionCompletions")]
    pub periodic_mission_completions: Option<Vec<collections::PeriodicMissionCompletion>>,

    /// Personal goal progress (events)
    #[serde(rename = "PersonalGoalProgress")]
    pub personal_goal_progress: Option<Vec<collections::PersonalGoalProgress>>,

    /// Personal tech projects (dojo research)
    #[serde(rename = "PersonalTechProjects")]
    pub personal_tech_projects: Option<Vec<collections::PersonalTechProject>>,

    /// Recent vendor purchases
    #[serde(rename = "RecentVendorPurchases")]
    pub recent_vendor_purchases: Option<Vec<collections::RecentVendorPurchase>>,

    /// Nightwave season challenge history
    #[serde(rename = "SeasonChallengeHistory")]
    pub season_challenge_history: Option<Vec<collections::SeasonChallengeHistory>>,

    /// Shawzin song challenges
    #[serde(rename = "SongChallenges")]
    pub song_challenges: Option<Vec<collections::SongChallenge>>,

    /// Sortie reward attenuation
    #[serde(rename = "SortieRewardAttenuation")]
    pub sortie_reward_attenuation: Option<Vec<collections::RewardAttenuation>>,

    /// Special item reward attenuation
    #[serde(rename = "SpecialItemRewardAttenuation")]
    pub special_item_reward_attenuation: Option<Vec<collections::RewardAttenuation>>,

    /// Mandachord step sequencers
    #[serde(rename = "StepSequencers")]
    pub step_sequencers: Option<Vec<collections::StepSequencer>>,

    /// Taunt history
    #[serde(rename = "TauntHistory")]
    pub taunt_history: Option<Vec<collections::TauntHistory>>,

    /// Railjack salvaged weapon skins
    #[serde(rename = "CrewShipSalvagedWeaponSkins")]
    pub crew_ship_salvaged_weapon_skins: Option<Vec<Value>>,

    /// Pending spectre loadouts
    #[serde(rename = "PendingSpectreLoadouts")]
    pub pending_spectre_loadouts: Option<Vec<Value>>,

    /// Nemesis abandoned rewards
    #[serde(rename = "NemesisAbandonedRewards")]
    pub nemesis_abandoned_rewards: Option<Vec<Value>>,

    /// Qualifying invasions
    #[serde(rename = "QualifyingInvasions")]
    pub qualifying_invasions: Option<Vec<Value>>,

    /// Echoes hex conquest active frame variants
    #[serde(rename = "EchoesHexConquestActiveFrameVariants")]
    pub echoes_hex_conquest_active_frame_variants: Option<Vec<String>>,

    /// Echoes hex conquest active stickers
    #[serde(rename = "EchoesHexConquestActiveStickers")]
    pub echoes_hex_conquest_active_stickers: Option<Vec<Value>>,

    /// Echoes hex conquest bonus tokens given
    #[serde(rename = "EchoesHexConquestBonusTokensGiven")]
    pub echoes_hex_conquest_bonus_tokens_given: Option<Vec<i64>>,

    /// Entratilab conquest active frame variants
    #[serde(rename = "EntratiLabConquestActiveFrameVariants")]
    pub entrati_lab_conquest_active_frame_variants: Option<Vec<String>>,

    // ── Simple string/id lists ──
    /// Completed alerts
    #[serde(rename = "CompletedAlerts")]
    pub completed_alerts: Option<Vec<String>>,

    /// Completed sorties
    #[serde(rename = "CompletedSorties")]
    pub completed_sorties: Option<Vec<String>>,

    /// Completed syndicates
    #[serde(rename = "CompletedSyndicates")]
    pub completed_syndicates: Option<Vec<String>>,

    /// Claimed junction challenge rewards
    #[serde(rename = "ClaimedJunctionChallengeRewards")]
    pub claimed_junction_challenge_rewards: Option<Vec<String>>,

    /// Current loadout IDs
    #[serde(rename = "CurrentLoadOutIds")]
    pub current_load_out_ids: Option<Vec<ObjectId>>,

    /// Death marks (assassin triggers)
    #[serde(rename = "DeathMarks")]
    pub death_marks: Option<Vec<String>>,

    /// Equipped emotes
    #[serde(rename = "EquippedEmotes")]
    pub equipped_emotes: Option<Vec<String>>,

    /// Equipped gear wheel items
    #[serde(rename = "EquippedGear")]
    pub equipped_gear: Option<Vec<String>>,

    /// Faction scores
    #[serde(rename = "FactionScores")]
    pub faction_scores: Option<Vec<i64>>,

    /// Login milestone rewards
    #[serde(rename = "LoginMilestoneRewards")]
    pub login_milestone_rewards: Option<Vec<String>>,

    /// Node intros completed
    #[serde(rename = "NodeIntrosCompleted")]
    pub node_intros_completed: Option<Vec<String>>,

    /// One-time purchases
    #[serde(rename = "OneTimePurchases")]
    pub one_time_purchases: Option<Vec<String>>,

    // ── Additional scalar fields ──
    /// Active dojo color research
    #[serde(rename = "ActiveDojoColorResearch")]
    pub active_dojo_color_research: Option<String>,

    /// Bounty score
    #[serde(rename = "BountyScore")]
    pub bounty_score: Option<i64>,

    /// Challenges fix version
    #[serde(rename = "ChallengesFixVersion")]
    pub challenges_fix_version: Option<i64>,

    /// Death squadable flag
    #[serde(rename = "DeathSquadable")]
    pub death_squadable: Option<bool>,

    /// Echoes hex conquest cache score mission
    #[serde(rename = "EchoesHexConquestCacheScoreMission")]
    pub echoes_hex_conquest_cache_score_mission: Option<i64>,

    /// Echoes hex conquest hard mode status
    #[serde(rename = "EchoesHexConquestHardModeStatus")]
    pub echoes_hex_conquest_hard_mode_status: Option<i64>,

    /// Echoes hex conquest unlocked
    #[serde(rename = "EchoesHexConquestUnlocked")]
    pub echoes_hex_conquest_unlocked: Option<i64>,

    /// EntratiLab conquest cache score mission
    #[serde(rename = "EntratiLabConquestCacheScoreMission")]
    pub entrati_lab_conquest_cache_score_mission: Option<i64>,

    /// EntratiLab conquest hard mode status
    #[serde(rename = "EntratiLabConquestHardModeStatus")]
    pub entrati_lab_conquest_hard_mode_status: Option<i64>,

    /// EntratiLab conquest unlocked
    #[serde(rename = "EntratiLabConquestUnlocked")]
    pub entrati_lab_conquest_unlocked: Option<i64>,

    /// Entrati vault count last period
    #[serde(rename = "EntratiVaultCountLastPeriod")]
    pub entrati_vault_count_last_period: Option<i64>,

    /// HWID protect enabled
    #[serde(rename = "HWIDProtectEnabled")]
    pub hwid_protect_enabled: Option<bool>,

    /// Handler points
    #[serde(rename = "HandlerPoints")]
    pub handler_points: Option<i64>,

    /// Harvestable flag
    #[serde(rename = "Harvestable")]
    pub harvestable: Option<bool>,

    /// Has owned void projections previously
    #[serde(rename = "HasOwnedVoidProjectionsPreviously")]
    pub has_owned_void_projections_previously: Option<bool>,

    /// Has received email item
    #[serde(rename = "HasRecievedEmailItem")]
    pub has_recieved_email_item: Option<bool>,

    /// Has reset account
    #[serde(rename = "HasResetAccount")]
    pub has_reset_account: Option<bool>,

    /// Last region played
    #[serde(rename = "LastRegionPlayed")]
    pub last_region_played: Option<String>,

    /// Library personal target
    #[serde(rename = "LibraryPersonalTarget")]
    pub library_personal_target: Option<String>,

    /// Operator customization slot purchases
    #[serde(rename = "OperatorCustomizationSlotPurchases")]
    pub operator_customization_slot_purchases: Option<i64>,

    /// Played parkour tutorial
    #[serde(rename = "PlayedParkourTutorial")]
    pub played_parkour_tutorial: Option<bool>,

    /// Prime tokens (Aya)
    #[serde(rename = "PrimeTokens")]
    pub prime_tokens: Option<i64>,

    /// Received starting gear
    #[serde(rename = "ReceivedStartingGear")]
    pub received_starting_gear: Option<bool>,

    /// Reward seed
    #[serde(rename = "RewardSeed")]
    pub reward_seed: Option<i64>,

    /// Story mode choice
    #[serde(rename = "StoryModeChoice")]
    pub story_mode_choice: Option<String>,

    /// Subscribed to emails
    #[serde(rename = "SubscribedToEmails")]
    pub subscribed_to_emails: Option<i64>,

    /// Subscribed to personalized emails
    #[serde(rename = "SubscribedToEmailsPersonalized")]
    pub subscribed_to_emails_personalized: Option<i64>,

    /// Theme background
    #[serde(rename = "ThemeBackground")]
    pub theme_background: Option<String>,

    /// Theme style
    #[serde(rename = "ThemeStyle")]
    pub theme_style: Option<String>,

    /// Use adult operator loadout
    #[serde(rename = "UseAdultOperatorLoadout")]
    pub use_adult_operator_loadout: Option<bool>,

    // ── Daily affiliation caps ──
    #[serde(rename = "DailyAffiliationCavia")]
    pub daily_affiliation_cavia: Option<i64>,

    #[serde(rename = "DailyAffiliationCetus")]
    pub daily_affiliation_cetus: Option<i64>,

    #[serde(rename = "DailyAffiliationEntrati")]
    pub daily_affiliation_entrati: Option<i64>,

    #[serde(rename = "DailyAffiliationHex")]
    pub daily_affiliation_hex: Option<i64>,

    #[serde(rename = "DailyAffiliationKahl")]
    pub daily_affiliation_kahl: Option<i64>,

    #[serde(rename = "DailyAffiliationLibrary")]
    pub daily_affiliation_library: Option<i64>,

    #[serde(rename = "DailyAffiliationNecraloid")]
    pub daily_affiliation_necraloid: Option<i64>,

    #[serde(rename = "DailyAffiliationPvp")]
    pub daily_affiliation_pvp: Option<i64>,

    #[serde(rename = "DailyAffiliationQuills")]
    pub daily_affiliation_quills: Option<i64>,

    #[serde(rename = "DailyAffiliationSolaris")]
    pub daily_affiliation_solaris: Option<i64>,

    #[serde(rename = "DailyAffiliationVentkids")]
    pub daily_affiliation_ventkids: Option<i64>,

    #[serde(rename = "DailyAffiliationVox")]
    pub daily_affiliation_vox: Option<i64>,

    #[serde(rename = "DailyAffiliationZariman")]
    pub daily_affiliation_zariman: Option<i64>,

    // ── Date fields ──
    /// Account creation date
    #[serde(rename = "Created")]
    pub created: Option<DateWrapper>,

    /// Blessing cooldown
    #[serde(rename = "BlessingCooldown")]
    pub blessing_cooldown: Option<DateWrapper>,

    /// Entrati vault count reset date
    #[serde(rename = "EntratiVaultCountResetDate")]
    pub entrati_vault_count_reset_date: Option<DateWrapper>,

    /// Last nemesis ally spawn time
    #[serde(rename = "LastNemesisAllySpawnTime")]
    pub last_nemesis_ally_spawn_time: Option<DateWrapper>,

    /// Next daily refill date
    #[serde(rename = "NextRefill")]
    pub next_refill: Option<DateWrapper>,

    /// Training date
    #[serde(rename = "TrainingDate")]
    pub training_date: Option<DateWrapper>,

    // ── Complex dict fields ──
    /// Alignment (morality system)
    #[serde(rename = "Alignment")]
    pub alignment: Option<Value>,

    /// Calendar progress
    #[serde(rename = "CalendarProgress")]
    pub calendar_progress: Option<Value>,

    /// Dialogue history
    #[serde(rename = "DialogueHistory")]
    pub dialogue_history: Option<Value>,

    /// Duviri info
    #[serde(rename = "DuviriInfo")]
    pub duviri_info: Option<Value>,

    /// Infested foundry (Helminth)
    #[serde(rename = "InfestedFoundry")]
    pub infested_foundry: Option<Value>,

    /// Last inventory sync
    #[serde(rename = "LastInventorySync")]
    pub last_inventory_sync: Option<ObjectId>,

    /// Library active daily task info
    #[serde(rename = "LibraryActiveDailyTaskInfo")]
    pub library_active_daily_task_info: Option<collections::LibraryDailyTaskInfo>,

    /// Library available daily task info
    #[serde(rename = "LibraryAvailableDailyTaskInfo")]
    pub library_available_daily_task_info: Option<collections::LibraryDailyTaskInfo>,

    /// Loadout presets
    #[serde(rename = "LoadOutPresets")]
    pub load_out_presets: Option<Value>,

    /// Lotus customization (Ordis/ship)
    #[serde(rename = "LotusCustomization")]
    pub lotus_customization: Option<Value>,

    /// Mailbox
    #[serde(rename = "Mailbox")]
    pub mailbox: Option<Value>,

    /// Active nemesis (Lich/Sister)
    #[serde(rename = "Nemesis")]
    pub nemesis: Option<Value>,

    /// Pending coupon
    #[serde(rename = "PendingCoupon")]
    pub pending_coupon: Option<Value>,

    /// Player skills (intrinsics, etc.)
    #[serde(rename = "PlayerSkills")]
    pub player_skills: Option<Value>,

    /// Sentient spawn chance boosters
    #[serde(rename = "SentientSpawnChanceBoosters")]
    pub sentient_spawn_chance_boosters: Option<Value>,

    /// Settings (friend invite, gift mode, etc.)
    #[serde(rename = "Settings")]
    pub settings: Option<Value>,

    /// Web flags
    #[serde(rename = "WebFlags")]
    pub web_flags: Option<Value>,

    // ── Catch-all for remaining fields ──
    #[serde(flatten)]
    pub other: Option<Value>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::from_str;

    pub fn load_test_inventory() -> Inventory {
        let inventory_str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/sample_inventory.json"
        ));
        from_str(inventory_str).unwrap()
    }

    #[test]
    fn test_inventory_deserialize() {
        let inventory: Inventory = load_test_inventory();
        assert!(!inventory.suits.is_empty(), "Suits should not be empty");
        assert!(
            !inventory.long_guns.is_empty(),
            "Long guns should not be empty"
        );
        assert!(!inventory.pistols.is_empty(), "Pistols should not be empty");
        assert!(
            !inventory.raw_upgrades.is_empty(),
            "Upgrades should not be empty"
        );
    }
}
