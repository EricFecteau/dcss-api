use dcss_api::Webtile;
use serde_json::from_str;

fn reset_test(username: &str, version: &str) {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials(username, "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game(version, "b", "i", "b")
        .expect("Failed to start game");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile.quit_game().expect("Failed to quit game");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn start_game_seeded_0_29() {
    // Safe test -- login start game, quit, and then test
    reset_test("Username", "dcss-0.29");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game_seeded("dcss-0.29", "1", true, "b", "i", "b")
        .expect("Failed to start game");

    // Get last message
    let mut msgs = from_str("{}").unwrap();
    while let Some(message) = webtile.get_message() {
        if message["msg"] == "msgs" {
            msgs = message;
            break;
        }
    }

    assert!(msgs["messages"]
        .to_string()
        .contains("Game seed: 1 (custom seed)"));

    webtile.quit_game().expect("Failed to quit game");

    webtile.disconnect().expect("Failed to disconnect");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game_seeded("dcss-0.29", "158985", false, "b", "i", "b")
        .expect("Failed to start game");

    // Get last message
    let mut msgs = from_str("{}").unwrap();
    while let Some(message) = webtile.get_message() {
        if message["msg"] == "msgs" {
            msgs = message;
            break;
        }
    }

    assert!(msgs["messages"]
        .to_string()
        .contains("Game seed: 158985 (custom seed)"));

    webtile.quit_game().expect("Failed to quit game");

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn start_game_seeded_0_30() {
    // Safe test -- login start game, quit, and then test
    reset_test("Username", "dcss-0.30");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.30").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game_seeded("dcss-0.30", "1", true, "b", "i", "b")
        .expect("Failed to start game");

    // Get last message
    let mut msgs = from_str("{}").unwrap();
    while let Some(message) = webtile.get_message() {
        if message["msg"] == "msgs" {
            msgs = message;
            break;
        }
    }

    assert!(msgs["messages"]
        .to_string()
        .contains("Game seed: 1 (custom seed)"));

    webtile.quit_game().expect("Failed to quit game");

    webtile.disconnect().expect("Failed to disconnect");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.30").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game_seeded("dcss-0.30", "158985", false, "b", "i", "b")
        .expect("Failed to start game");

    // Get last message
    let mut msgs = from_str("{}").unwrap();
    while let Some(message) = webtile.get_message() {
        if message["msg"] == "msgs" {
            msgs = message;
            break;
        }
    }

    assert!(msgs["messages"]
        .to_string()
        .contains("Game seed: 158985 (custom seed)"));

    webtile.quit_game().expect("Failed to quit game");

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn start_game_seeded_0_31() {
    // Safe test -- login start game, quit, and then test
    reset_test("Username", "dcss-0.31");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.31").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game_seeded("dcss-0.31", "1", true, "b", "i", "b")
        .expect("Failed to start game");

    // Get last message
    let mut msgs = from_str("{}").unwrap();
    while let Some(message) = webtile.get_message() {
        if message["msg"] == "msgs" {
            msgs = message;
            break;
        }
    }

    assert!(msgs["messages"]
        .to_string()
        .contains("Game seed: 1 (custom seed)"));

    webtile.quit_game().expect("Failed to quit game");

    webtile.disconnect().expect("Failed to disconnect");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.31").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game_seeded("dcss-0.31", "158985", false, "b", "i", "b")
        .expect("Failed to start game");

    // Get last message
    let mut msgs = from_str("{}").unwrap();
    while let Some(message) = webtile.get_message() {
        if message["msg"] == "msgs" {
            msgs = message;
            break;
        }
    }

    assert!(msgs["messages"]
        .to_string()
        .contains("Game seed: 158985 (custom seed)"));

    webtile.quit_game().expect("Failed to quit game");

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn start_game_seeded_0_32() {
    // Safe test -- login start game, quit, and then test
    reset_test("Username", "dcss-0.32");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game_seeded("dcss-0.32", "1", true, "b", "i", "b")
        .expect("Failed to start game");

    // Get last message
    let mut msgs = from_str("{}").unwrap();
    while let Some(message) = webtile.get_message() {
        if message["msg"] == "msgs" {
            msgs = message;
            break;
        }
    }

    assert!(msgs["messages"]
        .to_string()
        .contains("Game seed: 1 (custom seed)"));

    webtile.quit_game().expect("Failed to quit game");

    webtile.disconnect().expect("Failed to disconnect");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _game_ids = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .start_game_seeded("dcss-0.32", "158985", false, "b", "i", "b")
        .expect("Failed to start game");

    // Get last message
    let mut msgs = from_str("{}").unwrap();
    while let Some(message) = webtile.get_message() {
        if message["msg"] == "msgs" {
            msgs = message;
            break;
        }
    }

    assert!(msgs["messages"]
        .to_string()
        .contains("Game seed: 158985 (custom seed)"));

    webtile.quit_game().expect("Failed to quit game");

    webtile.disconnect().expect("Failed to disconnect");
}
