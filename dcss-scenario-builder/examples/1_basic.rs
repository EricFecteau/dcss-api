use dcss_api::Webtile;
use dcss_scenario_builder::{start_game_with_scenario, Error};

fn main() -> Result<(), Error> {
    // Connect to DCSS Webtile
    let mut webtile = Webtile::connect("ws://localhost:8080/socket", 0, "0.30")?;

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _gameid = webtile.login_with_credentials("Username", "Password")?;

    // Start game
    start_game_with_scenario(
        &mut webtile,
        "dcss-0.32",
        "b",
        "i",
        "c",
        "./scenarios/branches.yaml",
    )?;

    // dcss_scenario_builder::print_lua("./scenarios/features.yaml").expect("Failed");

    // dcss_scenario_builder::save_lua("./scenarios/features.yaml", "./scenarios/features.txt")?;

    // webtile.disconnect().expect("Failed");

    Ok(())
}
