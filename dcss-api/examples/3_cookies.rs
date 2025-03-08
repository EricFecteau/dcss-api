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

    // Get cookie from the game
    let cookie = webtile.request_cookie().unwrap();

    println!("{}", cookie);

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Disconnect from DCSS Webtile
    webtile.disconnect().expect("Failed to disconnect.");

    // Connect (again) to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    let _gameid = webtile
        .login_with_cookie(cookie.as_str())
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Disconnect from DCSS Webtile
    webtile.disconnect().expect("Failed to disconnect.");
}
