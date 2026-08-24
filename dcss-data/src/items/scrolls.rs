use crate::common::{ascii_to_letter, extract_param};

#[derive(Clone, Debug)]
pub(crate) enum ScrollType {
    Unknown,
    Acquirement,
    Amnesia,
    Blinking,
    BrandWeapon,
    EnchantArmour,
    EnchantWeapon,
    Fear,
    Fog,
    HolyWord,
    Immolation,
    Identify,
    Revelation,
    Noise,
    Silence,
    Summoning,
    Teleportation,
    Torment,
    Vulnerability,
    Poison,
    Butterflies,
}

#[derive(Clone, Debug)]
pub(crate) struct Scroll {
    pub(crate) data_collected: bool,
    pub(crate) letter: char,
    pub(crate) identified: bool,
    pub(crate) scroll_type: ScrollType,
    pub(crate) quantity: u64,
}

impl Scroll {
    pub(crate) fn new(letter: char) -> Self {
        Self {
            data_collected: false,
            letter,
            identified: false,
            scroll_type: ScrollType::Unknown,
            quantity: 0,
        }
    }

    pub(crate) fn update_scroll_values(
        &mut self,
        name: &str,
        letter: Option<i64>,
        quantity: Option<i64>,
    ) {
        let mut scroll_type = extract_param(name, "scroll of ", &vec!['\n', '{']);
        if scroll_type.is_none() {
            scroll_type = extract_param(name, "scrolls of ", &vec!['.', '{']);
        }

        if let Some(quantity) = quantity {
            self.quantity = quantity as u64;
        }

        if let Some(letter) = letter {
            self.letter = ascii_to_letter(letter as usize);
        }

        if let Some(pt) = scroll_type {
            self.data_collected = true;
            self.identified = true;
            self.scroll_type = type_of_scroll(pt.trim_end().to_owned());
        }
    }
}

pub(crate) fn type_of_scroll(scroll_type: String) -> ScrollType {
    match &scroll_type[..] {
        "acquirement" => ScrollType::Acquirement,
        "amnesia" => ScrollType::Amnesia,
        "blinking" => ScrollType::Blinking,
        "brand weapon" => ScrollType::BrandWeapon,
        "enchant armour" => ScrollType::EnchantArmour,
        "enchant weapon" => ScrollType::EnchantWeapon,
        "fear" => ScrollType::Fear,
        "fog" => ScrollType::Fog,
        "holy word" => ScrollType::HolyWord,
        "identify" => ScrollType::Identify,
        "immolation" => ScrollType::Immolation,
        "revelation" => ScrollType::Revelation,
        "noise" => ScrollType::Noise,
        "silence" => ScrollType::Silence,
        "summoning" => ScrollType::Summoning,
        "teleportation" => ScrollType::Teleportation,
        "torment" => ScrollType::Torment,
        "vulnerability" => ScrollType::Vulnerability,
        "poison" => ScrollType::Poison,
        "butterflies" => ScrollType::Butterflies,
        _ => {
            unreachable!();
        }
    }
}
