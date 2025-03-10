use crate::common::extract_param;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PotionType {
    Unknown,
    Berserk,
    Haste,
    Experience,
    Enlightenment,
    Might,
    Resistance,
    Attraction,
    Brilliance,
    HealWounds,
    Degeneration,
    Lignification,
    Curing,
    Invisibility,
    Cancellation,
    Mutation,
    Ambrosia,
    Magic,
}

#[derive(Clone, Debug)]
pub(crate) struct Potion {
    pub(crate) data_collected: bool,
    pub(crate) identified: bool,
    pub(crate) potion_type: PotionType,
}

impl Potion {
    pub(crate) fn new() -> Self {
        Self {
            data_collected: false,
            identified: false,
            potion_type: PotionType::Unknown,
        }
    }

    pub(crate) fn update_potion(&mut self, description: Value) {
        self.data_collected = true;

        if description["body"]
            .as_str()
            .unwrap()
            .contains(" identified ")
        {
            self.identified = true;
        } else {
            return;
        }

        if let Some(potion_type) = extract_param(
            description["title"].as_str().unwrap(),
            "potion of ",
            &vec!['.', '{'],
        ) {
            self.potion_type = type_of_potion(potion_type.trim_end().to_owned());
        } else {
            self.potion_type = type_of_potion(
                extract_param(
                    description["title"].as_str().unwrap(),
                    "potions of ",
                    &vec!['.', '{'],
                )
                .unwrap()
                .trim_end()
                .to_owned(),
            );
        }
    }

    pub(crate) fn update_potion_values(&mut self, name: &str, _quantity: u64) {
        let mut potion_type = extract_param(name, "potion of ", &vec!['\n', '{']);
        if potion_type.is_none() {
            potion_type = extract_param(name, "potions of ", &vec!['.', '{', '\n']);
        }

        if let Some(pt) = potion_type {
            self.identified = true;
            self.potion_type = type_of_potion(pt.trim_end().to_owned());
        }
    }
}

pub(crate) fn type_of_potion(potion_type: String) -> PotionType {
    match &potion_type[..] {
        "berserk rage" => PotionType::Berserk,
        "haste" => PotionType::Haste,
        "experience" => PotionType::Experience,
        "enlightenment" => PotionType::Enlightenment,
        "might" => PotionType::Might,
        "resistance" => PotionType::Resistance,
        "attraction" => PotionType::Attraction,
        "brilliance" => PotionType::Brilliance,
        "heal wounds" => PotionType::HealWounds,
        "degeneration" => PotionType::Degeneration,
        "lignification" => PotionType::Lignification,
        "curing" => PotionType::Curing,
        "invisibility" => PotionType::Invisibility,
        "cancellation" => PotionType::Cancellation,
        "mutation" => PotionType::Mutation,
        "ambrosia" => PotionType::Ambrosia,
        "magic" => PotionType::Magic,
        _ => {
            unreachable!();
        }
    }
}
