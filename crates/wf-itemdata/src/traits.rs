//! Common traits for querying item properties uniformly across all item types.

use crate::common::{Ability, Drop, Introduced, Patchlog};
use crate::components::Component;
use crate::damage::{Attack, DamageBreakdown};
use crate::enums::{Polarity, Slot, VaultStatus};

// =============================================================================
// Core Item Trait (Universal - ALL types implement this)
// =============================================================================

/// Core item properties that ALL items in the game have.
/// This is the base trait that every item type must implement.
#[enum_dispatch::enum_dispatch]
pub trait Item {
    /// Item category (e.g., "Warframes", "Primary", "Mods")
    fn category(&self) -> &str;

    /// Description text shown to players, if available
    fn description(&self) -> Option<&str> {
        None
    }

    /// Image filename for the item
    fn image_name(&self) -> Option<&str>;

    /// Whether the item contributes to Mastery Rank
    fn masterable(&self) -> bool;

    /// Display name shown to players
    fn name(&self) -> &str;

    /// History of patches that affected this item
    fn patchlogs(&self) -> &[Patchlog];

    /// Whether the item can be traded between players
    fn tradable(&self) -> bool;

    /// Item type within category (e.g., "Warframe", "Rifle", "Pistol")
    fn type_field(&self) -> &str;

    /// Internal unique identifier path (e.g., "/Lotus/Powersuits/...")
    fn unique_name(&self) -> &str;
}

// =============================================================================
// Capability Traits (Optional - types implement as appropriate)
// =============================================================================

/// Items that can drop from enemies, missions, or other sources.
pub trait Droppable: Item {
    /// All drop sources and chances for this item
    fn drops(&self) -> &[Drop];

    /// Check if the item has any recorded drop sources
    fn has_drops(&self) -> bool {
        !self.drops().is_empty()
    }
}

/// Items that can be crafted in the Foundry.
pub trait Buildable: Item {
    /// Blueprint credit cost
    fn bp_cost(&self) -> Option<i64>;

    /// Credit cost to build
    fn build_price(&self) -> Option<i64>;

    /// Quantity produced per build
    fn build_quantity(&self) -> Option<i64>;

    /// Build time in seconds
    fn build_time(&self) -> Option<i64>;

    /// Required crafting components
    fn components(&self) -> &[Component];

    /// Whether crafting consumes the blueprint
    fn consume_on_build(&self) -> Option<bool>;

    /// Check if item is craftable (has build price or components)
    fn is_craftable(&self) -> bool {
        self.build_price().is_some() || !self.components().is_empty()
    }

    /// Market platinum price (if available in market)
    fn market_cost(&self) -> Option<i64>;

    /// Required Mastery Rank to build/use
    fn mastery_req(&self) -> Option<i64>;

    /// Platinum cost to rush build
    fn skip_build_time_price(&self) -> Option<i64>;
}

/// Items that are Prime variants and can be vaulted.
pub trait Prime: Item {
    /// Predicted future vault date
    fn estimated_vault_date(&self) -> Option<&str>;

    /// Check if item is currently accessible (not vaulted or not a Prime)
    fn is_accessible(&self) -> bool {
        !self.is_prime() || !self.vaulted().unwrap_or(false)
    }

    /// Whether this is a Prime variant
    fn is_prime(&self) -> bool;

    /// Date when the item was vaulted
    fn vault_date(&self) -> Option<&str>;

    /// Get the computed vault status as a type-safe enum.
    ///
    /// This provides a cleaner API than checking individual fields.
    fn vault_status(&self) -> VaultStatus {
        if !self.is_prime() {
            return VaultStatus::NotPrime;
        }
        if self.vaulted().unwrap_or(false) {
            return VaultStatus::Vaulted {
                date: self.vault_date().map(str::to_owned),
            };
        }
        self.estimated_vault_date()
            .map_or(VaultStatus::Active, |estimated_date| {
                VaultStatus::EstimatedVault {
                    estimated_date: estimated_date.to_owned(),
                }
            })
    }

