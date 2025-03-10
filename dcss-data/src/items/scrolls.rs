use crate::common::extract_param;
use serde_json::Value;

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
    pub(crate) identified: bool,
    pub(crate) scroll_type: ScrollType,
}

impl Scroll {
    pub(crate) fn new() -> Self {
        Self {
            data_collected: false,
            identified: false,
            scroll_type: ScrollType::Unknown,
        }
    }

    pub(crate) fn update_scroll(&mut self, description: Value) {
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

        if let Some(scroll_type) = extract_param(
            description["title"].as_str().unwrap(),
            "scroll of ",
            &vec!['.', '{'],
        ) {
            self.scroll_type = type_of_scroll(scroll_type.trim_end().to_owned());
        } else {
            self.scroll_type = type_of_scroll(
                extract_param(
                    description["title"].as_str().unwrap(),
                    "scrolls of ",
                    &vec!['.', '{'],
                )
                .unwrap()
                .trim_end()
                .to_owned(),
            );
        }
    }

    pub(crate) fn update_scroll_values(&mut self, name: &str, _quantity: u64) {
        let mut scroll_type = extract_param(name, "scroll of ", &vec!['\n', '{']);
        if scroll_type.is_none() {
            scroll_type = extract_param(name, "scrolls of ", &vec!['.', '{']);
        }

        if let Some(pt) = scroll_type {
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
