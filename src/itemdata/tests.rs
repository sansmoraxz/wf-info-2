use std::collections::HashMap;

use crate::itemdata::enums::VaultStatus;
use crate::itemdata::traits::{
    Buildable, Character, Droppable, Equippable, HasAbilities, Item, MeleeWeapon, Prime,
    RangedWeapon, Weapon, WikiaLinked,
};
use crate::{inventory, itemdata};

// ── Helpers ──

macro_rules! load_json {
    ($file:literal) => {{
        // Try local dev copy first, then cached web download
        let local = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/warframe-items-data/json/",
            $file
        );
        if std::path::Path::new(local).exists() {
            std::fs::read_to_string(local).unwrap()
        } else if let Ok(cached) = crate::control::item_data_fetch::cached_path($file) {
            if cached.exists() {
                std::fs::read_to_string(cached).unwrap()
            } else {
                panic!(
                    "Item data file '{}' not found locally or in cache. \
                     Run the daemon once or download warframe-items-data.",
                    $file
                );
            }
        } else {
            panic!("Could not determine cache path for '{}'", $file);
        }
    }};
}

fn find_by_name<'a, T: Item>(items: &'a [T], name: &str) -> &'a T {
    items
        .iter()
        .find(|i| i.name() == name)
        .unwrap_or_else(|| panic!("item '{}' not found", name))
}

// ── Warframes ──

