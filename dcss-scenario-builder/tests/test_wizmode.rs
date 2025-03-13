mod common;

use dcss_api::{Error, Webtile};
use dcss_scenario_builder::start_game_with_scenario;

#[test]
fn verify_wizmode() -> Result<(), Error> {
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
        "./tests/test_scenarios/simple_map.yaml",
    )
    .expect("Failed to start game with scenario.");

    webtile.save_game().expect("Failed to save game.");

    webtile.continue_game(game_id.as_str())?;

    while let Some(message) = webtile.get_message() {
        if message["msg"].as_str().unwrap() == "player"
            && message.as_object().unwrap().contains_key("wizard")
            && message["wizard"].as_u64().unwrap() == 1
        {
            webtile.quit_game()?;
            webtile.disconnect().expect("Failed");
            return Ok(());
        }
    }

    unreachable!();
}
