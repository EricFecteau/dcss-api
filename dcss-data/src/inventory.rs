use rustc_hash::FxHashMap;
use serde_json::Value;

use crate::common::{char_to_index, extract_param};
use crate::items::Item;
use crate::CrawlData;

use crate::items::armours::Armour;
use crate::items::jewellery::Jewellery;
use crate::items::missiles::Missile;
use crate::items::potions::{type_of_potion, Potion};
use crate::items::scrolls::Scroll;
use crate::items::staves::Staff;
use crate::items::wands::Wand;
use crate::items::weapons::Weapon;

#[derive(Debug, Clone)]
pub(crate) struct Inventory {
    pub(crate) items: Vec<Item>,
    pub(crate) known_scroll: FxHashMap<String, bool>,
    pub(crate) known_potion: FxHashMap<String, bool>,
    pub(crate) identifying: bool,
}

impl Inventory {
    pub(crate) fn new() -> Self {
        let mut inv = Self {
            items: vec![Item::None; 52],
            known_scroll: FxHashMap::default(),
            known_potion: FxHashMap::default(),
            identifying: false, // To not confuse it with the first read of scroll of identify;
        };

        let scroll_vec = vec![
            (String::from("acquirement"), false),
            (String::from("amnesia"), false),
            (String::from("blinking"), false),
            (String::from("brand weapon"), false),
            (String::from("enchant armour"), false),
            (String::from("enchant weapon"), false),
            (String::from("fear"), false),
            (String::from("fog"), false),
            (String::from("holy word"), false),
            (String::from("identify"), false),
            (String::from("immolation"), false),
            (String::from("revelation"), false),
            (String::from("noise"), false),
            (String::from("silence"), false),
            (String::from("summoning"), false),
            (String::from("teleportation"), false),
            (String::from("torment"), false),
            (String::from("vulnerability"), false),
            (String::from("poison"), false),
            (String::from("butterflies"), false),
        ];

        for scroll in scroll_vec {
            inv.known_scroll.insert(scroll.0, scroll.1);
        }

        let potion_vec = vec![
            (String::from("berserk rage"), false),
            (String::from("haste"), false),
            (String::from("experience"), false),
            (String::from("enlightenment"), false),
            (String::from("might"), false),
            (String::from("resistance"), false),
            (String::from("attraction"), false),
            (String::from("brilliance"), false),
            (String::from("heal wounds"), false),
            (String::from("degeneration"), false),
            (String::from("lignification"), false),
            (String::from("curing"), false),
            (String::from("invisibility"), false),
            (String::from("cancellation"), false),
            (String::from("mutation"), false),
            (String::from("ambrosia"), false),
            (String::from("magic"), false),
        ];

        for potion in potion_vec {
            inv.known_potion.insert(potion.0, potion.1);
        }

        inv
    }

    pub(crate) fn update_inventory(&mut self, inventory: Value) {
        for (i, item) in inventory.as_object().unwrap() {
            let index: usize = i.parse().unwrap();

            let item_type = item["base_type"].as_i64();
            if let Some(it) = item_type {
                self.init_item(index, it as i32);
            } else if item.as_object().unwrap().contains_key("name") {
                let name = item.as_object().unwrap()["name"].as_str().unwrap_or("");
                let quantity = 1; // TODO:
                self.update_item(index, name, quantity);
            }
        }
    }

    pub(crate) fn init_item(&mut self, index: usize, item_type: i32) {
        self.items[index] = match item_type {
            0 => Item::Weapon(Weapon::new()),
            1 => Item::Missile(Missile::new()),
            2 => Item::Armour(Armour::new()),
            3 => Item::Wand(Wand::new()),
            4 => unimplemented!(),
            5 => Item::Scroll(Scroll::new()),
            6 => Item::Jewellery(Jewellery::new()),
            7 => Item::Potion(Potion::new()),
            8 => unimplemented!(),
            9 => Item::Staff(Staff::new()),
            _ => Item::None,
        }
    }

    pub(crate) fn _reexamine_item(&mut self, index: usize) {
        match &mut self.items[index] {
            Item::None => (),
            Item::Weapon(item) => item.data_collected = false,
            Item::Missile(item) => item.data_collected = false,
            Item::Armour(item) => item.data_collected = false,
            Item::Wand(item) => item.data_collected = false,
            Item::_Unknown4 => unimplemented!(),
            Item::Scroll(item) => item.data_collected = false,
            Item::Jewellery(item) => item.data_collected = false,
            Item::Potion(item) => item.data_collected = false,
            Item::_Unknown8 => unimplemented!(),
            Item::Staff(item) => item.data_collected = false,
        }
    }

