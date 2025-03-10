use rustc_hash::FxHashMap;

use crate::CrawlData;

/// Meta menu struct, stores the main menu that creates a menu hierarchy
#[derive(Debug)]
pub(crate) struct Menus {
    /// Entry point to the `Menu` object
    pub(crate) main_menu: Menu,
}

/// State of the menu, and actions to be taken - since multiple menu
/// can be on top of each other in DCSS, the `subs` item is a hierarchy
/// of menu. Deeper means opened later (should be closed first)
#[derive(Debug)]
pub(crate) struct Menu {
    /// Key to open the menu (also acts as hierarchy key)
    pub(crate) open_menu: String,
    /// Key to close the menu
    pub(crate) close_menu: String,
    /// Is the menu requested (should it be opened)
    pub(crate) requested: bool,
    /// Is the menu opened
    pub(crate) opened: bool,
    /// Is the menu closed
    pub(crate) closed: bool,
    /// Is the menu item a high priority (more important than others)
    pub(crate) high_priority: bool,
    /// Message to wait for from the API to confirm the menu is opened
    pub(crate) open_message: String,
    /// Message to wait for from the API to confirm the menu is closed
    pub(crate) close_message: String,
    /// Hashmap to sub-menus
    pub(crate) subs: FxHashMap<String, Menu>,
}

impl Menus {
    /// Create the top level menu
    pub(crate) fn init() -> Self {
        Self {
            main_menu: Menu::blank(),
        }
    }

    /// Add menu to the hierarchy, based on the hierarchy key
    ///
    /// # Arguments
    ///
    /// * `hierarchy` - A [Vec] of [&str] that takes the "open_menu" key to
    ///                 identify where in the menu hierarchy the new menu
    ///                 should be placed.
    /// * `open_menu` - A [&str] for the character used to open that menu
    /// * `close_menu` - A [&str] for the character used to close that menu
    /// * `requested` - A [bool] for if the menu was requested
    /// * `opened` - A [bool] for if the menu was opened
    /// * `closed` - A [bool] for if the menu was closed
    /// * `high_priority` - A [bool] for if the menu is of high priority
    /// * `open_message` - A [String] message to wait for from the API to
    ///                    confirm the menu is opened
    /// * `close_message` - A [String] message to wait for from the API to
    ///                     confirm the menu is closed
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn add_menu(
        &mut self,
        hierarchy: Vec<&str>,
        open_menu: &str,
        close_menu: &str,
        requested: bool,
        opened: bool,
        closed: bool,
        high_priority: bool,
        open_message: &str,
        close_message: &str,
    ) {
        let mut menu = &mut self.main_menu;
        for h_key in hierarchy {
            if h_key.is_empty() {
                continue;
            }
            menu = menu.subs.get_mut(h_key).unwrap();
        }

        if !menu.subs.contains_key(open_menu) {
            menu.subs.insert(
                open_menu.to_owned(),
                Menu {
                    open_menu: open_menu.to_owned(),
                    close_menu: close_menu.to_owned(),
                    requested,
                    opened,
                    closed,
                    high_priority,
                    open_message: open_message.to_owned(),
                    close_message: close_message.to_owned(),
                    subs: FxHashMap::default(),
                },
            );
        } else {
            menu.subs.get_mut(open_menu).unwrap().closed = false;
        }
    }

    /// Get the current, by prioritizing opened menus, followed by
    /// high_priority menus, followed by deep menus (as long as the
    /// shallower menus are opened). The deeper a menu is the higher
    /// priority it should get, because it means that multiple menus
    /// are currently opened and the deepest one is the latest one
    /// that is opened or needs to be opened (e.g. need to close the
    /// deepest one before closing the less deep one).
    pub(crate) fn current_menu(&mut self) -> &mut Menu {
        /// Returns a path (using the key to open the menu) to the
        /// deepest menu (priority to open, high_priority and deep)
        fn find_deepest_menu(menu: &Menu, path: &mut Vec<String>) -> Vec<String> {
            if !menu.subs.is_empty() && menu.opened {
                // Prioritize opened
                for sub_key in menu.subs.keys() {
                    if menu.subs[sub_key].opened {
                        path.push(menu.subs[sub_key].open_menu.clone());
                        return find_deepest_menu(&menu.subs[sub_key], path);
                    }
                }
                // Prioritize high_priority
                for sub_key in menu.subs.keys() {
                    if menu.subs[sub_key].high_priority {
                        path.push(menu.subs[sub_key].open_menu.clone());
                        return find_deepest_menu(&menu.subs[sub_key], path);
                    }
                }
                // Any other submenu
                if let Some(sub_key) = menu.subs.keys().next() {
                    path.push(menu.subs[sub_key].open_menu.clone());
                    return find_deepest_menu(&menu.subs[sub_key], path);
                }
            }

            path.to_vec()
        }

        let hierarchy = find_deepest_menu(&self.main_menu, &mut vec![]);

        // Follow the open_menu key path until you return the
        // deepest menu object
        let mut menu = &mut self.main_menu;
        for h_key in &hierarchy {
            if h_key.is_empty() {
                continue;
            }
            menu = menu.subs.get_mut(h_key).unwrap();
        }

        menu
    }

