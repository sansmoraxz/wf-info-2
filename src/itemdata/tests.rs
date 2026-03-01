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
fn test_warframes_tabular_fields() {
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, health, armor, shield, power, abilities, is_prime, vaulted)
    let cases: &[(&str, &str, i64, i64, i64, i64, usize, bool, Option<bool>)] = &[
        (
            "Excalibur",
            "/Lotus/Powersuits/Excalibur/Excalibur",
            270, 240, 270, 100, 4, false, None,
        ),
        (
            "Ash Prime",
            "/Lotus/Powersuits/Ninja/AshPrime",
            455, 185, 365, 100, 4, true, Some(true),
        ),
        (
            "Bonewidow",
            "/Lotus/Powersuits/EntratiMech/ThanoTech",
            1880, 480, 430, 175, 4, false, None,
        ),
        (
            "Helminth",
            "/Lotus/Powersuits/PowersuitAbilities/Helminth",
            0, 0, 0, 0, 13, false, None,
        ),
    ];

    for &(name, unique, health, armor, shield, power, abilities, prime, vaulted) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Warframes", "{name} category");
        assert_eq!(item.health(), health, "{name} health");
        assert_eq!(item.armor(), armor, "{name} armor");
        assert_eq!(item.shield(), shield, "{name} shield");
        assert_eq!(item.power(), power, "{name} power");
        assert_eq!(item.abilities().len(), abilities, "{name} abilities");
        assert_eq!(item.is_prime(), prime, "{name} prime");
        assert_eq!(item.vaulted(), vaulted, "{name} vaulted");
    }
}

#[test]
fn test_warframes_variant_discrimination() {
    let raw = load_json!("Warframes.json");
    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();

    // Suits variant
    let excal = find_by_name(&arr, "Excalibur");
    assert!(matches!(excal, itemdata::warframe::WarframeEntry::Suits(_)));
    assert_eq!(excal.build_price(), Some(25000));
    assert!(excal.wikia_url().is_some());
    assert_eq!(excal.vault_status(), VaultStatus::NotPrime);

    // Suits prime variant
    let ash = find_by_name(&arr, "Ash Prime");
    assert!(matches!(ash, itemdata::warframe::WarframeEntry::Suits(_)));
    assert_eq!(ash.vault_date(), Some("2017-05-30"));
    assert!(matches!(ash.vault_status(), VaultStatus::Vaulted { .. }));
    assert!(!ash.is_accessible());

    // MechSuits variant
    let bw = find_by_name(&arr, "Bonewidow");
    assert!(matches!(bw, itemdata::warframe::WarframeEntry::MechSuits(_)));
    assert_eq!(bw.mastery_req(), Some(0));

    // Helminth variant
    let helminth = find_by_name(&arr, "Helminth");
    assert!(matches!(helminth, itemdata::warframe::WarframeEntry::Helminth(_)));
    assert!(helminth.build_price().is_none());
    assert!(!helminth.tradable());
}

// ── Primary ──

#[test]
fn test_primary_deserialize_all() {
    let raw = load_json!("Primary.json");
    let arr: itemdata::primary::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_primary_tabular_fields() {
    let raw = load_json!("Primary.json");
    let arr: itemdata::primary::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, type, crit_chance, total_damage, mag_size, trigger, disposition, is_prime, vaulted)
    let cases: &[(&str, &str, &str, f64, f64, i64, &str, i64, bool, Option<bool>)] = &[
        (
            "Braton",
            "/Lotus/Weapons/Tenno/Rifle/Rifle",
            "Rifle",
            0.12, 24.0, 45, "Auto", 5, false, None,
        ),
        (
            "Soma Prime",
            "/Lotus/Weapons/Tenno/LongGuns/PrimeSoma/PrimeSomaRifle",
            "Rifle",
            0.30, 12.0, 200, "Auto", 3, true, Some(true),
        ),
        (
            "Acceltra",
            "/Lotus/Weapons/Tenno/LongGuns/SapientPrimary/SapientPrimaryWeapon",
            "Rifle",
            0.32, 70.0, 48, "Auto", 1, false, None,
        ),
    ];

    for &(name, unique, typ, crit, dmg, mag, trigger, dispo, prime, vaulted) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.type_field(), typ, "{name} type");
        assert!((item.critical_chance() - crit).abs() < 0.01, "{name} crit");
        assert!((item.total_damage() - dmg).abs() < 0.5, "{name} dmg");
        assert_eq!(item.magazine_size(), Some(mag), "{name} mag_size");
        assert_eq!(item.trigger(), trigger, "{name} trigger");
        assert_eq!(item.disposition(), Some(dispo), "{name} disposition");
        assert_eq!(item.is_prime(), prime, "{name} prime");
        assert_eq!(item.vaulted(), vaulted, "{name} vaulted");
        assert_eq!(item.damage_per_shot().len(), 20, "{name} damage_per_shot");
    }
}

// ── Secondary ──

