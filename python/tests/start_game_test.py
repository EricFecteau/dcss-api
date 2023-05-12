import pytest
import dcss_api
import json


def reset_test(username):
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials(username, "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game("dcss-web-trunk", "b", "i", "b")

    while (message := webtile.get_message()) != None:
        pass

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()


def test_start_game_seeded():
    reset_test("Username")

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded("seeded-web-trunk", "1", True, "b", "i", "b")

    msgs = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Game seed: 1 (custom seed)" in msg["text"]:
            found = True
            break

    assert found

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded("seeded-web-trunk", "158985", True, "b", "i", "b")

    msgs = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Game seed: 158985 (custom seed)" in msg["text"]:
            found = True
            break

    assert found

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()


def test_start_game():
    reset_test("Username")

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game("dcss-web-trunk", "b", "i", "b")

    msgs = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Game seed" in msg["text"]:
            found = True
            break

    assert not found

    webtile.quit_game()


def test_start_game():
    reset_test("Username")

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game("dcss-web-trunk", "b", "i", "b")

    json_message = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "map"

    webtile.save_game()

    json_message = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "go_lobby"

    webtile.continue_game("dcss-web-trunk")

    json_message = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "map"

    webtile.quit_game()

    webtile.disconnect()


# #[test]
# fn start_game_two_accounts() {
#     // Safe test -- login start game, quit, and then test
#     reset_test("Username");

#     // Connect to DCSS Webtile
#     let mut webtile =
#         Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

#     // Empty message queue;
#     while webtile.get_message().is_some() {}

#     // Log in (to a user called "Username", with a password "Password")
#     let _game_ids = webtile
#         .login_with_credentials("Username", "Password")
#         .expect("Failed to login");

#     // Empty message queue;
#     while webtile.get_message().is_some() {}

#     webtile
#         .start_game("dcss-web-trunk", "b", "i", "b")
#         .expect("Failed to start game");

#     let mut msgs = from_str("{}").unwrap();
#     while let Some(message) = webtile.get_message() {
#         if message["msg"] == "msgs" {
#             msgs = message;
#             break;
#         }
#     }

#     assert!(msgs["messages"].to_string().contains("Welcome, Username "));

#     // Get last message
#     let mut last_message = from_str("{}").unwrap();
#     while let Some(message) = webtile.get_message() {
#         last_message = message;
#     }

#     assert!(last_message["msg"] == "map");

#     webtile.quit_game().expect("Failed to quit game");

#     webtile.disconnect().expect("Failed to disconnect");

#     // Safe test -- login start game, quit, and then test
#     reset_test("Username2");

#     // Connect to DCSS Webtile
#     let mut webtile =
#         Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

#     // Empty message queue;
#     while webtile.get_message().is_some() {}

#     // Log in (to a user called "Username", with a password "Password")
#     let _game_ids = webtile
#         .login_with_credentials("Username2", "Password")
#         .expect("Failed to login");

#     // Empty message queue;
#     while webtile.get_message().is_some() {}

#     webtile
#         .start_game("dcss-web-trunk", "b", "i", "b")
#         .expect("Failed to start game");

#     let mut msgs = from_str("{}").unwrap();
#     while let Some(message) = webtile.get_message() {
#         if message["msg"] == "msgs" {
#             msgs = message;
#             break;
#         }
#     }

#     assert!(msgs["messages"].to_string().contains("Welcome, Username2 "));

#     // Get last message
#     let mut last_message = from_str("{}").unwrap();
#     while let Some(message) = webtile.get_message() {
#         last_message = message;
#     }

#     assert!(last_message["msg"] == "map");

#     webtile.quit_game().expect("Failed to quit game");

#     webtile.disconnect().expect("Failed to disconnect");
# }

# #[test]
# fn start_game_two_accounts_combined() {
#     // Safe test -- login start game, quit, and then test
#     reset_test("Username");
#     reset_test("Username2");

#     // Connect to DCSS Webtile
#     let mut webtile1 =
#         Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");
#     let mut webtile2 =
#         Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

#     // Empty message queue;
#     while webtile1.get_message().is_some() {}
#     while webtile2.get_message().is_some() {}

#     // Log in (to a user called "Username", with a password "Password")
#     let _game_ids = webtile1
#         .login_with_credentials("Username", "Password")
#         .expect("Failed to login");
#     let _game_ids = webtile2
#         .login_with_credentials("Username2", "Password")
#         .expect("Failed to login");

#     // Empty message queue;
#     while webtile1.get_message().is_some() {}
#     while webtile2.get_message().is_some() {}

#     webtile1
#         .start_game("dcss-web-trunk", "b", "i", "b")
#         .expect("Failed to start game");
#     webtile2
#         .start_game("dcss-web-trunk", "b", "i", "b")
#         .expect("Failed to start game");

#     let mut msgs = from_str("{}").unwrap();
#     while let Some(message) = webtile1.get_message() {
#         if message["msg"] == "msgs" {
#             msgs = message;
#             break;
#         }
#     }

#     assert!(msgs["messages"].to_string().contains("Welcome, Username "));

#     let mut msgs = from_str("{}").unwrap();
#     while let Some(message) = webtile2.get_message() {
#         if message["msg"] == "msgs" {
#             msgs = message;
#             break;
#         }
#     }

#     assert!(msgs["messages"].to_string().contains("Welcome, Username2 "));

#     // Get last message
#     let mut last_message = from_str("{}").unwrap();
#     while let Some(message) = webtile1.get_message() {
#         last_message = message;
#     }

#     assert!(last_message["msg"] == "map");

#     // Get last message
#     let mut last_message = from_str("{}").unwrap();
#     while let Some(message) = webtile2.get_message() {
#         last_message = message;
#     }

#     assert!(last_message["msg"] == "map");

#     webtile1.quit_game().expect("Failed to quit game");
#     webtile2.quit_game().expect("Failed to quit game");

#     webtile1.disconnect().expect("Failed to disconnect");
#     webtile2.disconnect().expect("Failed to disconnect");
# }

# #[test]
# fn real_blocking_error() {
#     // Safe test -- login start game, quit, and then test
#     reset_test("Username");

#     // Connect to DCSS Webtile
#     let mut webtile =
#         Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

#     // Empty message queue;
#     while webtile.get_message().is_some() {}

#     // Log in (to a user called "Username", with a password "Password")
#     let _game_ids = webtile
#         .login_with_credentials("Username", "Password")
#         .expect("Failed to login");

#     // Empty message queue;
#     while webtile.get_message().is_some() {}

#     webtile
#         .start_game_seeded("dcss-web-trunk", "1", true, "b", "i", "b")
#         .expect("Failed to start game");

#     // Get last message
#     let mut msgs = from_str("{}").unwrap();
#     while let Some(message) = webtile.get_message() {
#         if message["msg"] == "msgs" {
#             msgs = message;
#             break;
#         }
#     }

#     assert!(msgs["messages"].to_string().contains("Welcome, Username "));

#     // Get last message
#     let mut last_message = from_str("{}").unwrap();
#     while let Some(message) = webtile.get_message() {
#         last_message = message;
#     }

#     assert!(last_message["msg"] == "map");

#     // Drop everything
#     webtile.write_key("6").expect("");
#     webtile.write_key("i").expect("");
#     webtile.write_key("a").expect("");
#     webtile.write_key("d").expect("");
#     webtile.write_key("i").expect("");
#     webtile.write_key("b").expect("");
#     webtile.write_key("d").expect("");

#     // Empty message queue;
#     webtile.read_until("close_all_menus", None, None).expect("");

#     // Pick it up again (leading to an unexpected menu)
#     webtile.write_key(",").expect("");

#     let result = webtile.read_until("whatever", None, None);

#     assert!(matches!(
#         result,
#         Err(Error::Blocking(BlockingError::Pickup))
#     ));

#     // Ignore the menu after all
#     webtile.write_key("key_esc").expect("");

#     webtile.quit_game().expect("Failed to quit game");

#     webtile.disconnect().expect("Failed to disconnect");
# }
