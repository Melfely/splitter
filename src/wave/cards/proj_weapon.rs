use bevy::prelude::*;

use crate::player::definitions::{PlayerBlink, PlayerPhysics, Shield};
use crate::splitter_core::turret::Turret;
use crate::splitter_core::weapon::{Weapon, WeaponKind};
use crate::wave::card::WeaponInfo;
use crate::wave::definitions::{Card, CardHistory, CardId, CardType};

/// Evaluates equipped projectile weapons and returns only eligible cards.
pub fn get_proj_weapon_cards(equipped_weapons: &[WeaponInfo]) -> Vec<Box<dyn Card>> {
    let mut cards: Vec<Box<dyn Card>> = Vec::new();

    for w in equipped_weapons {
        let WeaponKind::Projectile {
            fire_delay,
            max_ammo,
            pellets_per_shot,
            aoe_max_range,
            ..
        } = &w.weapon.kind
        else {
            continue;
        };

        let is_aoe = aoe_max_range.is_some();
        let is_single_pellet = *pellets_per_shot <= 1;
        let has_fire_delay = *fire_delay > 0.0001;

        if has_fire_delay {
            cards.push(Box::new(FireRateCard {
                target: w.entity,
                weapon_name: w.name.clone(),
            }));
        }

        if has_fire_delay && *max_ammo > 1 {
            cards.push(Box::new(MultishotCard {
                target: w.entity,
                weapon_name: w.name.clone(),
            }));
        }

        if *max_ammo > 1 {
            cards.push(Box::new(ReloadSpeedCard {
                target: w.entity,
                weapon_name: w.name.clone(),
            }));
        }

        if is_single_pellet {
            cards.push(Box::new(ExtraPelletCard {
                target: w.entity,
                weapon_name: w.name.clone(),
            }));
        }

        if !is_aoe {
            cards.push(Box::new(ProjectileSpeedCard {
                target: w.entity,
                weapon_name: w.name.clone(),
            }));
            cards.push(Box::new(BaseRoundMassCard {
                target: w.entity,
                weapon_name: w.name.clone(),
            }));
        }

        if is_aoe {}
    }

    cards
}

pub struct FireRateCard {
    pub target: Entity,
    pub weapon_name: String,
}

impl Card for FireRateCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::FireRate,
            target_entity: Some(self.target),
        }
    }

    fn title(&self) -> String {
        format!("OVERCLOCK IGNITION [{}]", self.weapon_name.to_uppercase())
    }

    fn description(&self) -> String {
        format!("-20% Fire Delay on {}.", self.weapon_name)
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        _physics: &mut Query<&mut PlayerPhysics>,
        _blink: &mut Query<&mut PlayerBlink>,
        _shield: &mut Query<&mut Shield>,
        _turrets: &mut Query<&mut Turret>,
        weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();
        let factor = history.linear_decrease_factor(&card_id, 0.20);

        if let Ok((_, mut weapon)) = weapons.get_mut(self.target) {
            if let WeaponKind::Projectile {
                ref mut fire_delay, ..
            } = weapon.kind
            {
                *fire_delay *= factor;
            }
        }

        history.increment(&card_id);
    }
}

pub struct MultishotCard {
    pub target: Entity,
    pub weapon_name: String,
}

impl Card for MultishotCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::Multishot,
            target_entity: Some(self.target),
        }
    }

    fn title(&self) -> String {
        format!("MAGAZINE EXPANSION [{}]", self.weapon_name.to_uppercase())
    }

    fn description(&self) -> String {
        format!("+50% Max Ammo capacity on {}.", self.weapon_name)
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        _physics: &mut Query<&mut PlayerPhysics>,
        _blink: &mut Query<&mut PlayerBlink>,
        _shield: &mut Query<&mut Shield>,
        _turrets: &mut Query<&mut Turret>,
        weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();
        let factor = history.linear_increase_factor(&card_id, 0.50);

        if let Ok((_, mut weapon)) = weapons.get_mut(self.target) {
            if let WeaponKind::Projectile {
                ref mut max_ammo, ..
            } = weapon.kind
            {
                let new_max = ((*max_ammo as f32) * factor).round() as u32;
                *max_ammo = new_max.max(*max_ammo + 1);
            }
        }

        history.increment(&card_id);
    }
}