#[test]
fn test_secondary_deserialize_all() {
    let raw = load_json!("Secondary.json");
    let arr: itemdata::secondary::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 50);
}

#[test]
fn test_secondary_tabular_fields() {
    let raw = load_json!("Secondary.json");
    let arr: itemdata::secondary::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, type, crit_chance, total_damage, mag_size, trigger, disposition, is_prime, vaulted)
    let cases: &[(&str, &str, &str, f64, f64, i64, &str, i64, bool, Option<bool>)] = &[
        (
            "Lex",
            "/Lotus/Weapons/Tenno/Pistol/HeavyPistol",
            "Pistol",
            0.2, 130.0, 6, "Semi", 4, false, None,
        ),
        (
            "Furis",
            "/Lotus/Weapons/Tenno/Pistol/AutoPistol",
            "Pistol",
            0.05, 20.0, 35, "Auto", 5, false, None,
        ),
        (
            "Lex Prime",
            "/Lotus/Weapons/Tenno/Pistols/PrimeLex/PrimeLex",
            "Pistol",
            0.25, 180.0, 8, "Semi", 4, true, Some(false),
        ),
        (
            "Akstiletto Prime",
            "/Lotus/Weapons/Tenno/Pistols/PrimeAkstiletto/PrimeAkstiletto",
            "Pistol",
            0.15, 36.0, 40, "Auto", 2, true, Some(true),
        ),
    ];

    for &(name, unique, typ, crit, dmg, mag, trigger, dispo, prime, vaulted) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.type_field(), typ, "{name} type");
        assert!((item.critical_chance() - crit).abs() < 0.01, "{name} crit");
        assert!((item.total_damage() - dmg).abs() < 0.5, "{name} dmg");
        assert_eq!(item.magazine_size(), Some(mag), "{name} mag");
        assert_eq!(item.trigger(), trigger, "{name} trigger");
        assert_eq!(item.disposition(), Some(dispo), "{name} dispo");
        assert_eq!(item.is_prime(), prime, "{name} prime");
        assert_eq!(item.vaulted(), vaulted, "{name} vaulted");
        assert_eq!(item.slot(), Some(&itemdata::Slot::Secondary), "{name} slot");
    }
}

// ── Melee ──

#[test]
fn test_melee_deserialize_all() {
    let raw = load_json!("Melee.json");
    let arr: itemdata::melee::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_melee_tabular_fields() {
    let raw = load_json!("Melee.json");
    let arr: itemdata::melee::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, crit_chance, total_damage, disposition, is_prime, blocking_angle)
    let cases: &[(&str, &str, f64, f64, i64, bool, i64)] = &[
        (
            "Skana",
            "/Lotus/Weapons/Tenno/Melee/LongSword/LongSword",
            0.05, 120.0, 4, false, 55,
        ),
        (
            "Nikana Prime",
            "/Lotus/Weapons/Tenno/Melee/Swords/PrimeKatana/PrimeNikana",
            0.28, 198.0, 1, true, 55,
        ),
        (
            "Gram",
            "/Lotus/Weapons/Tenno/Melee/GreatSword/GreatSword",
            0.15, 160.0, 5, false, 55,
        ),
    ];

    for &(name, unique, crit, dmg, dispo, prime, block_angle) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert!((item.critical_chance() - crit).abs() < 0.01, "{name} crit");
        assert!((item.total_damage() - dmg).abs() < 0.5, "{name} dmg");
        assert_eq!(item.disposition(), Some(dispo), "{name} dispo");
        assert_eq!(item.is_prime(), prime, "{name} prime");
        assert_eq!(item.blocking_angle(), Some(block_angle), "{name} blocking_angle");
        assert_eq!(item.damage_per_shot().len(), 20, "{name} damage_per_shot len");
        assert_eq!(item.slot(), Some(&itemdata::Slot::Melee), "{name} slot");
    }
}

// ── Archwing ──

#[test]
fn test_archwing_deserialize_all() {
    let raw = load_json!("Archwing.json");
    let arr: itemdata::archwing::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_archwing_tabular_fields() {
    let raw = load_json!("Archwing.json");
    let arr: itemdata::archwing::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, health, armor, shield, abilities_count, is_prime, build_price)
    let cases: &[(&str, &str, i64, i64, i64, usize, bool, i64)] = &[
        (
            "Amesha",
            "/Lotus/Powersuits/Archwing/SupportJetPack/SupportJetPack",
            650, 195, 220, 4, false, 25000,
        ),
        (
            "Odonata",
            "/Lotus/Powersuits/Archwing/StandardJetPack/StandardJetPack",
            425, 100, 430, 4, false, 7000,
        ),
        (
            "Odonata Prime",
            "/Lotus/Powersuits/Archwing/PrimeJetPack/PrimeJetPack",
            650, 100, 640, 4, true, 25000,
        ),
    ];

    for &(name, unique, health, armor, shield, abil_count, prime, bp) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Archwing", "{name} category");
        assert_eq!(item.health(), health, "{name} health");
        assert_eq!(item.armor(), armor, "{name} armor");
        assert_eq!(item.shield(), shield, "{name} shield");
        assert_eq!(item.abilities().len(), abil_count, "{name} abilities");
        assert_eq!(item.is_prime(), prime, "{name} prime");
        assert_eq!(item.build_price(), Some(bp), "{name} build_price");
    }
}

