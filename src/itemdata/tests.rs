use crate::itemdata;

#[test]
fn test_deserialize_all_warframes() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-json-data/Warframes.json"
    ));

    let arr: Vec<itemdata::warframe::Warframe> = serde_json::from_str(raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_primary_weapons() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-json-data/Primary.json"
    ));

    let arr: Vec<itemdata::primary::Primary> = serde_json::from_str(raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_secondary_weapons() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-json-data/Secondary.json"
    ));

    let arr: Vec<itemdata::secondary::Secondary> = serde_json::from_str(raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_melee_weapons() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-json-data/Melee.json"
    ));

    let arr: Vec<itemdata::melee::Melee> = serde_json::from_str(raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_arcanes() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-json-data/Arcanes.json"
    ));

    let arr: Vec<itemdata::arcane::Arcane> = serde_json::from_str(raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_mods() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-json-data/Mods.json"
    ));

    let arr: Vec<itemdata::mods::Mod> = serde_json::from_str(raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_archwings() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-json-data/Archwing.json"
    ));

    let arr: Vec<itemdata::archwing::Archwing> = serde_json::from_str(raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_archguns() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-json-data/Arch-Gun.json"
    ));

    let arr: Vec<itemdata::arch_gun::ArchGun> = serde_json::from_str(raw).unwrap();

    assert!(!arr.is_empty());
}

#[test]
fn test_deserialize_all_archmelee() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/warframe-items-json-data/Arch-Melee.json"
    ));

    let arr: Vec<itemdata::arch_melee::ArchMelee> = serde_json::from_str(raw).unwrap();

    assert!(!arr.is_empty());
}
