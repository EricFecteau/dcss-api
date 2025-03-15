mod common;

use dcss_api::Webtile;
use dcss_scenario_builder::start_game_with_scenario;

use dcss_data::CrawlData;

#[test]
fn box_7x7() {
    let game_id = std::env::var("GAME_ID").unwrap();
    common::reset_test("Username", game_id.as_str());

    // Connect to DCSS Webtile
    let mut webtile = Webtile::connect("ws://localhost:8080/socket", 0, "0.32").unwrap();

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _ = webtile
        .login_with_credentials("Username1", "Password")
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

    // Tiles [x, y]
    //        [-y]
    //       ↖ ↑ ↗
    //  [-x] ← · → [+x]
    //       ↙ ↓ ↘
    //        [+y]

    // Verify area is explored
    for x in -5..5 {
        for y in -5..5 {
            if (-4..=4).contains(&x) && (-4..=4).contains(&y) {
                assert!(data.tile_explored(x, y));
            } else {
                assert!(!data.tile_explored(x, y));
            }
        }
    }

    // Verify area is walkable
    for x in -5..5 {
        for y in -5..5 {
            if (-3..=3).contains(&x) && (-3..=3).contains(&y) {
                assert!(data.tile_walkable(x, y));
            } else {
                assert!(!data.tile_walkable(x, y));
            }
        }
    }

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}
