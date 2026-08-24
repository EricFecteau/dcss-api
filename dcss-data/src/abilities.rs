use rustc_hash::FxHashMap;
use serde_json::Value;

use crate::CrawlData;

#[derive(Debug)]
pub(crate) struct Abilities {
    pub(crate) stale: bool,
    pub(crate) abilities: FxHashMap<String, Ability>,
}

#[derive(Debug)]
pub(crate) struct Ability {
    pub(crate) key: char,
    pub(crate) _cost: String,
    pub(crate) _failure_chance: u64,
}

impl Abilities {
    pub(crate) fn new() -> Self {
        Self {
            stale: false,
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
                let key = text[0].chars().next().unwrap();
                let name = text[0].split(" - ").collect::<Vec<&str>>()[1];
                let cost = text[1];
                let failure_chance = text[2].replace('%', "").parse::<u64>().unwrap();
                self.abilities
                    .insert(name.to_owned(), Ability::new(key, cost, failure_chance));
            }
        }
    }

    // For when abilities are used, piety will go down, and some may no longer
    // be available to use. Need to re-check.
    pub(crate) fn make_abilities_stale(&mut self) {
        self.stale = true;
    }

    pub(crate) fn make_abilities_current(&mut self) {
        self.stale = false;
    }
}

impl Ability {
    pub(crate) fn new(key: char, cost: &str, failure_chance: u64) -> Self {
        Self {
            key,
            _cost: cost.to_owned(),
            _failure_chance: failure_chance,
        }
    }
}

impl CrawlData {
    pub fn process_ability_menu(&mut self, menu_items: Value) {
        self.abilities.process_ability_menu(menu_items);
    }

    pub fn has_ability(&self, ability: &str) -> bool {
        self.abilities.abilities.contains_key(ability)
    }

    pub fn key_of_ability(&self, ability: &str) -> char {
        self.abilities.abilities[ability].key
    }

    pub fn abilities_stale(&self) -> bool {
        self.abilities.stale
    }
}
