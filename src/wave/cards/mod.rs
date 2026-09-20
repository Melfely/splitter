pub mod proj_weapon;
pub mod tank_systems;

pub use proj_weapon::*;
pub use tank_systems::*;

use crate::wave::card::WeaponInfo;
use crate::wave::definitions::Card;

/// Queries domain submodules and collects all currently valid cards for the tank.
pub fn get_all_valid_cards(equipped_weapons: &[WeaponInfo]) -> Vec<Box<dyn Card>> {
    let mut cards = Vec::new();
    cards.extend(proj_weapon::get_proj_weapon_cards(equipped_weapons));
    cards.extend(tank_systems::get_tank_system_cards());
    cards
}