// ── Arch-Gun ──

#[test]
fn test_archgun_deserialize_all() {
    let raw = load_json!("Arch-Gun.json");
    let arr: itemdata::arch_gun::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_archgun_tabular_fields() {
    let raw = load_json!("Arch-Gun.json");
    let arr: itemdata::arch_gun::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, total_damage, fire_rate, crit_chance, mag_size, trigger, dispo, is_prime)
    let cases: &[(&str, &str, f64, f64, f64, i64, &str, i64, bool)] = &[
        (
            "Cortege",
            "/Lotus/Weapons/Tenno/Archwing/Primary/ThanoTechArchGun/ThanoTechArchGun",
            90.0, 12.0, 0.2, 100, "Held", 3, false,
        ),
        (
            "Corvas",
            "/Lotus/Weapons/Tenno/Archwing/Primary/LaunchGrenade/ArchCannon",
            880.0, 2.0, 0.4, 25, "Charge", 4, false,
        ),
        (
            "Corvas Prime",
            "/Lotus/Weapons/Tenno/Archwing/Primary/PrimeCorvas/PrimeCorvasWeapon",
            960.0, 2.0, 0.44, 20, "Charge", 3, true,
        ),
    ];

    for &(name, unique, dmg, rate, crit, mag, trigger, dispo, prime) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Arch-Gun", "{name} category");
        assert!((item.total_damage() - dmg).abs() < 1.0, "{name} dmg");
        assert!((item.fire_rate() - rate).abs() < 0.1, "{name} rate");
        assert!((item.critical_chance() - crit).abs() < 0.01, "{name} crit");
        assert_eq!(item.magazine_size(), Some(mag), "{name} mag");
        assert_eq!(item.trigger(), trigger, "{name} trigger");
        assert_eq!(item.disposition(), Some(dispo), "{name} dispo");
        assert_eq!(item.is_prime(), prime, "{name} prime");
        assert_eq!(item.damage_per_shot().len(), 20, "{name} dps len");
    }
}

// ── Arch-Melee ──

#[test]
fn test_archmelee_deserialize_all() {
    let raw = load_json!("Arch-Melee.json");
    let arr: itemdata::arch_melee::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_archmelee_tabular_fields() {
    let raw = load_json!("Arch-Melee.json");
    let arr: itemdata::arch_melee::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, total_damage, crit_chance, blocking_angle, slam_attack)
    let cases: &[(&str, &str, f64, f64, i64, i64)] = &[
        (
            "Agkuza",
            "/Lotus/Weapons/Tenno/Archwing/Melee/ArchSwordHook/ArchHookSwordWeapon",
            436.0, 0.05, 90, 436,
        ),
        (
            "Centaur",
            "/Lotus/Weapons/Tenno/Archwing/Melee/Archswordandshield/ArchSwordShield",
            376.0, 0.25, 90, 376,
        ),
        (
            "Kaszas",
            "/Lotus/Weapons/Tenno/Archwing/Melee/ArchScythe/ArchScythe",
            392.0, 0.15, 90, 392,
        ),
    ];

    for &(name, unique, dmg, crit, block_angle, slam) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Arch-Melee", "{name} category");
        assert!((item.total_damage() - dmg).abs() < 1.0, "{name} dmg");
        assert!((item.critical_chance() - crit).abs() < 0.01, "{name} crit");
        assert_eq!(item.blocking_angle(), Some(block_angle), "{name} blocking");
        assert_eq!(item.slam_attack(), Some(slam), "{name} slam");
        assert_eq!(item.combo_duration(), Some(5), "{name} combo_dur");
        assert_eq!(item.damage_per_shot().len(), 20, "{name} dps len");
    }
}

// ── Arcanes ──

#[test]
fn test_arcanes_deserialize_all() {
    let raw = load_json!("Arcanes.json");
    let arr: itemdata::arcane::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 50);
}