pub struct ReloadSpeedCard {
    pub target: Entity,
    pub weapon_name: String,
}

impl Card for ReloadSpeedCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::ReloadSpeed,
            target_entity: Some(self.target),
        }
    }

    fn title(&self) -> String {
        format!(
            "AUTO-LOADER AUTOROTATION [{}]",
            self.weapon_name.to_uppercase()
        )
    }

    fn description(&self) -> String {
        format!("-25% Magazine Reload Delay on {}.", self.weapon_name)
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        _physics: &mut Query<&mut PlayerPhysics>,
        _blink: &mut Query<&mut PlayerBlink>,
        _shield: &mut Query<&mut Shield>,
        _turrets: &mut Query<&mut Turret>,
        weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();
        let factor = history.linear_decrease_factor(&card_id, 0.25);

        if let Ok((_, mut weapon)) = weapons.get_mut(self.target) {
            if let WeaponKind::Projectile {
                ref mut reload_delay,
                ..
            } = weapon.kind
            {
                *reload_delay *= factor;
            }
        }

        history.increment(&card_id);
    }
}

pub struct ExtraPelletCard {
    pub target: Entity,
    pub weapon_name: String,
}

impl Card for ExtraPelletCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::Multishot,
            target_entity: Some(self.target),
        }
    }

    fn title(&self) -> String {
        format!("SPLIT BARREL ADAPTER [{}]", self.weapon_name.to_uppercase())
    }

    fn description(&self) -> String {
        format!(
            "+1 Extra projectile per shot (+1.0° spread cone) on {}.",
            self.weapon_name
        )
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        _physics: &mut Query<&mut PlayerPhysics>,
        _blink: &mut Query<&mut PlayerBlink>,
        _shield: &mut Query<&mut Shield>,
        _turrets: &mut Query<&mut Turret>,
        weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();

        if let Ok((_, mut weapon)) = weapons.get_mut(self.target) {
            if let WeaponKind::Projectile {
                ref mut pellets_per_shot,
                ref mut spread_angle,
                ..
            } = weapon.kind
            {
                *pellets_per_shot += 1;
                *spread_angle += 1.0;
            }
        }

        history.increment(&card_id);
    }
}

pub struct ProjectileSpeedCard {
    pub target: Entity,
    pub weapon_name: String,
}

impl Card for ProjectileSpeedCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::ProjectileSpeed,
            target_entity: Some(self.target),
        }
    }

    fn title(&self) -> String {
        format!("HIGH VELOCITY BARREL [{}]", self.weapon_name.to_uppercase())
    }

    fn description(&self) -> String {
        format!("+25% Projectile Speed on {}.", self.weapon_name)
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        _physics: &mut Query<&mut PlayerPhysics>,
        _blink: &mut Query<&mut PlayerBlink>,
        _shield: &mut Query<&mut Shield>,
        _turrets: &mut Query<&mut Turret>,
        weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();
        let factor = history.linear_increase_factor(&card_id, 0.25);

        if let Ok((_, mut weapon)) = weapons.get_mut(self.target) {
            if let WeaponKind::Projectile { ref mut speed, .. } = weapon.kind {
                *speed *= factor;
            }
        }

        history.increment(&card_id);
    }
}

pub struct BaseRoundMassCard {
    pub target: Entity,
    pub weapon_name: String,
}

impl Card for BaseRoundMassCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::ProjectileSpeed,
            target_entity: Some(self.target),
        }
    }

    fn title(&self) -> String {
        format!("HEAVY CALIBER SLUGS [{}]", self.weapon_name.to_uppercase())
    }

    fn description(&self) -> String {
        format!("+30% Base Round Mass on {}.", self.weapon_name)
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        _physics: &mut Query<&mut PlayerPhysics>,
        _blink: &mut Query<&mut PlayerBlink>,
        _shield: &mut Query<&mut Shield>,
        _turrets: &mut Query<&mut Turret>,
        weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();
        let factor = history.linear_increase_factor(&card_id, 0.30);

        if let Ok((_, mut weapon)) = weapons.get_mut(self.target) {
            if let WeaponKind::Projectile { ref mut mass, .. } = weapon.kind {
                *mass *= factor;
            }
        }

        history.increment(&card_id);
    }
}
