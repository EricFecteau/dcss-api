use dcss_api::Webtile;

#[test]
fn write_read_rc() {
    let game_id = std::env::var("GAME_ID").unwrap();

    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .login_with_credentials("Username", "Password")
        .expect("Login failed.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .set_rc_file(game_id.as_str(), "this is a test")
        .expect("Failed to write");

    let rc_file = webtile
        .get_rc_file(game_id.as_str())
        .expect("Failed to read.");

    assert_eq!("this is a test", rc_file);

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .set_rc_file(game_id.as_str(), "show_more = false\nrest_delay = -1")
        .expect("Failed to write");

    let rc_file = webtile
        .get_rc_file(game_id.as_str())
        .expect("Failed to read.");

    assert_eq!("show_more = false\nrest_delay = -1", rc_file);

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn blank_rc_file() {
    let game_id = std::env::var("GAME_ID").unwrap();

    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .login_with_credentials("Username", "Password")
        .expect("Login failed.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .set_rc_file(game_id.as_str(), "")
        .expect("Failed to write");

    let rc_file = webtile
        .get_rc_file(game_id.as_str())
        .expect("Failed to read.");

    assert_eq!("", rc_file);

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .set_rc_file(game_id.as_str(), "show_more = false\nrest_delay = -1")
        .expect("Failed to write");

    let rc_file = webtile
        .get_rc_file(game_id.as_str())
        .expect("Failed to read.");

    assert_eq!("show_more = false\nrest_delay = -1", rc_file);

    webtile.disconnect().expect("Failed to disconnect");
}
