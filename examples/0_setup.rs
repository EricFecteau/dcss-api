extern crate dcss_api;

use dcss_api::Webtile;

fn main() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.32").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Register usernames for tests
    webtile
        .register_account("Username", "Password", None)
        .expect("Failed to register");
    webtile
        .register_account("Username2", "Password", None)
        .expect("Failed to register");
    webtile
        .register_account("Username3", "Password", None)
        .expect("Failed to register");
    webtile
        .register_account("Username4", "Password", None)
        .expect("Failed to register");

    // Disconnect from webtile
    webtile.disconnect().expect("Failed to disconnect");
}
