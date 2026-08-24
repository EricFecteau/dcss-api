use crate::common::{ascii_to_letter, extract_param};

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
    Moonshine,
}

#[derive(Clone, Debug)]
pub(crate) struct Potion {
    pub(crate) data_collected: bool,
    pub(crate) letter: char,
    pub(crate) identified: bool,
    pub(crate) potion_type: PotionType,
    pub(crate) quantity: u64,
}

impl Potion {
    pub(crate) fn new(letter: char) -> Self {
        Self {
            data_collected: false,
            letter,
            identified: false,
            potion_type: PotionType::Unknown,
            quantity: 0,
        }
    }

    pub(crate) fn update_potion_values(
        &mut self,
        name: &str,
        letter: Option<i64>,
        quantity: Option<i64>,
    ) {
        let mut potion_type = extract_param(name, "potion of ", &vec!['\n', '{']);
        if potion_type.is_none() {
            potion_type = extract_param(name, "potions of ", &vec!['.', '{', '\n']);
        }

        if let Some(quantity) = quantity {
            self.quantity = quantity as u64;
        }

        if let Some(letter) = letter {
            self.letter = ascii_to_letter(letter as usize);
        }

        if let Some(pt) = potion_type {
            self.data_collected = true;
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
        "moonshine" => PotionType::Moonshine,
        _ => {
            unreachable!();
        }
    }
}
