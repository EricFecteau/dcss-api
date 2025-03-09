use dcss_api::Webtile;
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
        .start_game("dcss-0.32", "b", "i", "b")
        .expect("Failed to start game");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile.quit_game().expect("Failed to quit game");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile.disconnect().expect("Failed to disconnect");
}

#[should_panic]
#[test]
fn verify_no_character() {
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
        "./tests/test_scenarios/no_char.yaml",
    )
    .expect("Failed to start game with scenario");
}

#[should_panic]
#[test]
fn verify_too_wide() {
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
        "./tests/test_scenarios/too_wide.yaml",
    )
    .expect("Failed to start game with scenario");
}

#[test]
fn verify_too_long() {
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
        "./tests/test_scenarios/too_long.yaml",
    )
    .expect("Failed to start game with scenario");
}
