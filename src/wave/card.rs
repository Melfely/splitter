use bevy::prelude::*;
use rand::RngExt;

use crate::player::definitions::{CoaxialWeapon, MainWeapon};
use crate::splitter_core::weapon::Weapon;
use crate::wave::cards;
use crate::wave::definitions::{Card, CardId};

pub struct WeaponInfo {
    pub entity: Entity,
    pub name: String,
    pub weapon: Weapon,
}

impl WeaponInfo {
    pub fn from_query(
        query: &Query<(
            Entity,
            &Weapon,
            Option<&Name>,
            Option<&MainWeapon>,
            Option<&CoaxialWeapon>,
        )>,
    ) -> Vec<Self> {
        query
            .iter()
            .enumerate()
            .map(|(index, (entity, weapon, name, is_main, is_coaxial))| {
                let weapon_name = if let Some(n) = name {
                    n.as_str().to_string()
                } else if is_main.is_some() {
                    "Main Turret".to_string()
                } else if is_coaxial.is_some() {
                    "Coaxial Cannon".to_string()
                } else {
                    format!("Weapon Slot {}", index + 1)
                };

                WeaponInfo {
                    entity,
                    name: weapon_name,
                    weapon: weapon.clone(),
                }
            })
            .collect()
    }
}

/// Single card generator: delegates card filtering to domain modules and selects one valid card.
pub fn roll_single_card(
    equipped_weapons: &[WeaponInfo],
    existing_options: &[Box<dyn Card>],
) -> Option<Box<dyn Card>> {
    let mut candidates = cards::get_all_valid_cards(equipped_weapons);

    // Exclude cards already selected in current selection screen
    let existing_ids: Vec<CardId> = existing_options.iter().map(|c| c.id()).collect();
    candidates.retain(|c| !existing_ids.contains(&c.id()));

    if candidates.is_empty() {
        return None;
    }

    let mut rng = rand::rng();
    let idx = rng.random_range(0..candidates.len());
    Some(candidates.swap_remove(idx))
}

#[derive(Resource, Default)]
pub struct ActiveCardSelection {
    pub options: Vec<Box<dyn Card>>,
}

impl ActiveCardSelection {
    pub fn generate_selection(&mut self, equipped_weapons: &[WeaponInfo]) {
        self.options.clear();
        for _ in 0..3 {
            if let Some(card) = roll_single_card(equipped_weapons, &self.options) {
                self.options.push(card);
            }
        }
    }
}
