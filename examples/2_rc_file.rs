extern crate dcss_api;

use dcss_api::Webtile;

fn main() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Write RC File
    webtile
        .set_rc_file("seeded-web-trunk", "show_more = false\nrest_delay = -1")
        .expect("Failed to set RC file.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Read RC File
    let rc_file = webtile
        .get_rc_file("seeded-web-trunk")
        .expect("Failed to get RC file.");

    print!("RC FILE: \n\n {}\n\n", rc_file);

    // Empty message queue;
    while webtile.get_message().is_some() {}
}
