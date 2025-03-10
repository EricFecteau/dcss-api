use rustc_hash::FxHashMap;
use serde_json::Value;

#[derive(Debug)]
pub(crate) struct Abilities {
    pub(crate) abilities: FxHashMap<String, Ability>,
}

#[derive(Debug)]
pub(crate) struct Ability {
    pub(crate) key: String,
    pub(crate) _cost: String,
    pub(crate) _failure_chance: u64,
}

impl Abilities {
    pub(crate) fn new() -> Self {
        Self {
            abilities: FxHashMap::default(),
        }
    }

    pub(crate) fn process_ability_menu(&mut self, menu_items: Value) {
        for item in menu_items.as_array().unwrap() {
            if item["level"] == 2 {
                let text = item["text"]
                    .as_str()
                    .unwrap()
                    .split("  ")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();
                let key = &text[0][0..1];
                let name = text[0].split(" - ").collect::<Vec<&str>>()[1];
                let cost = text[1];
                let failure_chance = text[2].replace('%', "").parse::<u64>().unwrap();
                self.abilities
                    .insert(name.to_owned(), Ability::new(key, cost, failure_chance));
            }
        }
    }
}

impl Ability {
    pub(crate) fn new(key: &str, cost: &str, failure_chance: u64) -> Self {
        Self {
            key: key.to_owned(),
            _cost: cost.to_owned(),
            _failure_chance: failure_chance,
        }
    }
}
