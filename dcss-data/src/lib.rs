//! A library to process data received from [DCSS Webtile](http://crawl.develz.org/wordpress/howto).

mod abilities;
mod common;
mod inventory;
mod items;
mod log;
mod menus;
mod monsters;
mod pickup;
mod player;
mod skills;
mod tiles;

pub use crate::common::move_adjacent_key;
pub use crate::common::RelCoord;

use crate::abilities::Abilities;
use crate::common::{convert_coords_to_relative, pathfinding, structured_table, AbsCoord};
use crate::inventory::Inventory;
use crate::items::armours::ArmourType;
use crate::items::jewellery::AmuletType;
use crate::items::scrolls::ScrollType;
use crate::items::{Item, WearSlots};
use crate::log::Log;
use crate::menus::Menus;
use crate::monsters::Monsters;
use crate::pickup::Pickup;
use crate::player::Player;
use crate::skills::Skills;
use crate::tiles::Tiles;
use std::error::Error;

use common::convert_coord_to_absolute;
use common::convert_coord_to_relative;
use serde_json::Value;

#[derive(Debug)]
pub struct CrawlData {
    /// Field of view (FOV) of the character for item pickup, battle mechanics,
    /// monsters, etc.
    fov: u32,
    /// Number of spectator currently watching.
    spectators: u64,
    /// TO COMMENT
    player: Player,
    /// TO COMMENT
    tiles: Tiles,
    /// TO COMMENT
    monsters: Monsters,
    /// TO COMMENT
    pickup: Pickup,
    /// TO COMMENT
    menus: Menus,
    /// TO COMMENT
    abilities: Abilities,
    /// TO COMMENT
    skills: Skills,
    /// TO COMMENT
    inventory: Inventory,
    /// TO COMMENT
    log: Log,
}

impl CrawlData {
    pub fn init(fov: u32, _version: &str) -> Self {
        Self {
            fov,
            spectators: 0,
            player: Player::init(),
            tiles: Tiles::init(),
            monsters: Monsters::init(),
            pickup: Pickup::init(),
            menus: Menus::init(),
            abilities: Abilities::new(),
            skills: Skills::new(),
            inventory: Inventory::new(),
            log: Log::init(),
        }
    }

    pub fn process_json(&mut self, message: &Value) -> Result<(), Box<dyn Error>> {
        let msg = message["msg"].as_str().unwrap();

        match msg {
            // Ignore
            "ping" => (),
            "lobby_clear" => (),
            "go_lobby" => (),
            "html" => (),
            "set_game_links" => (),
            "game_client" => (),
            "chat" => (),
            "version" => (),
            "options" => (),
            "layout" => (),
            "ui-state-sync" => (),
            "text_cursor" => (),
            "cursor" => (),
            "ui_state" => (),
            "flash" => (),
            "ui-stack" => (),
            "ui-state" => (),
            "update_menu_items" => (),
            "close_all_menus" => (),
            "delay" => (),
            "menu_scroll" => (),
            "ui-scroller-scroll" => (),
            "ui_cutoff" => (),
            "game_ended" => (),
            "init_input" => (),
            "close_input" => (),

            // Input & blocking
            "input_mode" => (),

            // Messages
            "msgs" => self.process_log(message)?,

            // Lobby
            "lobby_complete" => (),
            "login_success" => (),
            "game_started" => (),
            "update_spectators" => self.spectators = message["count"].as_u64().unwrap(),

            // Player
            "player" => self.update_player(message),

            // Dungeon
            "map" => self.update_map(message)?,

            // Menu
            "menu" => self.menus(message)?,
            "update_menu" => self.update_menu()?,
            "close_menu" => self.remove_closed_menus(),
            "ui-push" => self.ui_type(message)?,
            "ui-pop" => self.remove_closed_menus(),
            "txt" => self.process_skills(message),

            _ => {
                unreachable!("NOT IMPLEMENTED: {:?}", message);
            }
        };
        Ok(())
    }

    pub fn get_spectator_num(&self) -> u64 {
        self.spectators
    }

    fn menus(&mut self, message: &Value) -> Result<(), Box<dyn Error>> {
        if message["tag"] == "pickup" {
            let table = structured_table(message["items"].clone());

            if table.contains_key("Monsters") {
                self.queue_select_monster();
            } else {
                self.queue_pickup_all();
            }

            return Ok(());
        }

        self.identify_menu_as_opened();
        self.remove_closed_menus();

        if message["tag"] == "ability" {
            self.process_ability_menu(message["items"].clone());
        }

        if message["tag"] == "inventory"
            && message["title"]["text"]
                .as_str()
                .unwrap()
                .contains("Recognised items.")
        {
            self.process_known_item_menu(message["items"].clone());
        }

        Ok(())
    }

    fn update_menu(&mut self) -> Result<(), Box<dyn Error>> {
        // Menu was opened or action performed (same thing for this bot)
        self.identify_menu_as_opened();

        // Remove closed menus
        self.remove_closed_menus();

        Ok(())
    }

