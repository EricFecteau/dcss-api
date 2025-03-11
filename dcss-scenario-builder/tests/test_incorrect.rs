mod common;

use dcss_api::Webtile;
use dcss_scenario_builder::start_game_with_scenario;

#[should_panic]
#[test]
fn verify_no_character() {
    let game_id = std::env::var("GAME_ID").unwrap();

    // Safe test -- login start game, quit, and then test
    common::reset_test("Username", game_id.as_str());

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login.");

    // Start game with simple scenario.
    start_game_with_scenario(
        &mut webtile,
        game_id.as_str(),
        "b",
        "i",
        "c",
        "./tests/test_scenarios/no_char.yaml",
    )
    .expect("Failed to start game with scenario");
}

#[should_panic]
#[test]
fn verify_too_wide() {
    let game_id = std::env::var("GAME_ID").unwrap();

    // Safe test -- login start game, quit, and then test
    common::reset_test("Username", game_id.as_str());

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login.");

    // Start game with simple scenario.
    start_game_with_scenario(
        &mut webtile,
        game_id.as_str(),
        "b",
        "i",
        "c",
        "./tests/test_scenarios/too_wide.yaml",
    )
    .expect("Failed to start game with scenario");
}

#[should_panic]
#[test]
fn verify_too_long() {
    let game_id = std::env::var("GAME_ID").unwrap();

    // Safe test -- login start game, quit, and then test
    common::reset_test("Username", game_id.as_str());

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login.");

    // Start game with simple scenario.
    start_game_with_scenario(
        &mut webtile,
        game_id.as_str(),
        "b",
        "i",
        "c",
        "./tests/test_scenarios/too_long.yaml",
    )
    .expect("Failed to start game with scenario");
}