#[test]
fn test_arcanes_tabular_fields() {
    let raw = load_json!("Arcanes.json");
    let arr: itemdata::arcane::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, type, rarity, tradable, level_stats_count)
    let cases: &[(&str, &str, &str, itemdata::Rarity, bool, usize)] = &[
        (
            "Arcane Energize",
            "/Lotus/Upgrades/CosmeticEnhancers/Utility/GolemArcaneRadialEnergyOnEnergyPickup",
            "Warframe Arcane",
            itemdata::Rarity::Legendary,
            true,
            6,
        ),
        (
            "Arcane Agility",
            "/Lotus/Upgrades/CosmeticEnhancers/Defensive/SpeedOnDamage",
            "Warframe Arcane",
            itemdata::Rarity::Uncommon,
            true,
            6,
        ),
        (
            "Akimbo Slip Shot",
            "/Lotus/Upgrades/CosmeticEnhancers/Offensive/AmmoEfficiencyOnSliding",
            "Secondary Arcane",
            itemdata::Rarity::Rare,
            true,
            6,
        ),
    ];

    for &(name, unique, typ, ref rarity, tradable, levels) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Arcanes", "{name} category");
        assert_eq!(item.type_field(), typ, "{name} type");
        assert_eq!(item.rarity, Some(rarity.clone()), "{name} rarity");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
        assert_eq!(item.level_stats.len(), levels, "{name} level_stats");
    }
}

// ── Mods ──

#[test]
fn test_mods_deserialize_all() {
    let raw = load_json!("Mods.json");
    let arr: itemdata::mods::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 1000);
}

#[test]
fn test_mods_tabular_fields() {
    let raw = load_json!("Mods.json");
    let arr: itemdata::mods::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, type_field, tradable, is_regular, is_riven, is_set_member, is_set_definition)
    let cases: &[(&str, &str, &str, bool, bool, bool, bool, bool)] = &[
        (
            "Serration",
            "/Lotus/Upgrades/Mods/Rifle/Beginner/WeaponDamageAmountModBeginner",
            "Primary Mod",
            true, true, false, false, false,
        ),
        (
            "Archgun Riven Mod",
            "/Lotus/Upgrades/Mods/Randomized/LotusArchgunRandomModRare",
            "Arch-Gun Riven Mod",
            false, false, true, false, false,
        ),
        (
            "Vigilante Offense",
            "/Lotus/Upgrades/Mods/Sets/Vigilante/PrimaryVigilanteOffenseMod",
            "Primary Mod",
            true, false, false, true, false,
        ),
        (
            "Amarsetmod",
            "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod",
            "Mod Set Mod",
            false, false, false, false, true,
        ),
    ];

    for &(name, unique, typ, tradable, regular, riven, set_member, set_def) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.type_field(), typ, "{name} type");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
        assert_eq!(item.is_regular(), regular, "{name} regular");
        assert_eq!(item.is_riven(), riven, "{name} riven");
        assert_eq!(item.is_set_member(), set_member, "{name} set_member");
        assert_eq!(item.is_set_definition(), set_def, "{name} set_def");
    }
}

#[test]
fn test_mods_variant_details() {
    let raw = load_json!("Mods.json");
    let arr: itemdata::mods::Root = serde_json::from_str(&raw).unwrap();

    // Regular mod details
    let serration = find_by_name(&arr, "Serration");
    match serration {
        itemdata::mods::ModEntry::Regular(m) => {
            assert_eq!(m.rarity, itemdata::Rarity::Uncommon);
            assert_eq!(m.polarity, itemdata::Polarity::Madurai);
            assert_eq!(m.base_drain, 2);
            assert_eq!(m.fusion_limit, 3);
            assert_eq!(m.compat_name, Some("Rifle".to_string()));
        }
        _ => panic!("Expected Regular variant"),
    }
    assert!(serration.has_drops());

    // Riven mod details
    let riven = find_by_name(&arr, "Archgun Riven Mod");
    match riven {
        itemdata::mods::ModEntry::Riven(r) => {
            assert!(!r.available_challenges.is_empty());
        }
        _ => panic!("Expected Riven variant"),
    }

    // Set member details
    let vig = find_by_name(&arr, "Vigilante Offense");
    assert!(matches!(vig.mod_category(), itemdata::ModCategory::SetMember { .. }));

    // Set definition details
    let set_def = find_by_name(&arr, "Amarsetmod");
    match set_def {
        itemdata::mods::ModEntry::SetDefinition(m) => {
            assert_eq!(m.num_upgrades_in_set, 3);
        }
        _ => panic!("Expected SetDefinition variant"),
    }
}

#[test]
fn test_mods_all_variants_present() {
    let raw = load_json!("Mods.json");
    let arr: itemdata::mods::Root = serde_json::from_str(&raw).unwrap();

    let mut has_regular = false;
    let mut has_riven = false;
    let mut has_set_member = false;
    let mut has_set_def = false;

    for m in &arr {
        match m {
            itemdata::mods::ModEntry::Regular(_) => has_regular = true,
            itemdata::mods::ModEntry::Riven(_) => has_riven = true,
            itemdata::mods::ModEntry::SetMember(_) => has_set_member = true,
            itemdata::mods::ModEntry::SetDefinition(_) => has_set_def = true,
        }
    }

    assert!(has_regular, "Should have Regular mods");
    assert!(has_riven, "Should have Riven mods");
    assert!(has_set_member, "Should have SetMember mods");
    assert!(has_set_def, "Should have SetDefinition mods");
}

// ── Pets ──

