use crate::common::extract_param;
use serde_json::Value;

#[derive(Clone, Debug)]
pub(crate) enum WeaponType {
    None,
    Maces,
    Polearms,
    Short,
    Long,
    Axes,
    Slings,
    Staves,
    Bows,
    Ranged,
}

#[derive(Clone, Debug)]
pub(crate) struct Weapon {
    pub(crate) data_collected: bool,
    pub(crate) weapon_type: WeaponType,
    pub(crate) base_accuracy: Option<i32>,
    pub(crate) base_damage: Option<i32>,
    pub(crate) base_attack_delay: Option<i32>,
    pub(crate) damage_rating: Option<i32>,
    pub(crate) rating: i32,
}

impl Weapon {
    pub(crate) fn new() -> Self {
        Self {
            data_collected: false,
            weapon_type: WeaponType::None,
            base_accuracy: None,
            base_damage: None,
            base_attack_delay: None,
            damage_rating: None,
            rating: -100,
        }
    }

    pub(crate) fn update_weapon(&mut self, weapon_desc: Value) {
        self.data_collected = true;

        let body = weapon_desc["body"].to_string();

        if body.contains("{Maces & Flails}") {
            self.weapon_type = WeaponType::Maces;
        } else if body.contains("{Polearms}") {
            self.weapon_type = WeaponType::Polearms;
        } else if body.contains("{Short Blades}") {
            self.weapon_type = WeaponType::Short;
        } else if body.contains("{Long Blades}") {
            self.weapon_type = WeaponType::Long;
        } else if body.contains("{Axes}") {
            self.weapon_type = WeaponType::Axes;
        } else if body.contains("{Slings}") {
            self.weapon_type = WeaponType::Slings;
        } else if body.contains("{Staves}") {
            self.weapon_type = WeaponType::Staves;
        } else if body.contains("{Bows}") {
            self.weapon_type = WeaponType::Bows;
        } else if body.contains("{Ranged Weapons}") {
            self.weapon_type = WeaponType::Ranged;
        }

        // Base accuracy
        let temp = extract_param(&body, "Base accuracy: ", &vec![' ', '\\']);
        self.base_accuracy = match temp {
            None => None,
            Some(temp) if temp.parse::<i32>().is_ok() => Some(temp.parse::<i32>().unwrap()),
            _ => None,
        };

        // Base damage
        let temp = extract_param(&body, "Base damage: ", &vec![' ', '\\']);
        self.base_damage = match temp {
            None => None,
            Some(temp) if temp.parse::<i32>().is_ok() => Some(temp.parse::<i32>().unwrap()),
            _ => None,
        };

        // Base attack delay
        let temp = extract_param(&body, "Base attack delay: ", &vec![' ', '\\']);
        self.base_attack_delay = match temp {
            None => None,
            Some(temp) if temp.parse::<f64>().is_ok() => {
                Some((temp.parse::<f64>().unwrap() * 10.0) as i32)
            }
            _ => None,
        };

        // Damage rating
        let temp = extract_param(&body, "Damage rating: ", &vec![' ', '\\']);
        self.damage_rating = match temp {
            None => None,
            Some(temp) if temp.parse::<i32>().is_ok() => Some(temp.parse::<i32>().unwrap()),
            _ => None,
        };

        // TODO - Implement ranged weapons, temp solution to disable them
        if body.contains("{Ranged Weapons}") || body.contains("{Bows}") {
            self.damage_rating = None;
        }

        // TEMP -- Only allow axes
        if !body.contains("{Axes}") {
            self.damage_rating = None;
        }

        self.rating = self.rate_weapon();
    }

    pub(crate) fn rate_weapon(&mut self) -> i32 {
        self.damage_rating.unwrap_or(-100)
    }
}