    /// Identify the current menu as "opened"
    pub(crate) fn identify_menu_as_opened(&mut self) {
        let menu = self.current_menu();
        menu.opened = true;
        menu.requested = false;
    }

    /// Delete all menus that are identified as "closed"
    pub(crate) fn remove_closed_menus(&mut self) {
        fn iterate_through_menus(menu: &Menu, hierarchy: Vec<String>) -> Vec<Vec<String>> {
            let mut to_delete = vec![];
            if !menu.subs.is_empty() {
                for sub_key in menu.subs.keys() {
                    if menu.subs[sub_key].closed {
                        let mut delete_hierarchy = hierarchy.clone();
                        delete_hierarchy.push(sub_key.to_owned());
                        to_delete.push(delete_hierarchy)
                    } else {
                        let mut new_hierarchy = hierarchy.clone();
                        new_hierarchy.push(sub_key.to_owned());
                        let mut hierarchies =
                            iterate_through_menus(menu.subs.get(sub_key).unwrap(), new_hierarchy);
                        to_delete.append(&mut hierarchies);
                    }
                }
            }

            to_delete
        }

        let hierarchies = iterate_through_menus(&self.main_menu, vec![]);
        for hierarchy in hierarchies {
            let mut menu = &mut self.main_menu;
            for h_key in &hierarchy[0..hierarchy.len() - 1] {
                menu = menu.subs.get_mut(h_key).unwrap();
            }
            menu.subs.remove(&hierarchy[hierarchy.len() - 1]);
        }
    }
}

impl Menu {
    /// Create blank menu -- consider it opened
    pub(crate) fn blank() -> Self {
        Self {
            open_menu: "".to_owned(),
            close_menu: "".to_owned(),
            requested: false,
            opened: true,
            closed: false,
            high_priority: false,
            open_message: "".to_owned(),
            close_message: "".to_owned(),
            subs: FxHashMap::default(),
        }
    }
}

impl CrawlData {
    /// Provides a [bool] of if a menu is ready to be processed.
    pub fn menu_to_process(&mut self) -> bool {
        !self.menus.main_menu.subs.is_empty()
    }

    /// Collect the current menu, according to the menu order rules
    /// set out in [menus.current_menu] and select the next action
    /// (opening, closing, etc) and what message the API should
    /// return if the action is correct.
    pub fn interact_with_menu(&mut self) -> (String, String) {
        let menu = self.menus.current_menu();
        if !menu.opened {
            menu.requested = true;
            let key = menu.open_menu.clone();
            let message = menu.open_message.clone();
            return (key, message);
        }

        menu.closed = true;

        let key = menu.close_menu.clone();
        let message = menu.close_message.clone();

        (key, message)
    }

    /// Identify all closed menus, and delete them from the menus.
    pub fn remove_closed_menus(&mut self) {
        self.menus.remove_closed_menus();
    }

    /// Identify the current menu as "opened"
    pub fn identify_menu_as_opened(&mut self) {
        self.menus.identify_menu_as_opened();
    }

    /// Create a menu to collect "ability" data
    pub fn queue_collect_ability_data(&mut self) {
        self.menus.add_menu(
            vec![""],
            "a",
            "key_esc",
            false,
            false,
            false,
            true,
            "menu",
            "close_menu",
        );
    }

