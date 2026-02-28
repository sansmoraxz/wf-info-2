// Shared types - import from these modules instead of defining locally
pub mod common;
pub mod components;
pub mod damage;
pub mod enums;
pub mod props;
pub mod traits;

// Re-export common traits for easier access
pub use traits::{
    Buildable, Character, Droppable, Equippable, HasAbilities, Item, MeleeWeapon, Prime,
    RangedWeapon, Weapon, WikiaLinked,
};

// Re-export important enums for easier access
pub use enums::{
    ArcaneType, ArchGunProductCategory, ArchMeleeProductCategory, ArchwingProductCategory,
    GearType, MeleeProductCategory, MeleeType, ModCategory, Noise, Polarity,
    PrimaryProductCategory, PrimaryType, RailjackType, Rarity, ResourceType,
    SecondaryProductCategory, SecondaryType, SentinelProductCategory,
    SentinelWeaponProductCategory, Sex, Slot, Trigger, VaultStatus,
};
pub use props::{MeleeWeaponData, RangedWeaponData, WeaponTypeStats};

// Category-specific modules
pub mod melee;
pub mod primary;
pub mod secondary;
pub mod warframe;

pub mod arch_gun;
pub mod arch_melee;
pub mod archwing;

pub mod pet;
pub mod sentinel;
pub mod sentinel_weapon;

pub mod arcane;
pub mod mods;

pub mod gear;
pub mod misc;
pub mod relics;
pub mod resource;

pub mod enemy;
pub mod fish;
pub mod glyph;
pub mod node;
pub mod quest;
pub mod railjack;
pub mod sigil;
pub mod skin;

/// Trait for items that belong to product categories (for inventory lookup).
pub trait ProductCategory {
    fn get_product_categories(&self) -> Vec<String>;
}

#[cfg(test)]
mod tests;
