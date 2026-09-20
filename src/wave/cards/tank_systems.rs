use bevy::prelude::*;

use crate::player::definitions::{PlayerBlink, PlayerPhysics, Shield};
use crate::splitter_core::turret::Turret;
use crate::splitter_core::weapon::Weapon;
use crate::wave::definitions::{Card, CardHistory, CardId, CardType};

pub fn get_tank_system_cards() -> Vec<Box<dyn Card>> {
    vec![
        Box::new(TurretSpeedCard),
        Box::new(EngineAccelerationCard),
        Box::new(BlinkCooldownCard),
        Box::new(ShieldRegenCard),
    ]
}

pub struct TurretSpeedCard;

impl Card for TurretSpeedCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::TurretSpeed,
            target_entity: None,
        }
    }

    fn title(&self) -> String {
        "TURRET SERVO MOTORS".to_string()
    }

    fn description(&self) -> String {
        "+30% Tank Turret Rotation Speed.".to_string()
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        _physics: &mut Query<&mut PlayerPhysics>,
        _blink: &mut Query<&mut PlayerBlink>,
        _shield: &mut Query<&mut Shield>,
        turrets: &mut Query<&mut Turret>,
        _weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();
        let factor = history.linear_increase_factor(&card_id, 0.30);

        for mut turret in turrets.iter_mut() {
            turret.turn_speed *= factor;
        }

        history.increment(&card_id);
    }
}

pub struct EngineAccelerationCard;

impl Card for EngineAccelerationCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::EngineAcceleration,
            target_entity: None,
        }
    }

    fn title(&self) -> String {
        "HEAVY TREAD ACTUATORS".to_string()
    }

    fn description(&self) -> String {
        "+25% Track Acceleration and +15% Maximum Speed.".to_string()
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        physics: &mut Query<&mut PlayerPhysics>,
        _blink: &mut Query<&mut PlayerBlink>,
        _shield: &mut Query<&mut Shield>,
        _turrets: &mut Query<&mut Turret>,
        _weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();
        let accel_factor = history.linear_increase_factor(&card_id, 0.25);
        let speed_factor = history.linear_increase_factor(&card_id, 0.15);

        for mut phys in physics.iter_mut() {
            phys.acceleration *= accel_factor;
            phys.max_speed *= speed_factor;
        }

        history.increment(&card_id);
    }
}

pub struct BlinkCooldownCard;

impl Card for BlinkCooldownCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::BlinkCooldown,
            target_entity: None,
        }
    }

    fn title(&self) -> String {
        "TACTICAL TELEPORT CAPACITOR".to_string()
    }

    fn description(&self) -> String {
        "-20% Dash Cooldown Duration.".to_string()
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        _physics: &mut Query<&mut PlayerPhysics>,
        blink: &mut Query<&mut PlayerBlink>,
        _shield: &mut Query<&mut Shield>,
        _turrets: &mut Query<&mut Turret>,
        _weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();
        let factor = history.linear_decrease_factor(&card_id, 0.20);

        for mut b in blink.iter_mut() {
            let current_q = b.q_cooldown.duration().as_secs_f32();
            let current_e = b.e_cooldown.duration().as_secs_f32();
            b.q_cooldown
                .set_duration(std::time::Duration::from_secs_f32(current_q * factor));
            b.e_cooldown
                .set_duration(std::time::Duration::from_secs_f32(current_e * factor));
        }

        history.increment(&card_id);
    }
}

pub struct ShieldRegenCard;

impl Card for ShieldRegenCard {
    fn id(&self) -> CardId {
        CardId {
            card_type: CardType::ShieldRegen,
            target_entity: None,
        }
    }

    fn title(&self) -> String {
        "REINFORCED DEFLECTOR GRID".to_string()
    }

    fn description(&self) -> String {
        "+25% Energy Shield Regeneration Rate and -15% Recharge Delay.".to_string()
    }

    fn apply(
        &self,
        history: &mut CardHistory,
        _physics: &mut Query<&mut PlayerPhysics>,
        _blink: &mut Query<&mut PlayerBlink>,
        shield: &mut Query<&mut Shield>,
        _turrets: &mut Query<&mut Turret>,
        _weapons: &mut Query<(Entity, &mut Weapon)>,
    ) {
        let card_id = self.id();
        let rate_factor = history.linear_increase_factor(&card_id, 0.25);
        let delay_factor = history.linear_decrease_factor(&card_id, 0.15);

        for mut s in shield.iter_mut() {
            s.regen_rate *= rate_factor;
            s.recharge_delay *= delay_factor;
        }

        history.increment(&card_id);
    }
}
