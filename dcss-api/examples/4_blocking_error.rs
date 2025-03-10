extern crate dcss_api;

use dcss_api::Webtile;
use dcss_api::{BlockingError, Error};

fn main() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.32").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Start a random game on 'dcss-web-trunk', for Minotaur berserker with a mace.
    webtile
        .start_game("dcss-web-trunk", "b", "f", "b")
        .expect("Failed to start game");

    // Print the messages you get upon starting the game (should be processed)
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }

    // Open inventory, drop everything
    webtile.write_key("i").expect("");
    webtile
        .read_until("menu", None, None)
        .expect("Failed to read");
    webtile.write_key("a").expect("");
    webtile
        .read_until("ui-push", None, None)
        .expect("Failed to read");
    webtile.write_key("d").expect("");
    webtile
        .read_until("player", None, None)
        .expect("Failed to read");
    webtile.write_key("i").expect("");
    webtile
        .read_until("menu", None, None)
        .expect("Failed to read");
    webtile.write_key("b").expect("");
    webtile
        .read_until("ui-push", None, None)
        .expect("Failed to read");
    webtile.write_key("d").expect("");
    webtile
        .read_until("player", None, None)
        .expect("Failed to read");

    // Print the messages you get upon doing these actions (should be processed)
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }

    // Try to pick up what was dropped.
    webtile.write_key(",").expect("");

    // Normally when picking up ONE item on the ground, you would read until
    // DCSS Webtiles returns a "input_mode" of mode = 1 (ready for input),
    // but since there are two items on the ground, a menu will pop up so you can
    // select the item to pick up(can't be easily anticipated, so dealt with using
    // a BlockingError).
    match webtile.read_until("input_mode", Some("mode"), Some(1)) {
        Ok(_) => (),
        Err(e) => match e {
            Error::Blocking(BlockingError::Pickup) => {
                println!("Pickup menu pop-up -- decide what to do");
                webtile.write_key("key_esc").expect(""); // Esc to ignore it
                webtile
                    .read_until("msgs", None, None)
                    .expect("Failed to read");
            }
            _ => panic!("Unexpected Error"),
        },
    };

    // Print the messages you get upon picking up an item (should be processed)
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }

    // Quit game (same as dying)
    webtile.quit_game().expect("Failed to quit");

    // Print the messages after you quit game
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }

    // Disconnect from webtile
    webtile.disconnect().expect("Failed to disconnect");
}