#[test]
fn test_warframes_deserialize_all() {
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_warframes_excalibur_fields() {
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();
    let excal = arr.iter().find(|w| w.name() == "Excalibur").unwrap();

    assert_eq!(excal.unique_name(), "/Lotus/Powersuits/Excalibur/Excalibur");
    assert_eq!(excal.category(), "Warframes");
    assert_eq!(excal.type_field(), "Warframe");
    assert!(!excal.tradable());
    assert!(excal.masterable());

    // Character stats
    assert_eq!(excal.health(), 270);
    assert_eq!(excal.shield(), 270);
    assert_eq!(excal.armor(), 240);
    assert_eq!(excal.power(), 100);
    assert!((excal.sprint_speed().unwrap() - 1.0).abs() < f64::EPSILON);

    // Abilities
    assert_eq!(excal.abilities().len(), 4);
    assert_eq!(excal.abilities()[0].name, "Slash Dash");
    assert_eq!(excal.abilities()[3].name, "Exalted Blade");

    // Buildable
    assert_eq!(excal.build_price(), Some(25000));
    assert_eq!(excal.mastery_req(), Some(0));

    // Not prime
    assert!(!excal.is_prime());
    assert_eq!(excal.vault_status(), VaultStatus::NotPrime);

    // Wikia
    assert!(excal.wikia_url().is_some());
    assert!(excal.introduced().is_some());
}

#[test]
fn test_warframes_ash_prime_vaulted() {
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();
    let ash = arr.iter().find(|w| w.name() == "Ash Prime").unwrap();

    assert_eq!(ash.unique_name(), "/Lotus/Powersuits/Ninja/AshPrime");
    assert!(ash.is_prime());
    assert_eq!(ash.vaulted(), Some(true));
    assert_eq!(ash.vault_date(), Some("2017-05-30"));
    assert!(matches!(ash.vault_status(), VaultStatus::Vaulted { .. }));
    assert!(!ash.is_accessible());

    assert_eq!(ash.health(), 455);
    assert_eq!(ash.armor(), 185);
}

#[test]
fn test_warframes_necramech_bonewidow() {
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();
    let bw = arr.iter().find(|w| w.name() == "Bonewidow").unwrap();

    assert_eq!(
        bw.unique_name(),
        "/Lotus/Powersuits/EntratiMech/ThanoTech"
    );
    assert_eq!(bw.category(), "Warframes");
    assert_eq!(bw.health(), 1880);
    assert_eq!(bw.shield(), 430);
    assert_eq!(bw.armor(), 480);
    assert_eq!(bw.power(), 175);
    assert_eq!(bw.abilities().len(), 4);
    assert_eq!(bw.mastery_req(), Some(0));

    match bw {
        itemdata::warframe::WarframeEntry::MechSuits(_) => {}
        _ => panic!("Expected MechSuits variant"),
    }
}

#[test]
fn test_warframes_helminth() {
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();
    let helminth = arr.iter().find(|w| w.name() == "Helminth").unwrap();

    assert_eq!(
        helminth.unique_name(),
        "/Lotus/Powersuits/PowersuitAbilities/Helminth"
    );
    assert_eq!(helminth.health(), 0);
    assert_eq!(helminth.armor(), 0);
    assert!(!helminth.tradable());
    assert_eq!(helminth.abilities().len(), 13);
    assert!(helminth.build_price().is_none());

    match helminth {
        itemdata::warframe::WarframeEntry::Helminth(_) => {}
        _ => panic!("Expected Helminth variant"),
    }
}

// ── Primary ──

#[test]
fn test_primary_deserialize_all() {
    let raw = load_json!("Primary.json");
    let arr: itemdata::primary::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_primary_braton_fields() {
    let raw = load_json!("Primary.json");
    let arr: itemdata::primary::Root = serde_json::from_str(&raw).unwrap();
    let braton = find_by_name(&arr, "Braton");

    assert_eq!(braton.unique_name(), "/Lotus/Weapons/Tenno/Rifle/Rifle");
    assert_eq!(braton.type_field(), "Rifle");
    assert!(!braton.tradable());
    assert_eq!(braton.mastery_req(), Some(0));

    // Weapon stats
    assert!((braton.critical_chance() - 0.12).abs() < 0.01);
    assert!((braton.critical_multiplier() - 1.6).abs() < 0.01);
    assert!((braton.fire_rate() - 8.75).abs() < 0.01);
    assert!((braton.total_damage() - 24.0).abs() < 0.01);
    assert_eq!(braton.disposition(), Some(5));
    assert_eq!(braton.damage_per_shot().len(), 20);

    // Ranged weapon stats
    assert!((braton.accuracy() - 28.571428).abs() < 0.01);
    assert_eq!(braton.multishot(), 1);
    assert_eq!(braton.noise(), "Alarming");
    assert_eq!(braton.trigger(), "Auto");
    assert_eq!(braton.magazine_size(), Some(45));
    assert!((braton.reload_time() - 2.0).abs() < 0.01);

    // Equippable
    assert_eq!(braton.slot(), Some(&itemdata::Slot::Primary));

    // Not prime
    assert!(!braton.is_prime());
}

#[test]
fn test_primary_soma_prime_vaulted() {
    let raw = load_json!("Primary.json");
    let arr: itemdata::primary::Root = serde_json::from_str(&raw).unwrap();
    let soma = find_by_name(&arr, "Soma Prime");

    assert_eq!(
        soma.unique_name(),
        "/Lotus/Weapons/Tenno/LongGuns/PrimeSoma/PrimeSomaRifle"
    );
    assert!(soma.is_prime());
    assert_eq!(soma.vaulted(), Some(true));
    assert!(matches!(soma.vault_status(), VaultStatus::Vaulted { .. }));

    assert!((soma.critical_chance() - 0.30).abs() < 0.01);
    assert!((soma.fire_rate() - 15.0).abs() < 0.1);
    assert_eq!(soma.magazine_size(), Some(200));
    assert_eq!(soma.disposition(), Some(3));
}

// ── Secondary ──

#[test]
fn test_secondary_deserialize_all() {
    let raw = load_json!("Secondary.json");
    let arr: itemdata::secondary::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 50);
}

#[test]
fn test_secondary_lex_fields() {
    let raw = load_json!("Secondary.json");
    let arr: itemdata::secondary::Root = serde_json::from_str(&raw).unwrap();
    let lex = find_by_name(&arr, "Lex");

    assert_eq!(lex.unique_name(), "/Lotus/Weapons/Tenno/Pistol/HeavyPistol");
    assert_eq!(lex.type_field(), "Pistol");
    assert!((lex.critical_chance() - 0.2).abs() < 0.01);
    assert!((lex.total_damage() - 130.0).abs() < 0.01);
    assert_eq!(lex.magazine_size(), Some(6));
    assert!((lex.accuracy() - 16.0).abs() < 0.01);
    assert_eq!(lex.noise(), "Alarming");
    assert_eq!(lex.trigger(), "Semi");
    assert_eq!(lex.disposition(), Some(4));
    assert!(!lex.is_prime());
    assert_eq!(lex.slot(), Some(&itemdata::Slot::Secondary));
}

// ── Melee ──

#[test]
fn test_melee_deserialize_all() {
    let raw = load_json!("Melee.json");
    let arr: itemdata::melee::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_melee_skana_fields() {
    let raw = load_json!("Melee.json");
    let arr: itemdata::melee::Root = serde_json::from_str(&raw).unwrap();
    let skana = find_by_name(&arr, "Skana");

    assert_eq!(
        skana.unique_name(),
        "/Lotus/Weapons/Tenno/Melee/LongSword/LongSword"
    );
    assert_eq!(skana.type_field(), "Melee");

    // Weapon stats
    assert!((skana.critical_chance() - 0.05).abs() < 0.01);
    assert!((skana.total_damage() - 120.0).abs() < 0.01);
    assert_eq!(skana.disposition(), Some(4));
    assert_eq!(skana.damage_per_shot().len(), 20);

    // Melee-specific stats
    assert_eq!(skana.blocking_angle(), Some(55));
    assert_eq!(skana.combo_duration(), Some(5));
    assert!((skana.follow_through().unwrap() - 0.6).abs() < 0.01);
    assert!((skana.range().unwrap() - 2.5).abs() < 0.01);
    assert_eq!(skana.slam_attack(), Some(360));
    assert_eq!(skana.heavy_attack_damage(), Some(600));
    assert_eq!(skana.stance_polarity(), Some("madurai"));

    // Equippable
    assert_eq!(skana.slot(), Some(&itemdata::Slot::Melee));
    assert!(!skana.is_prime());
}

// ── Archwing ──

#[test]
fn test_archwing_deserialize_all() {
    let raw = load_json!("Archwing.json");
    let arr: itemdata::archwing::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_archwing_sample_fields() {
    let raw = load_json!("Archwing.json");
    let arr: itemdata::archwing::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    // Every archwing should have basic Item fields
    assert!(!item.unique_name().is_empty());
    assert!(!item.name().is_empty());
    assert_eq!(item.category(), "Archwing");

    // Character stats should be non-negative
    assert!(item.health() > 0);
    assert!(item.armor() >= 0);
    assert!(item.shield() >= 0);

    // Archwings have abilities
    assert!(!item.abilities().is_empty());
}

// ── Arch-Gun ──

#[test]
fn test_archgun_deserialize_all() {
    let raw = load_json!("Arch-Gun.json");
    let arr: itemdata::arch_gun::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_archgun_sample_fields() {
    let raw = load_json!("Arch-Gun.json");
    let arr: itemdata::arch_gun::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    assert!(!item.unique_name().is_empty());
    assert_eq!(item.category(), "Arch-Gun");
    assert!(item.total_damage() > 0.0);
    assert!(item.fire_rate() > 0.0);
    assert_eq!(item.damage_per_shot().len(), 20);

    // Ranged weapon
    assert!(item.magazine_size().is_some());
    assert!(item.reload_time() > 0.0);
}

// ── Arch-Melee ──

#[test]
fn test_archmelee_deserialize_all() {
    let raw = load_json!("Arch-Melee.json");
    let arr: itemdata::arch_melee::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_archmelee_sample_fields() {
    let raw = load_json!("Arch-Melee.json");
    let arr: itemdata::arch_melee::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    assert!(!item.unique_name().is_empty());
    assert_eq!(item.category(), "Arch-Melee");
    assert!(item.total_damage() > 0.0);
    assert_eq!(item.damage_per_shot().len(), 20);

    // Melee weapon stats
    assert!(item.blocking_angle().is_some());
    assert!(item.combo_duration().is_some());
}

// ── Arcanes ──

#[test]
fn test_arcanes_deserialize_all() {
    let raw = load_json!("Arcanes.json");
    let arr: itemdata::arcane::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 50);
}

#[test]
fn test_arcanes_energize_fields() {
    let raw = load_json!("Arcanes.json");
    let arr: itemdata::arcane::Root = serde_json::from_str(&raw).unwrap();
    let energize = find_by_name(&arr, "Arcane Energize");

    assert_eq!(
        energize.unique_name(),
        "/Lotus/Upgrades/CosmeticEnhancers/Utility/GolemArcaneRadialEnergyOnEnergyPickup"
    );
    assert_eq!(energize.category(), "Arcanes");
    assert_eq!(energize.type_field(), "Warframe Arcane");
    assert!(energize.tradable());
    assert_eq!(energize.rarity, Some(itemdata::Rarity::Legendary));
    assert_eq!(energize.level_stats.len(), 6);
}

// ── Mods ──

#[test]
fn test_mods_deserialize_all() {
    let raw = load_json!("Mods.json");
    let arr: itemdata::mods::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 1000);
}

#[test]
fn test_mods_serration_fields() {
    let raw = load_json!("Mods.json");
    let arr: itemdata::mods::Root = serde_json::from_str(&raw).unwrap();
    let serration = find_by_name(&arr, "Serration");

    assert_eq!(
        serration.unique_name(),
        "/Lotus/Upgrades/Mods/Rifle/Beginner/WeaponDamageAmountModBeginner"
    );
    assert_eq!(serration.type_field(), "Primary Mod");
    assert!(serration.tradable());
    assert_eq!(serration.rarity, Some(itemdata::Rarity::Uncommon));
    assert_eq!(serration.polarity, Some(itemdata::Polarity::Madurai));
    assert_eq!(serration.base_drain, Some(2));
    assert_eq!(serration.fusion_limit, Some(3));
    assert_eq!(serration.compat_name, Some("Rifle".to_string()));
    assert_eq!(serration.mod_set, None);
    assert!(serration.has_drops());
}

#[test]
fn test_mods_set_mod_fields() {
    let raw = load_json!("Mods.json");
    let arr: itemdata::mods::Root = serde_json::from_str(&raw).unwrap();
    let vig = find_by_name(&arr, "Vigilante Offense");

    assert!(vig.mod_set.is_some());
    assert!(matches!(
        vig.mod_category(),
        itemdata::ModCategory::SetMember { .. }
    ));
}

// ── Pets ──

#[test]
fn test_pets_deserialize_all() {
    let raw = load_json!("Pets.json");
    let arr: itemdata::pet::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 10);
}

#[test]
fn test_pets_variant_discrimination() {
    let raw = load_json!("Pets.json");
    let arr: itemdata::pet::Root = serde_json::from_str(&raw).unwrap();

    // Should contain multiple variants
    let mut has_kubrow = false;
    let mut has_component = false;
    for pet in &arr {
        match pet {
            itemdata::pet::PetEntry::KubrowPets(_) => has_kubrow = true,
            itemdata::pet::PetEntry::Pistols(_) => has_component = true,
            _ => {}
        }
    }
    assert!(has_kubrow, "Should have KubrowPets entries");
    assert!(has_component, "Should have PetComponent entries");
}

// ── Sentinels ──

#[test]
fn test_sentinels_deserialize_all() {
    let raw = load_json!("Sentinels.json");
    let arr: itemdata::sentinel::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_sentinels_carrier_fields() {
    let raw = load_json!("Sentinels.json");
    let arr: itemdata::sentinel::Root = serde_json::from_str(&raw).unwrap();
    let carrier = find_by_name(&arr, "Carrier");

    assert_eq!(
        carrier.unique_name(),
        "/Lotus/Types/Sentinels/SentinelPowersuits/CarrierPowerSuit"
    );
    assert_eq!(carrier.category(), "Sentinels");
    assert!(carrier.health() > 0);
    assert!(carrier.armor() > 0);
    assert!(carrier.shield() > 0);
    assert!(carrier.build_price().is_some());
    assert!(!carrier.components().is_empty());
}

// ── Sentinel Weapons ──

#[test]
fn test_sentinel_weapons_deserialize_all() {
    let raw = load_json!("SentinelWeapons.json");
    let arr: itemdata::sentinel_weapon::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_sentinel_weapons_sample_fields() {
    let raw = load_json!("SentinelWeapons.json");
    let arr: itemdata::sentinel_weapon::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    assert!(!item.unique_name().is_empty());
    // Note: SentinelWeapons JSON uses "Primary" as category despite being sentinel weapons
    assert!(!item.category().is_empty());
    assert!(item.total_damage() >= 0.0);
    assert_eq!(item.damage_per_shot().len(), 20);
}

// ── Gear ──

#[test]
fn test_gear_deserialize_all() {
    let raw = load_json!("Gear.json");
    let arr: itemdata::gear::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_gear_sample_fields() {
    let raw = load_json!("Gear.json");
    let arr: itemdata::gear::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    assert!(!item.unique_name().is_empty());
    assert_eq!(item.category(), "Gear");
    assert!(!item.type_field().is_empty());
}

// ── Misc ──

#[test]
fn test_misc_deserialize_all() {
    let raw = load_json!("Misc.json");
    let arr: itemdata::misc::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

// ── Relics ──

#[test]
fn test_relics_deserialize_all() {
    let raw = load_json!("Relics.json");
    let arr: itemdata::relics::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_relics_sample_fields() {
    let raw = load_json!("Relics.json");
    let arr: itemdata::relics::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    assert!(!item.unique_name().is_empty());
    assert_eq!(item.category(), "Relics");
}

// ── Resources ──

#[test]
fn test_resources_deserialize_all() {
    let raw = load_json!("Resources.json");
    let arr: itemdata::resource::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_resources_sample_fields() {
    let raw = load_json!("Resources.json");
    let arr: itemdata::resource::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    assert!(!item.unique_name().is_empty());
    assert_eq!(item.category(), "Resources");
    assert!(!item.type_field().is_empty());
}

// ── Fish ──

#[test]
fn test_fish_deserialize_all() {
    let raw = load_json!("Fish.json");
    let arr: itemdata::fish::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_fish_sample_fields() {
    let raw = load_json!("Fish.json");
    let arr: itemdata::fish::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    assert!(!item.unique_name().is_empty());
    assert_eq!(item.category(), "Fish");
    assert_eq!(item.type_field(), "Fish");
}

// ── Glyphs ──

#[test]
fn test_glyphs_deserialize_all() {
    let raw = load_json!("Glyphs.json");
    let arr: itemdata::glyph::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

// ── Sigils ──

#[test]
fn test_sigils_deserialize_all() {
    let raw = load_json!("Sigils.json");
    let arr: itemdata::sigil::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

// ── Skins ──

#[test]
fn test_skins_deserialize_all() {
    let raw = load_json!("Skins.json");
    let arr: itemdata::skin::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

// ── Quests ──

#[test]
fn test_quests_deserialize_all() {
    let raw = load_json!("Quests.json");
    let arr: itemdata::quest::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_quests_sample_fields() {
    let raw = load_json!("Quests.json");
    let arr: itemdata::quest::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    assert!(!item.unique_name().is_empty());
    assert_eq!(item.category(), "Quests");
}

// ── Nodes ──

#[test]
fn test_nodes_deserialize_all() {
    let raw = load_json!("Node.json");
    let arr: itemdata::node::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_nodes_sample_fields() {
    let raw = load_json!("Node.json");
    let arr: itemdata::node::Root = serde_json::from_str(&raw).unwrap();

    let adaro = find_by_name(&arr, "Adaro");
    assert_eq!(adaro.unique_name(), "SolNode181");
    assert_eq!(adaro.system_name, "Sedna");
    assert_eq!(adaro.category(), "Node");
}

// ── Enemies ──

#[test]
fn test_enemies_deserialize_all() {
    let raw = load_json!("Enemy.json");
    let arr: itemdata::enemy::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_enemies_sample_fields() {
    let raw = load_json!("Enemy.json");
    let arr: itemdata::enemy::Root = serde_json::from_str(&raw).unwrap();

    let bombard = find_by_name(&arr, "Corrupted Bombard");
    assert_eq!(
        bombard.unique_name(),
        "/Lotus/Types/Enemies/Orokin/OrokinRocketBombardAvatar"
    );
    assert!((bombard.health - 300.0).abs() < f64::EPSILON);
    assert!((bombard.armor - 500.0).abs() < f64::EPSILON);
    assert!((bombard.shield - 0.0).abs() < f64::EPSILON);
    assert_eq!(bombard.resistances.len(), 3);
    assert!(bombard.has_drops());
}

// ── Railjack ──

#[test]
fn test_railjack_deserialize_all() {
    let raw = load_json!("Railjack.json");
    let arr: itemdata::railjack::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 50);
}

#[test]
fn test_railjack_sample_fields() {
    let raw = load_json!("Railjack.json");
    let arr: itemdata::railjack::Root = serde_json::from_str(&raw).unwrap();
    let item = &arr[0];

    assert!(!item.unique_name().is_empty());
    assert_eq!(item.category(), "Railjack");
    assert!(item.total_damage() >= 0.0);
    assert!(item.fire_rate() > 0.0);
}

// ── Cross-cutting trait invariants ──

#[test]
fn test_all_warframes_have_category() {
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();
    for wf in &arr {
        assert_eq!(wf.category(), "Warframes");
        assert!(!wf.unique_name().is_empty());
        assert!(!wf.name().is_empty());
    }
}

#[test]
fn test_all_primaries_have_20_damage_values() {
    let raw = load_json!("Primary.json");
    let arr: itemdata::primary::Root = serde_json::from_str(&raw).unwrap();
    for item in &arr {
        assert_eq!(
            item.damage_per_shot().len(),
            20,
            "Primary '{}' should have 20 damage values",
            item.name()
        );
    }
}

#[test]
fn test_all_secondaries_have_20_damage_values() {
    let raw = load_json!("Secondary.json");
    let arr: itemdata::secondary::Root = serde_json::from_str(&raw).unwrap();
    for item in &arr {
        assert_eq!(
            item.damage_per_shot().len(),
            20,
            "Secondary '{}' should have 20 damage values",
            item.name()
        );
    }
}

#[test]
fn test_all_melees_have_20_damage_values() {
    let raw = load_json!("Melee.json");
    let arr: itemdata::melee::Root = serde_json::from_str(&raw).unwrap();
    for item in &arr {
        assert_eq!(
            item.damage_per_shot().len(),
            20,
            "Melee '{}' should have 20 damage values",
            item.name()
        );
    }
}

#[test]
fn test_prime_warframes_have_vault_status() {
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();
    for wf in &arr {
        if wf.is_prime() {
            let status = wf.vault_status();
            assert!(
                matches!(
                    status,
                    VaultStatus::Vaulted { .. }
                        | VaultStatus::Active
                        | VaultStatus::EstimatedVault { .. }
                ),
                "Prime warframe '{}' has unexpected vault status {:?}",
                wf.name(),
                status
            );
        }
    }
}

// ── Inventory mapping (existing) ──

#[test]
fn test_map_warframe_inventory() {
    use crate::inventory::tests::load_test_inventory;
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();

    let info_idx: HashMap<String, itemdata::warframe::WarframeEntry> = arr
        .into_iter()
        .map(|item| {
            let key = item.unique_name().to_string();
            (key, item)
        })
        .collect();

    let inventory: inventory::Inventory = load_test_inventory();
    let inv_index = inventory
        .suits
        .iter()
        .map(|frame| (frame.item_type.clone(), frame.clone()))
        .collect::<HashMap<_, _>>();

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Data {
        info: itemdata::warframe::WarframeEntry,
        inventory: Option<inventory::suit::Suit>,
    }

    let data: Vec<Data> = info_idx
        .iter()
        .map(
            |(key, info): (&String, &itemdata::warframe::WarframeEntry)| {
                let inv_data = inv_index.get(key).cloned();
                Data {
                    info: info.clone(),
                    inventory: inv_data,
                }
            },
        )
        .collect();

    assert!(!data.is_empty());
}
