use dcss_api::{Error, Webtile};
use dcss_scenario_builder::start_game_with_scenario;

fn reset_test(username: &str) {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials(username, "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game("dcss-0.32", "b", "f", "b")
        .expect("Failed to start game");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile.quit_game().expect("Failed to quit game");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn verify_wizmode() -> Result<(), Error> {
    // Safe test -- login start game, quit, and then test
    reset_test("Username");

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
        "dcss-0.32",
        "b",
        "i",
        "c",
        "./tests/test_scenarios/simple_map.yaml",
    )
    .expect("Failed to start game with scenario.");

    webtile.save_game().expect("Failed to save game.");

    webtile.continue_game("dcss-0.32")?;

    while let Some(message) = webtile.get_message() {
        if message["msg"].as_str().unwrap() == "player"
            && message.as_object().unwrap().contains_key("wizard")
            && message["wizard"].as_u64().unwrap() == 1
        {
            // webtile.quit_game()?;
            webtile.disconnect().expect("Failed");
            return Ok(());
        }
    }

    unreachable!();
}