#[test]
fn test_pets_deserialize_all() {
    let raw = load_json!("Pets.json");
    let arr: itemdata::pet::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 10);
}

#[test]
fn test_pets_tabular_fields() {
    let raw = load_json!("Pets.json");
    let arr: itemdata::pet::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, tradable, has_wikia, has_build_price)
    let cases: &[(&str, &str, bool, bool, bool)] = &[
        (
            "Adarza Kavat",
            "/Lotus/Types/Game/CatbrowPet/MirrorCatbrowPetPowerSuit",
            false, true, false,
        ),
        (
            "Adlet Core",
            "/Lotus/Types/Friendly/Pets/ZanukaPets/ZanukaPetParts/ZanukaPetPartBodyA",
            false, false, true,
        ),
        (
            "Venari",
            "/Lotus/Powersuits/Khora/Kavat/KhoraKavatPowerSuit",
            false, true, false,
        ),
    ];

    for &(name, unique, tradable, has_wikia, has_build) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Pets", "{name} category");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
        assert_eq!(item.wikia_url().is_some(), has_wikia, "{name} has_wikia");
        assert_eq!(item.build_price().is_some(), has_build, "{name} has_build");
    }
}

#[test]
fn test_pets_all_variants_present() {
    let raw = load_json!("Pets.json");
    let arr: itemdata::pet::Root = serde_json::from_str(&raw).unwrap();

    let mut has_kubrow = false;
    let mut has_component = false;
    let mut has_special = false;
    for pet in &arr {
        match pet {
            itemdata::pet::PetEntry::KubrowPets(_) => has_kubrow = true,
            itemdata::pet::PetEntry::Pistols(_) => has_component = true,
            itemdata::pet::PetEntry::SpecialItems(_) => has_special = true,
        }
    }
    assert!(has_kubrow, "Should have KubrowPets entries");
    assert!(has_component, "Should have Pistols (component) entries");
    assert!(has_special, "Should have SpecialItems entries");
}

#[test]
fn test_pets_variant_details() {
    let raw = load_json!("Pets.json");
    let arr: itemdata::pet::Root = serde_json::from_str(&raw).unwrap();

    // KubrowPets variant
    let kavat = find_by_name(&arr, "Adarza Kavat");
    match kavat {
        itemdata::pet::PetEntry::KubrowPets(p) => {
            assert_eq!(p.stats.health, 310);
            assert_eq!(p.stats.armor, 300);
            assert_eq!(p.stats.shield, 270);
        }
        _ => panic!("Expected KubrowPets variant"),
    }

    // Pistols (component) variant
    let core = find_by_name(&arr, "Adlet Core");
    assert!(matches!(core, itemdata::pet::PetEntry::Pistols(_)));

    // SpecialItems variant
    let venari = find_by_name(&arr, "Venari");
    match venari {
        itemdata::pet::PetEntry::SpecialItems(p) => {
            assert_eq!(p.stats.health, 900);
            assert_eq!(p.stats.armor, 350);
            assert!(p.exclude_from_codex);
        }
        _ => panic!("Expected SpecialItems variant"),
    }
}

// ── Sentinels ──

#[test]
fn test_sentinels_deserialize_all() {
    let raw = load_json!("Sentinels.json");
    let arr: itemdata::sentinel::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_sentinels_tabular_fields() {
    let raw = load_json!("Sentinels.json");
    let arr: itemdata::sentinel::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, health, armor, shield, is_prime, vaulted)
    let cases: &[(&str, &str, i64, i64, i64, bool, Option<bool>)] = &[
        (
            "Carrier",
            "/Lotus/Types/Sentinels/SentinelPowersuits/CarrierPowerSuit",
            560, 80, 250, false, None,
        ),
        (
            "Carrier Prime",
            "/Lotus/Types/Sentinels/SentinelPowersuits/PrimeCarrierPowerSuit",
            650, 150, 300, true, Some(true),
        ),
        (
            "Shade",
            "/Lotus/Types/Sentinels/SentinelPowersuits/ShadePowerSuit",
            600, 80, 130, false, None,
        ),
    ];

    for &(name, unique, health, armor, shield, prime, vaulted) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Sentinels", "{name} category");
        assert_eq!(item.health(), health, "{name} health");
        assert_eq!(item.armor(), armor, "{name} armor");
        assert_eq!(item.shield(), shield, "{name} shield");
        assert_eq!(item.is_prime(), prime, "{name} prime");
        assert_eq!(item.vaulted(), vaulted, "{name} vaulted");
    }
}

// ── Sentinel Weapons ──