    pub(crate) fn update_item(&mut self, index: usize, name: &str, quantity: u64) {
        match &mut self.items[index] {
            Item::None => (),
            Item::Weapon(_) => (),
            Item::Missile(_) => (),
            Item::Armour(_) => (),
            Item::Wand(_) => (),
            Item::_Unknown4 => unimplemented!(),
            Item::Scroll(item) => item.update_scroll_values(name, quantity),
            Item::Jewellery(_) => (),
            Item::Potion(item) => item.update_potion_values(name, quantity),
            Item::_Unknown8 => unimplemented!(),
            Item::Staff(_) => (),
        }
    }

    pub(crate) fn description(&mut self, description: Value) {
        let key = &description["title"].to_string()[1..2];
        match &mut self.items[char_to_index(key)] {
            Item::None => (),
            Item::Weapon(item) => item.update_weapon(description),
            Item::Missile(item) => item.data_collected = true,
            Item::Armour(item) => item.update_armour(description),
            Item::Wand(item) => item.data_collected = true,
            Item::_Unknown4 => unimplemented!(),
            Item::Scroll(item) => item.update_scroll(description),
            Item::Jewellery(item) => item.update_jewellery(description),
            Item::Potion(item) => item.update_potion(description),
            Item::_Unknown8 => unimplemented!(),
            Item::Staff(item) => item.data_collected = true,
        }
    }

    pub(crate) fn process_known_item_menu(&mut self, menu_item: Value) {
        let table_array = menu_item.as_array().unwrap();

        let mut item_type = "";

        for item in table_array {
            let item_obj = item.as_object().unwrap();
            let level = item_obj["level"].as_u64().unwrap();

            if level == 1 && !item_type.is_empty() {
                item_type = "";
            }

            // Focus on scrolls
            if level == 1 && item_obj["text"].as_str().unwrap().contains("Scrolls") {
                item_type = "scroll";
                continue;
            }

            // Focus on potions
            if level == 1 && item_obj["text"].as_str().unwrap().contains("Potions") {
                item_type = "potion";
                continue;
            }

            if item_type == "scroll" {
                let text = item["text"].as_str().unwrap();
                let scroll_param = extract_param(text, "scrolls of ", &vec!['(']);
                let scroll_type;
                if let Some(param) = scroll_param {
                    scroll_type = param.trim_end().to_owned();
                } else {
                    continue;
                }
                *self.known_scroll.get_mut(&scroll_type).unwrap() = true;
            }

            if item_type == "potion" {
                let text = item["text"].as_str().unwrap();
                let potion_param = extract_param(text, "potions of ", &vec!['(']);
                let potion_type;
                if let Some(param) = potion_param {
                    potion_type = param.trim_end().to_owned();
                } else {
                    continue;
                }
                *self.known_potion.get_mut(&potion_type).unwrap() = true;
            }
        }
    }
}

impl CrawlData {
    pub fn known_scroll(&self, scroll_type: &str) -> bool {
        self.inventory.known_scroll[scroll_type]
    }

    pub fn known_potion(&self, potion_type: &str) -> bool {
        self.inventory.known_potion[potion_type]
    }

    /// Identify a specific scroll as "known"
    pub fn learn_scroll(&mut self, scroll_type: &str) {
        *self.inventory.known_scroll.get_mut(scroll_type).unwrap() = true;
    }

    /// Identify a specific potion as "known"
    pub fn learn_potion(&mut self, potion_type: &str) {
        *self.inventory.known_potion.get_mut(potion_type).unwrap() = true;
    }

    pub fn potion_index(&self, potion_type: &str) -> Option<usize> {
        let potion_type_enum = type_of_potion(potion_type.to_owned());

        for index in 0..52 {
            if self.item_is_none(index) || !self.item_data_collected(index) {
                continue;
            }

            let item_type = self.item_type(index);

            if item_type != "Potion" {
                continue;
            }

            if self.inventory.items[index].potion_type() == potion_type_enum {
                return Some(index);
            } else {
                continue;
            }
        }

        None
    }
}
