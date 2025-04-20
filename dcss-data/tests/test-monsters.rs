mod common;

use dcss_api::Webtile;
use dcss_scenario_builder::start_game_with_scenario;

use dcss_data::CrawlData;

#[test]
fn no_monster() {
    let username = "Monsters1";
    let game_id = std::env::var("GAME_ID").unwrap_or("dcss-0.32".to_owned());
    common::reset_test(username, game_id.as_str());

    // Connect to DCSS Webtile
    let mut webtile = Webtile::connect("ws://localhost:8080/socket", 0, "0.32").unwrap();

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _ = webtile
        .login_with_credentials(username, "Password")
        .unwrap();

    // Start game with simple scenario.
    start_game_with_scenario(
        &mut webtile,
        game_id.as_str(),
        "b",
        "f",
        "b",
        "./tests/scenarios/tiles/box_7x7.yaml",
    )
    .unwrap();

    // Setup data object
    let mut data = CrawlData::init(9, "0.32");

    // Wait for Ready
    webtile
        .read_until("input_mode", Some("mode"), Some(1))
        .unwrap();

    // Process the data
    while let Some(message) = webtile.get_message() {
        data.process_json(&message).unwrap()
    }

    // Number of monsters that can be reached
    assert!(data.monster_count_path() == 0);

    // Number of monsters that can be seen
    assert!(data.monster_count_fov() == 0);

    // Test nearest_monster_path
    assert!(data.nearest_monster_path().is_empty());

    // Test coord_nearest_monster
    let coord = None;
    assert!(data.coord_nearest_monster() == coord);

    // Touching a monster
    assert!(!data.monster_touching());

    // Get threat vector
    assert!(data.get_monster_threat_vec().is_empty());

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn monster_from_tiles() {
    let username = "Monsters2";
    let game_id = std::env::var("GAME_ID").unwrap_or("dcss-0.32".to_owned());
    common::reset_test(username, game_id.as_str());

    // Connect to DCSS Webtile
    let mut webtile = Webtile::connect("ws://localhost:8080/socket", 0, "0.32").unwrap();

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _ = webtile
        .login_with_credentials(username, "Password")
        .unwrap();

    // Start game with simple scenario.
    start_game_with_scenario(
        &mut webtile,
        game_id.as_str(),
        "b",
        "f",
        "b",
        "./tests/scenarios/monsters/kobold.yaml",
    )
    .unwrap();

    // Setup data object
    let mut data = CrawlData::init(9, "0.32");

    // Wait for Ready
    webtile
        .read_until("input_mode", Some("mode"), Some(1))
        .unwrap();

    // Process the data
    while let Some(message) = webtile.get_message() {
        data.process_json(&message).unwrap()
    }

    // Number of monsters that can be reached
    assert!(data.monster_count_path() == 1);

    // Number of monsters that can be seen
    assert!(data.monster_count_fov() == 1);

    // Test nearest_monster_path
    let path = [(1, 1), (1, 1), (1, 1), (1, 0), (1, 0)];
    assert!(data.nearest_monster_path() == path.to_vec());

    // Test coord_nearest_monster
    let coord = (5, 3);
    assert!(data.coord_nearest_monster() == Some(coord));

    // Touching a monster
    assert!(!data.monster_touching());

    // Touching a monster
    assert!(!data.monster_touching());

    // Get threat vector
    assert!(data.get_monster_threat_vec() == [1]);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn glass_box() {
    let username = "Monsters3";
    let game_id = std::env::var("GAME_ID").unwrap_or("dcss-0.32".to_owned());
    common::reset_test(username, game_id.as_str());

    // Connect to DCSS Webtile
    let mut webtile = Webtile::connect("ws://localhost:8080/socket", 0, "0.32").unwrap();

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _ = webtile
        .login_with_credentials(username, "Password")
        .unwrap();

    // Start game with simple scenario.
    start_game_with_scenario(
        &mut webtile,
        game_id.as_str(),
        "b",
        "f",
        "b",
        "./tests/scenarios/monsters/glass_box.yaml",
    )
    .unwrap();

    // Setup data object
    let mut data = CrawlData::init(9, "0.32");

    // Wait for Ready
    webtile
        .read_until("input_mode", Some("mode"), Some(1))
        .unwrap();

    // Process the data
    while let Some(message) = webtile.get_message() {
        data.process_json(&message).unwrap()
    }

    // Number of monsters that can be reached
    assert!(data.monster_count_path() == 1);

    // Number of monsters that can be seen
    assert!(data.monster_count_fov() == 2);

    // Test nearest_monster_path
    let path = [(0, 1), (0, 1), (0, 1)];
    assert!(data.nearest_monster_path() == path.to_vec());

    // Test coord_nearest_monster
    let coord = (0, 3);
    assert!(data.coord_nearest_monster() == Some(coord));

    // Touching a monster
    assert!(!data.monster_touching());

    // Get threat vector
    assert!(data.get_monster_threat_vec() == [1, 1]);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn touching() {
    let username = "Monsters4";
    let game_id = std::env::var("GAME_ID").unwrap_or("dcss-0.32".to_owned());
    common::reset_test(username, game_id.as_str());

    // Connect to DCSS Webtile
    let mut webtile = Webtile::connect("ws://localhost:8080/socket", 0, "0.32").unwrap();

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _ = webtile
        .login_with_credentials(username, "Password")
        .unwrap();

    // Start game with simple scenario.
    start_game_with_scenario(
        &mut webtile,
        game_id.as_str(),
        "b",
        "f",
        "b",
        "./tests/scenarios/monsters/touching.yaml",
    )
    .unwrap();

    // Setup data object
    let mut data = CrawlData::init(9, "0.32");

    // Wait for Ready
    webtile
        .read_until("input_mode", Some("mode"), Some(1))
        .unwrap();

    // Process the data
    while let Some(message) = webtile.get_message() {
        data.process_json(&message).unwrap()
    }

    // Number of monsters that can be reached
    assert!(data.monster_count_path() == 1);

    // Number of monsters that can be seen
    assert!(data.monster_count_fov() == 1);

    // Test nearest_monster_path
    let path = [(1, 0)];
    assert!(data.nearest_monster_path() == path.to_vec());

    // Test coord_nearest_monster
    let coord = (1, 0);
    assert!(data.coord_nearest_monster() == Some(coord));

    // Touching a monster
    assert!(data.monster_touching());

    // Get threat vector
    assert!(data.get_monster_threat_vec() == [1]);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

// // Get mon in battle info
// assert!(data.get_battle_monster_info().is_empty());

// // Get mon touching info
// assert!(data.get_touching_monster_info().is_empty());
