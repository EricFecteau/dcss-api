use crate::common::add_i32_to_usize;
use crate::common::AbsCoord;
use crate::CrawlData;
use serde_json::Value;

const MAX_FLOOR_SIZE: usize = 500;

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
pub(crate) struct Player {
    pub(crate) pos: AbsCoord,
    pub(crate) health: Health,
    pub(crate) stats: Stats,
    pub(crate) defense: Defense,
    pub(crate) equipped: Vec<i32>,
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

impl Player {
    pub(crate) fn init() -> Self {
        Self {
            pos: (MAX_FLOOR_SIZE / 2, MAX_FLOOR_SIZE / 2),
            health: Health::new(),
            stats: Stats::new(),
            defense: Defense::new(),
            equipped: vec![-1; 21],
            status: vec![],
        }
    }

    pub(crate) fn update_pos(&mut self, x: Value, y: Value) {
        let offset: usize = MAX_FLOOR_SIZE / 2;

        self.pos.0 = add_i32_to_usize(x.as_i64().unwrap() as i32, offset);
        self.pos.1 = add_i32_to_usize(y.as_i64().unwrap() as i32, offset);
    }

    pub(crate) fn update_equipped(&mut self, equipped: Value) {
        for (equip_index, item_index) in equipped.as_object().unwrap() {
            self.equipped[equip_index.parse::<usize>().unwrap()] =
                item_index.as_i64().unwrap() as i32;
        }
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
}

impl CrawlData {
    pub fn lethally_poisoned(&self) -> bool {
        self.player.health.poison_survival <= 0
    }
}
