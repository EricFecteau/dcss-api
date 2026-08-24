use rustc_hash::FxHashMap;
use serde_json::Value;

use crate::CrawlData;
use crate::common::{ascii_to_letter, extract_param};
use crate::items::{Item, jewellery};

use crate::items::armours::Armour;
use crate::items::jewellery::Jewellery;
use crate::items::missiles::Missile;
use crate::items::potions::{Potion, type_of_potion};
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
            items: vec![Item::None; 127],
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
            (String::from("moonshine"), false),
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
                let letter = item["letter"].as_i64();
                self.init_item(index, letter, it as i32);
            }

            if item.as_object().unwrap().contains_key("name") {
                let name = item.as_object().unwrap()["name"].as_str().unwrap_or("");
                let quantity_value = item.as_object().unwrap().get("quantity");
                let quantity = if quantity_value.is_none() {
                    None
                } else {
                    quantity_value.unwrap().as_i64()
                };
                let letter = item["letter"].as_i64();
                self.update_item(index, name, letter, quantity);
            }
        }
    }

    pub(crate) fn init_item(&mut self, index: usize, letter: Option<i64>, item_type: i32) {
        self.items[index] = match item_type {
            0 => Item::Weapon(Weapon::new(ascii_to_letter(letter.unwrap() as usize))),
            1 => Item::Missile(Missile::new(ascii_to_letter(letter.unwrap() as usize))),
            2 => Item::Armour(Armour::new(ascii_to_letter(letter.unwrap() as usize))),
            3 => Item::Wand(Wand::new(ascii_to_letter(letter.unwrap() as usize))),
            4 => unimplemented!(),
            5 => Item::Scroll(Scroll::new(ascii_to_letter(letter.unwrap() as usize))),
            6 => Item::Jewellery(Jewellery::new(ascii_to_letter(letter.unwrap() as usize))),
            7 => Item::Potion(Potion::new(ascii_to_letter(letter.unwrap() as usize))),
            8 => unimplemented!(),
            9 => Item::Staff(Staff::new(ascii_to_letter(letter.unwrap() as usize))),
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

    pub(crate) fn update_item(
        &mut self,
        index: usize,
        name: &str,
        letter: Option<i64>,
        quantity: Option<i64>,
    ) {
        match &mut self.items[index] {
            Item::None => (),
            Item::Weapon(_) => (),
            Item::Missile(_) => (),
            Item::Armour(_) => (),
            Item::Wand(_) => (),
            Item::_Unknown4 => unimplemented!(),
            Item::Scroll(item) => item.update_scroll_values(name, letter, quantity),
            Item::Jewellery(_) => (),
            Item::Potion(item) => item.update_potion_values(name, letter, quantity),
            Item::_Unknown8 => unimplemented!(),
            Item::Staff(_) => (),
        }
    }

    pub(crate) fn gear_letter_to_index(&self, letter: char) -> usize {
        for (index, item) in self.items.iter().enumerate() {
            if item.is_none() {
                continue;
            }

            if item.letter() == letter {
                return index;
            }
        }

        unreachable!("Should only be able to get index for known letters.")
    }

    pub(crate) fn description(&mut self, description: &Value) {
        // Ignore descriptions of scrolls and potions, they provide no useful info
        let title = description["title"].to_string();
        if title.contains(" scroll ")
            || title.contains(" scrolls ")
            || title.contains(" potion ")
            || title.contains(" potions ")
        {
            return;
        }

        let letter = title[1..2].chars().next().unwrap();
        let index = self.gear_letter_to_index(letter);
        match &mut self.items[index] {
            Item::None => (),
            Item::Weapon(item) => item.update_weapon(description),
            Item::Missile(item) => item.data_collected = true,
            Item::Armour(item) => item.update_armour(description),
            Item::Wand(item) => item.data_collected = true,
            Item::_Unknown4 => unimplemented!(),
            Item::Scroll(_) => unimplemented!("Should have been ignored."),
            Item::Jewellery(item) => item.update_jewellery(description),
            Item::Potion(_) => unimplemented!("Should have been ignored."),
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

    pub(crate) fn item_type(&self, item_index: usize) -> String {
        match self.items[item_index] {
            Item::None => "None".to_owned(),
            Item::Weapon(_) => "Weapon".to_owned(),
            Item::Missile(_) => "Missile".to_owned(),
            Item::Armour(_) => "Armour".to_owned(),
            Item::Wand(_) => "Wand".to_owned(),
            Item::_Unknown4 => unimplemented!(),
            Item::Scroll(_) => "Scroll".to_owned(),
            Item::Jewellery(_) => "Jewellery".to_owned(),
            Item::Potion(_) => "Potion".to_owned(),
            Item::_Unknown8 => unimplemented!(),
            Item::Staff(_) => "Staff".to_owned(),
        }
    }

    pub(crate) fn item_quantity(&self, item_index: usize) -> u64 {
        match &self.items[item_index] {
            Item::None => 0,
            Item::Weapon(_) => 1,
            Item::Missile(_) => 1,
            Item::Armour(_) => 1,
            Item::Wand(_) => 1,
            Item::_Unknown4 => unimplemented!(),
            Item::Scroll(scroll) => scroll.quantity,
            Item::Jewellery(_) => 1,
            Item::Potion(potion) => potion.quantity,
            Item::_Unknown8 => unimplemented!(),
            Item::Staff(_) => 1,
        }
    }

    pub(crate) fn get_ring_index_from_name(&self, ring_name: String) -> i32 {
        let ring_type = jewellery::ring_type_from_name(ring_name);

        for index in 0..127 {
            // If not identified, ignore
            if self.items[index].is_none() {
                continue;
            }

            // Skip all the wrong item types
            if self.item_type(index) != "Jewellery" {
                continue;
            }

            match &self.items[index] {
                Item::Jewellery(jewellery) => {
                    if jewellery.ring_type == ring_type {
                        return index as i32;
                    }
                }
                _ => unreachable!(),
            }
        }

        unreachable!()
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

    // TODO: Doing potion letter and scroll letter using a different method (using PotionType vs using String)

    pub fn potion_letter(&self, potion_type: &str) -> Option<char> {
        let potion_type_enum = type_of_potion(potion_type.to_owned());

        for index in 0..127 {
            if self.item_is_none(index) || !self.item_data_collected(index) {
                continue;
            }

            if self.item_type(index) != "Potion" {
                continue;
            }

            if self.inventory.items[index].potion_type() == potion_type_enum {
                return Some(self.inventory.items[index].letter());
            }
        }

        None
    }

    pub fn scroll_letter(&self, potion_type: &str) -> Option<char> {
        if !self.known_scroll(potion_type) {
            return None;
        }

        for index in 0..127 {
            if self.item_is_none(index) || !self.item_data_collected(index) {
                continue;
            }

            if self.item_type(index) != "Scroll" {
                continue;
            }

            if self.scroll_type(index) == potion_type {
                return Some(self.inventory.items[index].letter());
            }
        }

        None
    }

    pub fn item_letter_from_index(&self, index: usize) -> char {
        self.inventory.items[index].letter()
    }

    pub fn unknown_item(&self, item_type: &str) -> Option<char> {
        let mut item = None;
        let mut quantity = 0;

        for index in 0..127 {
            if self.item_is_none(index) {
                continue;
            }

            let curr_item_type = self.item_type(index);
            let curr_item_quant = self.item_quantity(index);
            if curr_item_type == item_type
                && !self.item_is_identified(index)
                && curr_item_quant > quantity
            {
                item = Some(self.item_letter_from_index(index));
                quantity = curr_item_quant
            }
        }

        item
    }

    pub fn item_type(&self, item_index: usize) -> String {
        self.inventory.item_type(item_index)
    }

    pub fn item_quantity(&self, item_index: usize) -> u64 {
        self.inventory.item_quantity(item_index)
    }
}