#[test]
fn test_sentinel_weapons_deserialize_all() {
    let raw = load_json!("SentinelWeapons.json");
    let arr: itemdata::sentinel_weapon::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_sentinel_weapons_tabular_fields() {
    let raw = load_json!("SentinelWeapons.json");
    let arr: itemdata::sentinel_weapon::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, total_damage, fire_rate, disposition)
    let cases: &[(&str, &str, f64, f64, i64)] = &[
        (
            "Artax",
            "/Lotus/Types/Sentinels/SentinelWeapons/Gremlin",
            5.0, 16.67, 3,
        ),
        (
            "Akaten",
            "/Lotus/Types/Friendly/Pets/ZanukaPets/ZanukaPetMeleeWeaponPS",
            300.0, 1.0, 3,
        ),
        (
            "Batoten",
            "/Lotus/Types/Friendly/Pets/ZanukaPets/ZanukaPetMeleeWeaponIP",
            300.0, 1.0, 3,
        ),
    ];

    for &(name, unique, dmg, rate, dispo) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert!((item.total_damage() - dmg).abs() < 1.0, "{name} dmg");
        assert!((item.fire_rate() - rate).abs() < 0.1, "{name} rate");
        assert_eq!(item.disposition(), Some(dispo), "{name} dispo");
        assert_eq!(item.damage_per_shot().len(), 20, "{name} dps len");
    }
}

// ── Gear ──

#[test]
fn test_gear_deserialize_all() {
    let raw = load_json!("Gear.json");
    let arr: itemdata::gear::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_gear_tabular_fields() {
    let raw = load_json!("Gear.json");
    let arr: itemdata::gear::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, tradable, has_build_price)
    let cases: &[(&str, &str, bool, bool)] = &[
        (
            "Advanced Nosam Cutter",
            "/Lotus/Types/Restoratives/Consumable/MiningLaserC",
            false, true,
        ),
        (
            "Codex Scanner",
            "/Lotus/Types/Restoratives/Consumable/Scanner",
            false, false,
        ),
        (
            "Air Support Charges",
            "/Lotus/Types/Restoratives/LisetAirSupport",
            false, true,
        ),
    ];

    for &(name, unique, tradable, has_bp) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Gear", "{name} category");
        assert_eq!(item.type_field(), "Gear", "{name} type");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
        assert_eq!(item.build_price().is_some(), has_bp, "{name} has_build_price");
    }
}

// ── Misc ──

#[test]
fn test_misc_deserialize_all() {
    let raw = load_json!("Misc.json");
    let arr: itemdata::misc::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_misc_tabular_fields() {
    let raw = load_json!("Misc.json");
    let arr: itemdata::misc::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, type, tradable)
    let cases: &[(&str, &str, &str, bool)] = &[
        (
            "<Shard_blue_simple> Azure Archon Shard",
            "/Lotus/Types/Gameplay/NarmerSorties/ArchonCrystalBoreal",
            "Misc", false,
        ),
        (
            "<Shard_blue_simple> Tauforged Azure Archon Shard",
            "/Lotus/Types/Gameplay/NarmerSorties/ArchonCrystalBorealMythic",
            "Misc", false,
        ),
        (
            "\"Circle Of Comrades\" Series On Vhs",
            "/Lotus/Types/Gameplay/1999Wf/Gifts/VideoCassette",
            "Misc", false,
        ),
    ];

    for &(name, unique, typ, tradable) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Misc", "{name} category");
        assert_eq!(item.type_field(), typ, "{name} type");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
    }
}

// ── Relics ──

#[test]
fn test_relics_deserialize_all() {
    let raw = load_json!("Relics.json");
    let arr: itemdata::relics::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_relics_tabular_fields() {
    let raw = load_json!("Relics.json");
    let arr: itemdata::relics::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, tradable)
    let cases: &[(&str, &str, bool)] = &[
        (
            "Axi A1 Exceptional",
            "/Lotus/Types/Game/Projections/T4VoidProjectionESilver",
            true,
        ),
        (
            "Axi A1 Flawless",
            "/Lotus/Types/Game/Projections/T4VoidProjectionEGold",
            true,
        ),
        (
            "Axi A1 Intact",
            "/Lotus/Types/Game/Projections/T4VoidProjectionEBronze",
            true,
        ),
    ];

    for &(name, unique, tradable) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Relics", "{name} category");
        assert_eq!(item.type_field(), "Relic", "{name} type");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
    }
}

// ── Resources ──

#[test]
fn test_resources_deserialize_all() {
    let raw = load_json!("Resources.json");
    let arr: itemdata::resource::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_resources_tabular_fields() {
    let raw = load_json!("Resources.json");
    let arr: itemdata::resource::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, type, tradable)
    let cases: &[(&str, &str, &str, bool)] = &[
        (
            "35mm Film",
            "/Lotus/Types/Gameplay/1999Wf/Resources/HexDogTagQuincy",
            "Resource", false,
        ),
        (
            "Adramalium",
            "/Lotus/Types/Items/Gems/Deimos/DeimosCommonOreAItem",
            "Gem", false,
        ),
        (
            "Adramal Alloy",
            "/Lotus/Types/Items/Gems/Deimos/DeimosCommonOreAAlloyItem",
            "Gem", false,
        ),
    ];

    for &(name, unique, typ, tradable) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Resources", "{name} category");
        assert_eq!(item.type_field(), typ, "{name} type");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
    }
}

