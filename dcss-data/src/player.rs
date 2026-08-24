use crate::CrawlData;
use crate::common::AbsCoord;
use crate::common::add_i32_to_usize;
use crate::inventory::Inventory;
use crate::items::Item;
use crate::items::armours::ArmourType;
use crate::items::jewellery::JewelleryType;
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
    pub(crate) str: i32,
    pub(crate) int: i32,
    pub(crate) dex: i32,
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
    pub(crate) rings: (i32, i32), // TODO: Add 8 rings for octopods
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
            rings: (-1, -1),
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
            self.stats.str = message["str"].as_u64().unwrap() as i32;
        }

        if message_obj.contains_key("int") {
            self.stats.int = message["int"].as_u64().unwrap() as i32;
        }

        if message_obj.contains_key("dex") {
            self.stats.dex = message["dex"].as_u64().unwrap() as i32;
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
            Item::Jewellery(jewellery) => match &jewellery.jewellery_type {
                JewelleryType::Unknown => unreachable!("Unknown-type can not be worn."),
                JewelleryType::Amulet => self.equipped.amulet = item_index as i32,
                JewelleryType::Ring => self.equip_ring(item_index as i32),
            },
            Item::Potion(_) => unreachable!("Can't equip a potion"),
            Item::_Unknown8 => unimplemented!(),
            Item::Staff(_) => unimplemented!(),
        }
    }

    pub(crate) fn equipped_from_description(&mut self, inventory: &Inventory, description: &Value) {
        if !description["body"].to_string().contains("equipped") {
            return;
        }

        let letter = description["title"].to_string()[1..2]
            .chars()
            .next()
            .unwrap();
        let item_index = inventory.gear_letter_to_index(letter);

        self.update_equipped(item_index, inventory);
    }

    pub(crate) fn worn_ring(&self, index: i32) -> bool {
        if self.equipped.rings.0 == index || self.equipped.rings.1 == index {
            return true;
        }

        false
    }

    pub(crate) fn equip_ring(&mut self, index: i32) {
        if self.worn_ring(index) {
            return;
        }

        if self.equipped.rings.0 == -1 {
            self.equipped.rings.0 = index;
            return;
        }

        if self.equipped.rings.1 == -1 {
            self.equipped.rings.1 = index;
        }
    }

    pub(crate) fn remove_ring(&mut self, index: i32) {
        if self.equipped.rings.0 == index {
            self.equipped.rings.0 = -1;
            return;
        }

        if self.equipped.rings.1 == index {
            self.equipped.rings.1 = -1;
        }
    }
}

impl CrawlData {
    /// Get the player's current HP
    pub fn player_hp(&self) -> i32 {
        self.player.health.hp
    }

    /// Get the player's max HP
    pub fn player_hp_max(&self) -> i32 {
        self.player.health.hp_max
    }

    /// Get the player's poison HP (e.g. how much HP is left after poison has run it's course)
    pub fn player_poison_hp(&self) -> i32 {
        self.player.health.poison_survival
    }

    /// Get a bool if the player is lethally poisoned (poison survival == 0)
    pub fn lethally_poisoned(&self) -> bool {
        self.player.health.poison_survival <= 0
    }

    /// Get the player's AC
    pub fn player_ac(&self) -> i32 {
        self.player.defense.ac
    }

    /// Get the player's EV
    pub fn player_ev(&self) -> i32 {
        self.player.defense.ev
    }

    /// Get the player's SH
    pub fn player_sh(&self) -> i32 {
        self.player.defense.sh
    }

    /// Get the player's Str
    pub fn player_str(&self) -> i32 {
        self.player.stats.str
    }

    /// Get the player's Int
    pub fn player_int(&self) -> i32 {
        self.player.stats.int
    }

    /// Get the player's Dex
    pub fn player_dex(&self) -> i32 {
        self.player.stats.dex
    }

    /// Is ring (by index) already being worn
    pub fn worn_ring(&self, index: i32) -> bool {
        self.player.worn_ring(index)
    }

    pub fn remove_ring_by_name(&mut self, ring_name: String) {
        // TODO: Will this cause an issue if you have two of the same ring (wrong index removed)?

        let index = self.inventory.get_ring_index_from_name(ring_name);
        self.player.remove_ring(index)
    }

    /// Provide the index of newly adorned jewellery
    pub fn worn_jewellery(&mut self, letter: char) {
        let index = self.inventory.gear_letter_to_index(letter);

        match &self.inventory.items[index] {
            Item::Jewellery(jewellery) => match &jewellery.jewellery_type {
                JewelleryType::Unknown => unreachable!("Unknown-type should be impossible."),
                JewelleryType::Amulet => self.player.equipped.amulet = index as i32,
                JewelleryType::Ring => self.player.equip_ring(index as i32),
            },
            _ => unreachable!("Should never have '(worn)' on anything other than jewellery in log"),
        }

        // self.player.equip_ring
    }
}
