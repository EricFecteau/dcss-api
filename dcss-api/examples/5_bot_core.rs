extern crate dcss_api;

use dcss_api::Webtile;
use dcss_api::{BlockingError, Error as APIError};
use serde_json::Value;
use std::process;

fn main() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.32").expect("Failed to connect");

    // Log in (to a user called "Username", with a password "Password")
    let gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Start a random game on 'dcss-web-trunk', for Minotaur berserker with a mace.
    webtile
        .start_game_seeded(&gameid[0], "1", false, "b", "f", "b")
        .expect("Failed to start game");

    // Process the messages
    while let Some(message) = webtile.get_message() {
        processor(&message);
    }

    // Depending on what is found in the "map" data, a move up may make sense (up to the
    // bot to decide this) -- note this may if a north wall exists (no bot intelligence here).
    write_key_bot(&mut webtile, "key_dir_n", "player").expect("Failed");
    write_key_bot(&mut webtile, "key_dir_s", "player").expect("Failed");

    // Quit game (same as dying)
    webtile.quit_game().expect("Failed to quit");

    // Disconnect from webtile
    webtile.disconnect().expect("Failed to disconnect");
}

fn write_key_bot(webtile: &mut Webtile, to_send: &str, to_receive: &str) -> Result<(), APIError> {
    println!("SEND: {}", to_send);

    webtile.write_key(to_send)?;

    // Make sure you verify for blocking errors;
    if let Err(e) = webtile.read_until(to_receive, None, None) {
        match e {
            APIError::Blocking(BlockingError::More) => webtile.write_key(" ")?,
            APIError::Blocking(BlockingError::TextInput) => {
                println!("ERROR: Likely level up choice");
            }
            APIError::Blocking(BlockingError::Pickup) => println!("ERROR: Pickup"),
            APIError::Blocking(BlockingError::Acquirement(_)) => println!("ERROR: Acquirement"),
            APIError::Blocking(BlockingError::Identify(_)) => println!("ERROR: Identify"),
            APIError::Blocking(BlockingError::EnchantWeapon(_)) => println!("ERROR: EnchantWeapon"),
            APIError::Blocking(BlockingError::EnchantItem(_)) => println!("ERROR: EnchantItem"),
            APIError::Blocking(BlockingError::BrandWeapon(_)) => println!("ERROR: BrandWeapon"),
            APIError::Blocking(BlockingError::Skill) => println!("ERROR: Skill"),
            APIError::Blocking(BlockingError::Died) => {
                println!("ERROR: Died");
                process::exit(0);
            }
            _ => Err(e)?,
        }
    }

    // Process the data based on what was done (e.g. new map revealed, health of player...)
    while let Some(message) = webtile.get_message() {
        processor(&message);
    }

    Ok(())
}

fn processor(message: &Value) {
    let msg = message["msg"].as_str().unwrap();

    match msg {
        // Ignore
        "ping" => (),
        "lobby_clear" => (),
        "go_lobby" => (),
        "html" => (),
        "set_game_links" => (),
        "game_client" => (),
        "chat" => (),
        "version" => (),
        "options" => (),
        "layout" => (),
        "ui-state-sync" => (),
        "text_cursor" => (),
        "cursor" => (),
        "ui_state" => (),
        "flash" => (),
        "ui-stack" => (),
        "ui-state" => (),
        "update_menu_items" => (),
        "close_all_menus" => (),
        "delay" => (),
        "menu_scroll" => (),
        "ui-scroller-scroll" => (),
        "ui_cutoff" => (),
        "lobby_complete" => (),
        "login_success" => (),
        "game_started" => (),

        // Input & blocking
        "input_mode" => println!("PROCESS: input_mode"),

        // Messages
        "msgs" => println!("PROCESS: game log"),

        // Lobby
        "update_spectators" => println!("PROCESS: number of spectators"),

        // Player
        "player" => println!("PROCESS: player data"),

        // Dungeon
        "map" => println!("PROCESS: map data"),

        // Menu
        "menu" => println!("PROCESS: menu data"),
        "update_menu" => println!("PROCESS: menu data"),
        "close_menu" => println!("PROCESS: menu data"),
        "ui-push" => println!("PROCESS: menu data"),
        "ui-pop" => println!("PROCESS: menu data"),

        _ => {
            unreachable!();
        }
    };
}
