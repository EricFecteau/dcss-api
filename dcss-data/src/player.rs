use crate::CrawlData;
use crate::common::AbsCoord;
use crate::common::add_i32_to_usize;
use crate::common::char_to_index;
use crate::inventory::Inventory;
use crate::items::Item;
use crate::items::armours::ArmourType;
use serde_json::Value;

use crate::MAX_FLOOR_SIZE;

#[derive(Debug)]
pub(crate) struct Health {
    pub(crate) hp: i32,
    pub(crate) hp_max: i32,
    pub(crate) poison_survival: i32,
}

#[derive(Debug)]
pub(crate) struct Stats {
    pub(crate) str: u32,
    pub(crate) int: u32,
    pub(crate) dex: u32,
}

#[derive(Debug)]
pub(crate) struct Defense {
    pub(crate) ac: i32,
    pub(crate) ev: i32,
    pub(crate) sh: i32,
}

#[derive(Debug)]
pub(crate) struct Equipped {
    pub(crate) weapon: i32,
    pub(crate) quiver: i32,
    pub(crate) amulet: i32,
    pub(crate) body: i32,
    pub(crate) boots: i32,
    pub(crate) cloak: i32,
    pub(crate) helmet: i32,
    pub(crate) shield: i32,
    pub(crate) gloves: i32,
    pub(crate) _ring_left: i32, // TODO: Add 8 rings for octopods
    pub(crate) _ring_right: i32,
}

#[derive(Debug)]
/// Stores the character's information
pub(crate) struct Player {
    pub(crate) pos: AbsCoord,
    pub(crate) health: Health,
    pub(crate) stats: Stats,
    pub(crate) defense: Defense,
    pub(crate) equipped: Equipped,
    pub(crate) status: Vec<String>,
}

impl Health {
    pub(crate) fn new() -> Self {
        Self {
            hp: 1,
            hp_max: 1,
            poison_survival: 1,
        }
    }
}

impl Stats {
    pub(crate) fn new() -> Self {
        Self {
            str: 0,
            int: 0,
            dex: 0,
        }
    }
}

impl Defense {
    pub(crate) fn new() -> Self {
        Self {
            ac: 0,
            ev: 0,
            sh: 0,
        }
    }
}

impl Equipped {
    pub(crate) fn new() -> Self {
        Self {
            weapon: -1,
            quiver: -1,
            amulet: -1,
            body: -1,
            boots: -1,
            cloak: -1,
            helmet: -1,
            shield: -1,
            gloves: -1,
            _ring_left: -1,
            _ring_right: -1,
        }
    }
}

impl Player {
    pub(crate) fn init() -> Self {
        Self {
            pos: (MAX_FLOOR_SIZE / 2, MAX_FLOOR_SIZE / 2),
            health: Health::new(),
            stats: Stats::new(),
            defense: Defense::new(),
            equipped: Equipped::new(),
            status: vec![],
        }
    }

    pub(crate) fn update_pos(&mut self, x: Value, y: Value) {
        let offset: usize = MAX_FLOOR_SIZE / 2;

        self.pos.0 = add_i32_to_usize(x.as_i64().unwrap() as i32, offset);
        self.pos.1 = add_i32_to_usize(y.as_i64().unwrap() as i32, offset);
    }

    pub(crate) fn update_health(&mut self, message: &Value) {
        let message_obj = message.as_object().unwrap();

        if message_obj.contains_key("hp") {
            self.health.hp = message["hp"].as_i64().unwrap() as i32;
        }

        if message_obj.contains_key("poison_survival") {
            self.health.poison_survival = message["poison_survival"].as_i64().unwrap() as i32;
        }

        if message_obj.contains_key("hp_max") {
            self.health.hp_max = message["hp_max"].as_i64().unwrap() as i32;
        }
    }

    pub(crate) fn update_stats(&mut self, message: &Value) {
        let message_obj = message.as_object().unwrap();

        if message_obj.contains_key("str") {
            self.stats.str = message["str"].as_u64().unwrap() as u32;
        }

        if message_obj.contains_key("int") {
            self.stats.int = message["int"].as_u64().unwrap() as u32;
        }

        if message_obj.contains_key("dex") {
            self.stats.dex = message["dex"].as_u64().unwrap() as u32;
        }
    }

    pub(crate) fn update_defense(&mut self, message: &Value) {
        let message_obj = message.as_object().unwrap();

        if message_obj.contains_key("ac") {
            self.defense.ac = message["ac"].as_i64().unwrap() as i32;
        }

        if message_obj.contains_key("ev") {
            self.defense.ev = message["ev"].as_i64().unwrap() as i32;
        }

        if message_obj.contains_key("sh") {
            self.defense.sh = message["sh"].as_i64().unwrap() as i32;
        }
    }

    pub(crate) fn update_status(&mut self, messages: Value) {
        // Always reset -- full list sent every time
        self.status = vec![];

        let val_array = messages.as_array().unwrap();

        for message in val_array {
            let message_obj = message.as_object().unwrap();

            if message_obj.contains_key("light") {
                self.status
                    .push(message_obj["light"].as_str().unwrap().to_owned())
            } else if message_obj.contains_key("text") {
                self.status
                    .push(message_obj["text"].as_str().unwrap().to_owned())
            }
        }
    }

    pub(crate) fn update_equipped(&mut self, item_index: usize, inventory: &Inventory) {
        match &inventory.items[item_index] {
            Item::None => unreachable!("None-type can not be equipped."),
            Item::Weapon(_) => self.equipped.weapon = item_index as i32,
            Item::Missile(_) => self.equipped.quiver = item_index as i32,
            Item::Armour(armour) => match &armour.armour_type {
                ArmourType::None => unreachable!("None-type can not be equipped."),
                ArmourType::Body => self.equipped.body = item_index as i32,
                ArmourType::Boots => self.equipped.boots = item_index as i32,
                ArmourType::Cloak => self.equipped.cloak = item_index as i32,
                ArmourType::Helmet => self.equipped.helmet = item_index as i32,
                ArmourType::Shield => self.equipped.shield = item_index as i32,
                ArmourType::Gloves => self.equipped.gloves = item_index as i32,
            },
            Item::Wand(_) => unimplemented!(),
            Item::_Unknown4 => unimplemented!(),
            Item::Scroll(_) => unreachable!("Can't equip a scroll"),
            Item::Jewellery(_) => unimplemented!(),
            Item::Potion(_) => unreachable!("Can't equip a potion"),
            Item::_Unknown8 => unimplemented!(),
            Item::Staff(_) => unimplemented!(),
        }
    }

    pub(crate) fn equipped_from_description(&mut self, inventory: &Inventory, description: &Value) {
        if !description["body"].to_string().contains("equipped") {
            return;
        }

        let key = &description["title"].to_string()[1..2];
        let item_index = char_to_index(key);

        self.update_equipped(item_index, inventory);
    }
}

impl CrawlData {
    pub fn lethally_poisoned(&self) -> bool {
        self.player.health.poison_survival <= 0
    }
}
