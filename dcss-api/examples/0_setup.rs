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

    for i in 1..20 {
        webtile
            .register_account(format!("Username{i}").as_str(), "Password", None)
            .expect("Failed to register");
    }

    // Disconnect from webtile
    webtile.disconnect().expect("Failed to disconnect");
}
