extern crate dcss_api;

use dcss_api::Webtile;

fn main() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Print the game id's that can be started
    println!("{:?}", gameid);

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Start a random game on 'dcss-web-trunk', for Minotaur berserker with a mace.
    webtile
        .start_game(&gameid[0], "b", "i", "b")
        .expect("Failed to start game");

    // Print the messages you get upon starting the game (should be processed)
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }

    // Move up and back
    webtile.write_key("key_dir_n").expect("Failed to write key");
    webtile.write_key("key_dir_s").expect("Failed to write key");

    // Print the messages you while moving
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
