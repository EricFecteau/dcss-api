extern crate dcss_api;

use dcss_api::Webtile;

fn main() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    webtile
        .start_game_seeded("seeded-web-trunk", "1", true, "b", "i", "b")
        .expect("Failed to start game");

    webtile.save_game().expect("Failed to save the game.");

    webtile
        .continue_game("dcss-web-trunk")
        .expect("Failed to continue game");

    webtile.quit_game().expect("Failed to quit the game.");

    // Empty message queue (would be used to process the info);
    while webtile.get_message().is_some() {}
}
