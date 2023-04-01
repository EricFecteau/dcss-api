extern crate dcss_api;

use dcss_api::Webtile;

fn main() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Print the messages you get upon connecting
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }

    // Log in (to a user called "Username", with a password "Password")
    webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Read RC File
    webtile
        .get_rc_file("seeded-web-trunk")
        .expect("Failed to get RC file.");

    // Print the messages you get upon connecting
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }

    webtile
        .set_rc_file("seeded-web-trunk", "show_more = false\nrest_delay = -1")
        .expect("Failed to set RC file.");

    // Print the messages you get upon connecting
    while let Some(message) = webtile.get_message() {
        println!("{:?}", message)
    }
}
