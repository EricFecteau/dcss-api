#![allow(dead_code)]

use dcss_api::Webtile;
use dcss_data::CrawlData;
use dcss_scenario_builder::start_game_with_scenario;
use serde_json::json;

pub(crate) fn reset_test(username: &str, game_id: &str) {
    // Connect to DCSS Webtile
    let mut webtile = Webtile::connect("ws://localhost:8080/socket", 0).expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials(username, "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game(game_id, "b", "f", "b")
        .expect("Failed to start game");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile.quit_game().expect("Failed to quit game");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile.disconnect().expect("Failed to disconnect");
}

pub(crate) fn setup_webtile(username: &str, scenario_file: &str) -> Webtile {
    let game_id = std::env::var("GAME_ID").unwrap_or("dcss-0.34".to_owned());
    reset_test(username, game_id.as_str());

    // Connect to DCSS Webtile
    let mut webtile = Webtile::connect("ws://localhost:8080/socket", 0).unwrap();

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _ = webtile
        .login_with_credentials(username, "Password")
        .unwrap();

    // Start game with simple scenario.
    start_game_with_scenario(&mut webtile, game_id.as_str(), "b", "f", "b", scenario_file).unwrap();

    // Wait for Ready
    webtile
        .read_until("input_mode", Some("mode"), Some(1))
        .unwrap();

    webtile
}

pub(crate) fn setup_data(webtile: &mut Webtile) -> CrawlData {
    // Setup data object
    let mut data = CrawlData::init(9);

    // Process the data
    while let Some(message) = webtile.get_message() {
        println!("RECEIVED: {}", &message);
        data.process_json(&message).unwrap()
    }

    data
}

/// Does the basic processing and menu management to look at a monster
pub(crate) fn examine_monster(coord: (i32, i32), webtile: &mut Webtile, data: &mut CrawlData) {
    // Send right click to Webtile
    let abs_coord = data.get_dcss_coord(coord.0, coord.1);

    let json = json!({
        "x":abs_coord.0,"y":abs_coord.1,"button":3,"msg":"click_cell"
    });

    webtile.write_json(json).unwrap();

    // Let `data` know you queried the monster
    data.ready_examine_monster(coord);

    // Read the menu that pops up about the monster
    data.look_at_monster_menu();
    webtile.read_until("ui-push", None, None).unwrap();
    manage_menu(webtile, data);

    // Process the data
    while let Some(message) = webtile.get_message() {
        data.process_json(&message).unwrap()
    }
}

/// Manage the menus stored in DCSS Data (opening, closing, etc) by
/// taking the key necessary for the next action, sending it to the
/// API and collecting API returned values until they expected
/// message is returned.
pub(crate) fn manage_menu(webtile: &mut Webtile, data: &mut CrawlData) {
    while data.menu_to_process() {
        // Collect the current menu, according to the menu order rules
        // and select the next action (opening, closing, etc) and what
        // the API should return if the action is correct.
        let (key, message) = data.interact_with_menu();
        if !key.is_empty() {
            webtile.write_key(&key[..]).unwrap();
        }
        if !message.is_empty() {
            webtile.read_until(&message[..], None, None).unwrap();
        }
        data.remove_closed_menus();
    }
}