    /// Whether the item is currently vaulted (relics no longer drop)
    fn vaulted(&self) -> Option<bool>;
}

/// Items with wiki/external documentation links.
pub trait WikiaLinked: Item {
    /// Update that introduced this item
    fn introduced(&self) -> Option<&Introduced>;

    /// Release date string
    fn release_date(&self) -> Option<&str>;

    /// Whether wiki information is available
    fn wiki_available(&self) -> Option<bool>;

    /// Wiki thumbnail image URL
    fn wikia_thumbnail(&self) -> Option<&str>;

    /// Fandom wiki URL
    fn wikia_url(&self) -> Option<&str>;
}

/// Weapons with damage stats and attack properties.
pub trait Weapon: Item {
    /// Alternative attack modes
    fn attacks(&self) -> &[Attack];

    /// Critical hit chance (0.0 - 1.0+)
    fn critical_chance(&self) -> f64;

    /// Critical damage multiplier
    fn critical_multiplier(&self) -> f64;

    /// Damage breakdown by type
    fn damage(&self) -> Option<&DamageBreakdown>;

    /// Raw damage values per shot
    fn damage_per_shot(&self) -> &[f64];

    /// Riven disposition (1-5, affects Riven mod power)
    fn disposition(&self) -> Option<i64>;

    /// Attack speed / fire rate
    fn fire_rate(&self) -> f64;

    /// Omega attenuation value
    fn omega_attenuation(&self) -> f64;

    /// Status proc chance (0.0 - 1.0+)
    fn proc_chance(&self) -> f64;

    /// Total damage per attack
    fn total_damage(&self) -> f64;
}

/// Ranged weapons with gun-specific properties.
pub trait RangedWeapon: Weapon {
    /// Accuracy rating
    fn accuracy(&self) -> f64;

    /// Magazine capacity
    fn magazine_size(&self) -> Option<i64>;

    /// Projectiles per shot
    fn multishot(&self) -> i64;

    /// Noise level (Alarming/Silent)
    fn noise(&self) -> &str;

    /// Reload time in seconds
    fn reload_time(&self) -> f64;

    /// Trigger type (Auto/Semi/Burst/etc)
    fn trigger(&self) -> &str;
}

/// Melee weapons with close-combat properties.
pub trait MeleeWeapon: Weapon {
    /// Blocking angle in degrees
    fn blocking_angle(&self) -> Option<i64>;

    /// Combo counter duration
    fn combo_duration(&self) -> Option<i64>;

    /// Follow-through value
    fn follow_through(&self) -> Option<f64>;

    /// Heavy attack damage
    fn heavy_attack_damage(&self) -> Option<i64>;

    /// Attack range in meters
    fn range(&self) -> Option<f64>;

    /// Slam attack damage
    fn slam_attack(&self) -> Option<i64>;

    /// Stance mod polarity
    fn stance_polarity(&self) -> Option<&str>;
}

/// Character suits with health/shield/armor stats.
pub trait Character: Item {
    /// Armor value (damage reduction)
    fn armor(&self) -> i64;

    /// Base health value
    fn health(&self) -> i64;

    /// Energy pool
    fn power(&self) -> i64;

    /// Base shield value
    fn shield(&self) -> i64;

    /// Sprint speed multiplier
    fn sprint_speed(&self) -> Option<f64>;

    /// Base stamina
    fn stamina(&self) -> i64;
}

/// Characters with special abilities (Warframes, Archwings).
pub trait HasAbilities: Character {
    /// List of abilities
    fn abilities(&self) -> &[Ability];

    /// Number of abilities
    fn ability_count(&self) -> usize {
        self.abilities().len()
    }
}

/// Items that can be equipped with mods (have polarity slots).
pub trait Equippable: Item {
    /// Mod polarity slots
    fn polarities(&self) -> &[Polarity];

    /// Equipment slot classification
    fn slot(&self) -> Option<&Slot>;
}
