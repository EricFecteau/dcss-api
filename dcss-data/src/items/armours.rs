use crate::common::extract_param;
use serde_json::Value;

#[derive(Clone, Debug)]
pub(crate) enum ArmourType {
    None,
    Body,
    Boots,
    Cloak,
    Helmet,
    Shield,
    Gloves,
}

#[derive(Debug, Clone)]
pub(crate) struct Armour {
    pub(crate) data_collected: bool,
    pub(crate) armour_type: ArmourType,
    pub(crate) base_rating: Option<i32>,
    pub(crate) encumbrance: Option<i32>,
    pub(crate) useless: bool,
    pub(crate) rating: i32,
}

impl Armour {
    pub(crate) fn new() -> Self {
        Self {
            data_collected: false,
            armour_type: ArmourType::None,
            base_rating: None,
            encumbrance: None,
            useless: false,
            rating: -100,
        }
    }

    pub(crate) fn update_armour(&mut self, armour_desc: Value) {
        self.data_collected = true;

        let body = armour_desc["body"].to_string();

        if body.contains("{body armour}") {
            self.armour_type = ArmourType::Body;
        } else if body.contains("{boots armour}") {
            self.armour_type = ArmourType::Boots;
        } else if body.contains("{cloak armour}") {
            self.armour_type = ArmourType::Cloak;
        } else if body.contains("{helmet armour}") {
            self.armour_type = ArmourType::Helmet;
        } else if body.contains("{shield armour}") {
            self.armour_type = ArmourType::Shield;
        } else if body.contains("{gloves armour}") {
            self.armour_type = ArmourType::Gloves;
        }

        // Base armour rating
        let temp = extract_param(&body, "Base armour rating: ", &vec![' ', '\\']);
        self.base_rating = match temp {
            None => None,
            Some(temp) if temp.parse::<i32>().is_ok() => Some(temp.parse::<i32>().unwrap()),
            _ => None,
        };

        // Encumbrance rating
        let temp = extract_param(&body, "Encumbrance rating: ", &vec![' ', '\\']);
        self.encumbrance = match temp {
            None => None,
            Some(temp) if temp.parse::<i32>().is_ok() => Some(temp.parse::<i32>().unwrap()),
            _ => None,
        };

        // Useless item
        if body.contains("useless_item") {
            self.useless = true
        }

        self.rating = self.rate_armour();
    }

    pub(crate) fn rate_armour(&mut self) -> i32 {
        let mut rating = self.base_rating.unwrap_or(-100);
        if self.useless {
            rating = -100;
        }

        rating
    }
}