    fn ui_type(&mut self, message: &Value) -> Result<(), Box<dyn Error>> {
        match message["type"].as_str().unwrap() {
            "seed-selection" => (),
            "newgame-choice" => (),
            "describe-item" => {
                self.identify_menu_as_opened();
                self.inventory_description(message.clone());
            }
            "describe-monster" => {
                if self.monsters.examine_loc.is_none() {
                    panic!("You must set up `ready_examine_monster` before examining the monster.");
                }

                self.identify_menu_as_opened();
                self.monster_description(message.clone(), self.monsters.examine_loc.unwrap());
                self.monsters.examine_loc = None;
            }
            _ => (),
        }

        Ok(())
    }

    fn process_log(&mut self, message: &Value) -> Result<(), Box<dyn Error>> {
        // If no messages
        if !message.as_object().unwrap().contains_key("messages") {
            return Ok(());
        }

        // Send messages to the VecDeque
        for text_obj in message["messages"].as_array().unwrap() {
            let text = text_obj["text"].as_str().unwrap();

            if text.contains("You slide downwards.")
                || text.contains("You climb downwards.")
                || text.contains("You fall into a shaft")
            {
                self.new_floor();
            }

            self.log.log.push_back(text.to_owned())
        }

        Ok(())
    }

    pub fn update_map(&mut self, message: &Value) -> Result<(), Box<dyn Error>> {
        let extra_data = self.tiles.update(&message["cells"])?;

        for mon in extra_data.0 {
            self.monsters.update(mon.0, mon.1);
        }

        for ground_item_coord in extra_data.1 {
            self.pickup.update(ground_item_coord);
        }

        for invis_coord in extra_data.2 .0 {
            self.monsters.invisible_monster(invis_coord)
        }

        for invis_rm_coord in extra_data.2 .1 {
            self.monsters.invisible_removed(invis_rm_coord)
        }

        Ok(())
    }

    pub fn update_player(&mut self, message: &Value) {
        let message_obj = message.as_object().unwrap();

        if message_obj.contains_key("pos") {
            self.player
                .update_pos(message["pos"]["x"].clone(), message["pos"]["y"].clone());
        };

        if message_obj.contains_key("equip") {
            self.player.update_equipped(message["equip"].clone())
        }

        if message_obj.contains_key("status") {
            self.player.update_status(message["status"].clone());
        }

        if message_obj.contains_key("inv") {
            self.inventory.update_inventory(message["inv"].clone());
        }

        self.player.update_health(message);

        self.player.update_stats(message);

        self.player.update_defense(message);
    }

    pub fn get_player_status(&self) -> Vec<String> {
        self.player.status.clone()
    }

    pub fn index_of_equipped(&self, wear_type: &str) -> i32 {
        match wear_type {
            "Weapon" => self.player.equipped[WearSlots::Weapon as usize],
            "Amulet" => self.player.equipped[WearSlots::Amulet as usize],
            "Body" => self.player.equipped[WearSlots::Body as usize],
            "Boots" => self.player.equipped[WearSlots::Boots as usize],
            "Cloak" => self.player.equipped[WearSlots::Cloak as usize],
            "Helmet" => self.player.equipped[WearSlots::Helmet as usize],
            "Shield" => self.player.equipped[WearSlots::Shield as usize],
            "Gloves" => self.player.equipped[WearSlots::Gloves as usize],
            _ => unimplemented!("TODO"),
        }
    }