    /// Open the description menu of a specific item
    pub fn queue_collect_item_data(&mut self, item_key: &str) {
        self.menus.add_menu(
            vec![""],
            "i",
            "key_esc",
            false,
            false,
            false,
            false,
            "menu",
            "close_menu",
        );

        self.menus.add_menu(
            vec!["i"],
            item_key,
            "key_esc",
            false,
            false,
            false,
            false,
            "ui-push",
            "ui-pop",
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_menu(
        &mut self,
        hierarchy: Vec<&str>,
        open_menu: &str,
        close_menu: &str,
        requested: bool,
        opened: bool,
        closed: bool,
        high_priority: bool,
        open_message: &str,
        close_message: &str,
    ) {
        self.menus.add_menu(
            hierarchy,
            open_menu,
            close_menu,
            requested,
            opened,
            closed,
            high_priority,
            open_message,
            close_message,
        );
    }

    pub fn queue_drop_item(&mut self, item_key: &str) {
        self.menus.add_menu(
            vec![""],
            "d",
            "key_enter",
            false,
            false,
            false,
            false,
            "menu",
            "player",
        );

        self.menus.add_menu(
            vec!["d"],
            item_key,
            "",
            false,
            false,
            false,
            false,
            "update_menu",
            "",
        );
    }

    pub fn queue_wield_wear(&mut self, item_key: &str) {
        self.menus
            .add_menu(vec![""], "i", "", false, false, false, false, "menu", "");

        self.menus.add_menu(
            vec!["i"],
            item_key,
            "w",
            false,
            false,
            false,
            false,
            "ui-push",
            "player",
        );
    }

    pub fn queue_put_on(&mut self, item_key: &str) {
        self.menus
            .add_menu(vec![""], "i", "", false, false, false, false, "menu", "");

        self.menus.add_menu(
            vec!["i"],
            item_key,
            "p",
            false,
            false,
            false,
            false,
            "ui-push",
            "player",
        );
    }

    pub fn queue_read_scroll(&mut self, item_key: &str) {
        self.menus.add_menu(
            vec![""],
            "r",
            item_key,
            false,
            false,
            false,
            false,
            "menu_scroll",
            "player",
        );
    }

    pub fn queue_quaff_potion(&mut self, item_key: &str) {
        self.menus.add_menu(
            vec![""],
            "q",
            item_key,
            false,
            false,
            false,
            false,
            "menu_scroll",
            "player",
        );
    }

    pub fn queue_close_all_menus(&mut self, item_key: &str) {
        self.menus.add_menu(
            vec![""],
            "x",
            item_key,
            false,
            true,
            false,
            true,
            "",
            "close_all_menus",
        );
    }

    pub fn queue_use_ability(&mut self, ability_key: &str) {
        self.menus.add_menu(
            vec![""],
            "a",
            ability_key,
            false,
            false,
            false,
            false,
            "menu",
            "player",
        );
    }

    pub fn queue_collect_known_item_data(&mut self) {
        self.menus.add_menu(
            vec![""],
            "\\",
            "key_esc",
            false,
            false,
            false,
            true,
            "menu",
            "close_menu",
        );
    }

    pub fn queue_collect_skills_data(&mut self) {
        self.menus.add_menu(
            vec![""],
            "m",
            "key_esc",
            false,
            false,
            false,
            true,
            "txt",
            "close_menu",
        );
    }

    pub fn queue_pickup_all(&mut self) {
        self.menus.add_menu(
            vec![""],
            "*",
            "key_enter",
            false,
            false,
            false,
            true,
            "update_menu",
            "close_menu",
        );
    }

    pub fn look_at_monster_menu(&mut self) {
        self.menus.add_menu(
            vec![""],
            "monster",
            "key_esc",
            false,
            true,
            false,
            true,
            "",
            "close_all_menus",
        );
    }

    pub fn queue_select_monster(&mut self) {
        self.menus.add_menu(
            vec!["monster"],
            "a",
            "key_esc",
            true,
            false,
            false,
            true,
            "ui-push",
            "ui-pop",
        );
    }

    pub fn queue_identify_item(&mut self, identify_scroll: &str, item_to_identify: &str) {
        self.menus.add_menu(
            vec![""],
            "r",
            "",
            false,
            false,
            false,
            false,
            "menu",
            "close_all_menus",
        );

        self.menus.add_menu(
            vec!["r"],
            identify_scroll,
            item_to_identify,
            false,
            false,
            false,
            false,
            "menu",
            "",
        );
    }
}
