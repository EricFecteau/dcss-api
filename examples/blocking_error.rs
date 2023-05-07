extern crate dcss_api;

use dcss_api::Webtile;
use dcss_api::{BlockingError, Error};
use serde_json::json;

fn main() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Print the messages you get upon connecting
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }

    // Log in (to a user called "Username", with a password "Password")
    let _gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Print the messages you get upon connecting
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }

    // Start game
    webtile
        .write_json(json!({"msg": "play", "game_id": "seeded-web-trunk"}))
        .expect("");
    webtile.read_until("player", None, None).expect("");
    webtile.write_key("-").expect("");
    webtile.read_until("ui-state-sync", None, None).expect("");
    webtile.write_key("1").expect("");
    webtile.write_key("\t\t\t \r").expect("");
    webtile.read_until("ui-push", None, None).expect("");
    webtile.write_key("b").expect("");
    webtile.write_key("i").expect("");
    webtile.write_key("b").expect("");
    webtile
        .read_until("input_mode", Some("mode"), Some(1))
        .expect("");

    webtile.write_key("6").expect("");
    webtile.write_key("i").expect("");
    webtile.write_key("a").expect("");
    webtile.write_key("d").expect("");
    webtile.write_key("i").expect("");
    webtile.write_key("b").expect("");
    webtile.write_key("d").expect("");
    webtile.write_key(",").expect("");

    match webtile.read_until("ignore", None, None) {
        Ok(_) => (),
        Err(e) => match e {
            Error::Blocking(BlockingError::Pickup) => print!("PICKUP"),
            Error::Blocking(BlockingError::Died) => print!("PICKUP"),
            _ => panic!("Unexpected Error"),
        },
    };
}
