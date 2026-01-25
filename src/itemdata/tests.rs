use crate::itemdata;

#[test]
fn test_deserialize_all_warframes() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Warframes.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::warframe::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_primary_weapons() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Primary.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::primary::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_secondary_weapons() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Secondary.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::secondary::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_melee_weapons() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Melee.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::melee::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_archwings() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Archwing.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::archwing::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_archguns() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Arch-Gun.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::arch_gun::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_archmelee() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Arch-Melee.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::arch_melee::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_arcanes() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Arcanes.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::arcane::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_mods() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Mods.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::mods::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_pets() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Pets.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::pet::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_sentinel() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Sentinels.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::sentinel::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_sentinel_weapons() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/SentinelWeapons.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::sentinel_weapon::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_gear() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Gear.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::gear::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_misc() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Misc.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::misc::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_relics() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Relics.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::relics::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_resources() {
    let f = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-data/json/Resources.json"
    );

    let raw = std::fs::read_to_string(f).unwrap();

    let arr: itemdata::resource::Root = serde_json::from_str(&raw).unwrap();

    assert!(!arr.is_empty());
}