// ── Fish ──

#[test]
fn test_fish_deserialize_all() {
    let raw = load_json!("Fish.json");
    let arr: itemdata::fish::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_fish_tabular_fields() {
    let raw = load_json!("Fish.json");
    let arr: itemdata::fish::Root = serde_json::from_str(&raw).unwrap();

    // Fish can have duplicate names (size variants), so test by unique_name
    let cases: &[(&str, &str, bool)] = &[
        (
            "/Lotus/Types/Items/Fish/Deimos/InfestedCommonDFishItem",
            "Amniophysi", true,
        ),
        (
            "/Lotus/Types/Items/Fish/Deimos/InfestedCommonDFishItemLarge",
            "Amniophysi", true,
        ),
        (
            "/Lotus/Types/Items/Fish/Deimos/InfestedCommonDFishItemMedium",
            "Amniophysi", true,
        ),
    ];

    for &(unique, name, tradable) in cases {
        let item = arr.iter().find(|f| f.unique_name() == unique)
            .unwrap_or_else(|| panic!("fish with unique_name '{}' not found", unique));
        assert_eq!(item.name(), name, "{unique} name");
        assert_eq!(item.category(), "Fish", "{unique} category");
        assert_eq!(item.type_field(), "Fish", "{unique} type");
        assert_eq!(item.tradable(), tradable, "{unique} tradable");
    }
}

// ── Glyphs ──

#[test]
fn test_glyphs_deserialize_all() {
    let raw = load_json!("Glyphs.json");
    let arr: itemdata::glyph::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_glyphs_tabular_fields() {
    let raw = load_json!("Glyphs.json");
    let arr: itemdata::glyph::Root = serde_json::from_str(&raw).unwrap();

    let cases: &[(&str, &str, bool)] = &[
        (
            "-Chroma- Prime Partner Glyph",
            "/Lotus/Types/StoreItems/AvatarImages/FanChannel/AvatarImageChromaPrimePartner",
            false,
        ),
        (
            "13angtv Glyph",
            "/Lotus/Types/StoreItems/AvatarImages/FanChannel/AvatarImage13angTV",
            false,
        ),
        (
            "1999 Drippy Glyph",
            "/Lotus/Types/StoreItems/AvatarImages/AvatarImageDrippy",
            false,
        ),
    ];

    for &(name, unique, tradable) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Glyphs", "{name} category");
        assert_eq!(item.type_field(), "Glyph", "{name} type");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
    }
}

// ── Sigils ──

#[test]
fn test_sigils_deserialize_all() {
    let raw = load_json!("Sigils.json");
    let arr: itemdata::sigil::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_sigils_tabular_fields() {
    let raw = load_json!("Sigils.json");
    let arr: itemdata::sigil::Root = serde_json::from_str(&raw).unwrap();

    let cases: &[(&str, &str, bool)] = &[
        (
            "10 Year Anniversary Community Sigil",
            "/Lotus/Upgrades/Skins/Sigils/Community10YearAnniversarySigil",
            false,
        ),
        (
            "2-For-1 Sigil",
            "/Lotus/Upgrades/Skins/Sigils/Syndicate/HexRankThree",
            false,
        ),
        (
            "Accord Sigil",
            "/Lotus/Upgrades/Skins/Sigils/SyndicateSigilConclaveN",
            false,
        ),
    ];

    for &(name, unique, tradable) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Sigils", "{name} category");
        assert_eq!(item.type_field(), "Sigil", "{name} type");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
    }
}

// ── Skins ──

#[test]
fn test_skins_deserialize_all() {
    let raw = load_json!("Skins.json");
    let arr: itemdata::skin::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_skins_tabular_fields() {
    let raw = load_json!("Skins.json");
    let arr: itemdata::skin::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, type, tradable)
    let cases: &[(&str, &str, &str, bool)] = &[
        (
            "17173 Emblem",
            "/Lotus/Upgrades/Skins/Clan/CY17173MediaBadge",
            "Skin", false,
        ),
        (
            "A Lost Time",
            "/Lotus/Types/Items/ShipDecos/NewWar/LisetPropFamilyPortrait",
            "Ship Decoration", false,
        ),
        (
            "Smoke",
            "/Lotus/Types/StoreItems/SuitCustomizations/NinjaColourPickerItem",
            "Color Palette", false,
        ),
    ];

    for &(name, unique, typ, tradable) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Skins", "{name} category");
        assert_eq!(item.type_field(), typ, "{name} type");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
    }
}

// ── Quests ──

#[test]
fn test_quests_deserialize_all() {
    let raw = load_json!("Quests.json");
    let arr: itemdata::quest::Root = serde_json::from_str(&raw).unwrap();
    assert!(!arr.is_empty());
}

