extern crate dcss_api;

use dcss_api::Webtile;

fn main() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 50, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    println!("{:?}", game_ids);

    // Empty message queue;
    while webtile.get_message().is_some() {}
}