    pub fn item_type(&self, item_index: usize) -> String {
        match self.inventory.items[item_index] {
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

    pub fn item_rating(&self, item_index: usize) -> i32 {
        self.inventory.items[item_index].rating()
    }

    pub fn item_is_none(&self, item_index: usize) -> bool {
        self.inventory.items[item_index].is_none()
    }

    pub fn item_is_identified(&self, item_index: usize) -> bool {
        self.inventory.items[item_index].is_identified()
    }

    pub fn item_to_identified(&mut self, item_index: usize) {
        self.inventory.items[item_index].identified();
    }

    pub fn item_data_collected(&self, item_index: usize) -> bool {
        self.inventory.items[item_index].data_collected()
    }

    /// Not sure I understand
    pub fn set_scroll_of_identify_identifying(&mut self, val: bool) {
        self.inventory.identifying = val;
    }

    /// Not sure I understand
    pub fn read_scroll_of_identify_identifying(&mut self) -> bool {
        self.inventory.identifying
    }

    pub fn armour_type(&self, item_index: usize) -> String {
        match &self.inventory.items[item_index] {
            Item::Armour(armour) => match &armour.armour_type {
                ArmourType::None => "None".to_owned(),
                ArmourType::Body => "Body".to_owned(),
                ArmourType::Boots => "Boots".to_owned(),
                ArmourType::Cloak => "Cloak".to_owned(),
                ArmourType::Helmet => "Helmet".to_owned(),
                ArmourType::Shield => "Shield".to_owned(),
                ArmourType::Gloves => "Gloves".to_owned(),
            },
            _ => "".to_owned(),
        }
    }

    pub fn scroll_type(&self, item_index: usize) -> String {
        match &self.inventory.items[item_index] {
            Item::Scroll(scroll) => match &scroll.scroll_type {
                ScrollType::Identify => "Identify".to_owned(),
                _ => "".to_owned(),
            },
            _ => "".to_owned(),
        }
    }

    pub fn amulet_type(&self, item_index: usize) -> String {
        match &self.inventory.items[item_index] {
            Item::Jewellery(scroll) => match &scroll.amulet_type {
                AmuletType::Unknown => "Unknown".to_owned(),
                _ => "".to_owned(),
            },
            _ => "".to_owned(),
        }
    }

    pub fn process_known_item_menu(&mut self, known_item_menu: Value) {
        self.inventory.process_known_item_menu(known_item_menu);
    }

    pub fn inventory_description(&mut self, description: Value) {
        self.inventory.description(description);
    }

    pub fn monster_description(&mut self, description: Value, pos: AbsCoord) {
        self.monsters.description(description, pos);
    }

    pub fn process_ability_menu(&mut self, menu_items: Value) {
        self.abilities.process_ability_menu(menu_items);
    }

    pub fn has_ability(&self, ability: &str) -> bool {
        self.abilities.abilities.contains_key(ability)
    }

    pub fn has_status(&self, status: &str) -> bool {
        self.player.status.contains(&String::from(status))
    }

    pub fn key_of_ability(&self, ability: &str) -> String {
        self.abilities.abilities[ability].key.to_owned()
    }

    pub fn unknown_item(&self, item_type: &str) -> Option<usize> {
        let mut item = None;

        for index in 0..52 {
            if self.item_is_none(index) || !self.item_data_collected(index) {
                continue;
            }

            let curr_item_type = self.item_type(index);
            if curr_item_type == item_type && !self.item_is_identified(index) {
                item = Some(index);
            }
        }

        item
    }

    pub fn player_hp(&self) -> i32 {
        self.player.health.hp
    }

    pub fn player_hp_max(&self) -> i32 {
        self.player.health.hp_max
    }

    pub fn player_poison_hp(&self) -> i32 {
        self.player.health.poison_survival
    }

    pub fn player_defense(&self) -> (i32, i32, i32) {
        (
            self.player.defense.ac,
            self.player.defense.ev,
            self.player.defense.sh,
        )
    }

    pub fn fov_of_data(&mut self) -> u32 {
        self.fov
    }

    pub fn unknown_item_curr_loc(&mut self) -> bool {
        let pos = self.player.pos;
        self.pickup.unknown_item_loc(pos)
    }

    pub fn remove_item_curr_loc(&mut self) {
        let pos = self.player.pos;
        self.pickup.remove_item_loc(pos);
    }

    pub fn new_ignore_item_curr_loc(&mut self) {
        let pos = self.player.pos;
        self.pickup.new_ignore_item_loc(pos);
    }

    pub fn nearest_item_path(&mut self) -> Option<Vec<RelCoord>> {
        let pos = self.player.pos;
        let nearest = self.pickup.nearest(&self.tiles.tiles, pos, self.fov);

        if !nearest.is_empty() {
            let coords = convert_coords_to_relative(pos, nearest);
            return Some(coords);
        }

        None
    }

    pub fn examine_monsters(&mut self) -> Option<RelCoord> {
        let pos = self.player.pos;
        let coord = self.monsters.pos_unexamined_monster(pos, self.fov);

        coord.map(|coord| convert_coord_to_relative(pos, coord))
    }

    pub fn new_floor(&mut self) {
        self.tiles = Tiles::init();
        self.monsters = Monsters::init();
        self.pickup = Pickup::init();
    }

    /// Set path to an unexplored location in the [Tiles] object.
    ///
    /// # Arguments
    ///
    /// * `player_pos` - A [Coord] containing the player's position within
    ///   the [Tiles].tiles vector.
    pub fn path_to_unexplored(&mut self) -> Vec<RelCoord> {
        let coords = pathfinding(
            &self.tiles.tiles,
            self.player.pos,
            None,
            None,
            Some("unexplored"),
            10_000,
            false,
        );

        convert_coords_to_relative(self.player.pos, coords)
    }

    /// Set path to an down stair in the [Tiles] object.
    ///
    /// # Arguments
    ///
    /// * `player_pos` - A ([Coord]) containing the player's position within
    ///   the [Tiles].tiles vector.
    pub fn path_to_down_stairs(&mut self) -> Vec<RelCoord> {
        let coords = pathfinding(
            &self.tiles.tiles,
            self.player.pos,
            None,
            Some(13),
            None,
            10_000,
            false,
        );

        convert_coords_to_relative(self.player.pos, coords)
    }

    pub fn path_to_location(&mut self, cell_loc: RelCoord) -> Vec<RelCoord> {
        let cell_loc = convert_coord_to_absolute(self.player.pos, cell_loc);

        let coords = pathfinding(
            &self.tiles.tiles,
            self.player.pos,
            Some(cell_loc),
            Some(13),
            None,
            10_000,
            false,
        );

        convert_coords_to_relative(self.player.pos, coords)
    }
}