#[test]
fn test_quests_tabular_fields() {
    let raw = load_json!("Quests.json");
    let arr: itemdata::quest::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, tradable, has_build_price)
    let cases: &[(&str, &str, bool, bool)] = &[
        (
            "Clan Key",
            "/Lotus/Types/Keys/DojoKey",
            false, true,
        ),
        (
            "A Man Of Few Words",
            "/Lotus/Types/Keys/GetClemQuest/GetClemQuestKeyChain",
            false, false,
        ),
        (
            "Angels Of The Zariman",
            "/Lotus/Types/Keys/ZarimanQuest/ZarimanQuestKeyChain",
            false, false,
        ),
    ];

    for &(name, unique, tradable, has_bp) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Quests", "{name} category");
        assert_eq!(item.tradable(), tradable, "{name} tradable");
        assert_eq!(item.build_price().is_some(), has_bp, "{name} has_build_price");
    }
}

// ── Nodes ──

#[test]
fn test_nodes_deserialize_all() {
    let raw = load_json!("Node.json");
    let arr: itemdata::node::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_nodes_tabular_fields() {
    let raw = load_json!("Node.json");
    let arr: itemdata::node::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, system_name, min_level, max_level)
    let cases: &[(&str, &str, &str, i64, i64)] = &[
        ("Adaro", "SolNode181", "Sedna", 32, 36),
        ("Hydron", "SolNode195", "Sedna", 30, 40),
        ("Mot", "SolNode409", "Void", 40, 45),
    ];

    for &(name, unique, system, min_lv, max_lv) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Node", "{name} category");
        assert_eq!(item.system_name, system, "{name} system");
        assert_eq!(item.min_enemy_level, min_lv, "{name} min_level");
        assert_eq!(item.max_enemy_level, max_lv, "{name} max_level");
    }
}

// ── Enemies ──

#[test]
fn test_enemies_deserialize_all() {
    let raw = load_json!("Enemy.json");
    let arr: itemdata::enemy::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 100);
}

#[test]
fn test_enemies_tabular_fields() {
    let raw = load_json!("Enemy.json");
    let arr: itemdata::enemy::Root = serde_json::from_str(&raw).unwrap();

    // (unique_name, type, health, armor, shield, resistances_count)
    let cases: &[(&str, &str, i64, i64, i64, usize)] = &[
        (
            "/Lotus/Types/Enemies/Orokin/OrokinRocketBombardAvatar",
            "Orokin", 300, 500, 0, 3,
        ),
        (
            "/Lotus/Types/Enemies/Grineer/SeaLab/Avatars/EliteRifleLancerAvatar",
            "Grineer", 150, 200, 0, 3,
        ),
        (
            "/Lotus/Types/Enemies/Corpus/Spaceman/AIWeek/DeployableSpacemanAvatar",
            "Corpus", 700, 0, 250, 3,
        ),
    ];

    for &(unique, typ, health, armor, shield, res_count) in cases {
        let item = arr.iter().find(|e| e.unique_name() == unique)
            .unwrap_or_else(|| panic!("enemy '{}' not found", unique));
        assert_eq!(item.type_field(), typ, "{unique} type");
        assert_eq!(item.combat.health, health, "{unique} health");
        assert_eq!(item.combat.armor, armor, "{unique} armor");
        assert_eq!(item.combat.shield, shield, "{unique} shield");
        assert_eq!(item.combat.resistances.len(), res_count, "{unique} resistances");
    }
}

// ── Railjack ──

#[test]
fn test_railjack_deserialize_all() {
    let raw = load_json!("Railjack.json");
    let arr: itemdata::railjack::Root = serde_json::from_str(&raw).unwrap();
    assert!(arr.len() > 50);
}

#[test]
fn test_railjack_tabular_fields() {
    let raw = load_json!("Railjack.json");
    let arr: itemdata::railjack::Root = serde_json::from_str(&raw).unwrap();

    // (name, unique_name, total_damage, fire_rate, crit_chance)
    let cases: &[(&str, &str, f64, f64, f64)] = &[
        (
            "Apoc",
            "/Lotus/Weapons/CrewShip/MassDriver/AutoCannon/AutoCannon",
            126.0, 8.33, 0.1,
        ),
        (
            "Apoc Mk I",
            "/Lotus/Weapons/CrewShip/MassDriver/AutoCannon/AutoCannonTierA",
            227.0, 8.33, 0.1,
        ),
        (
            "Apoc Mk Ii",
            "/Lotus/Weapons/CrewShip/MassDriver/AutoCannon/AutoCannonTierB",
            386.0, 8.33, 0.14,
        ),
    ];

    for &(name, unique, dmg, rate, crit) in cases {
        let item = find_by_name(&arr, name);
        assert_eq!(item.unique_name(), unique, "{name} unique_name");
        assert_eq!(item.category(), "Railjack", "{name} category");
        assert!((item.total_damage() - dmg).abs() < 1.0, "{name} dmg");
        assert!((item.fire_rate() - rate).abs() < 0.1, "{name} rate");
        assert!((item.critical_chance() - crit).abs() < 0.01, "{name} crit");
    }
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
