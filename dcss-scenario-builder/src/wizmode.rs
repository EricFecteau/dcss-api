use crate::common::{branch_keys, Coord};
use crate::scenario_errors::Error;

use dcss_api::{BlockingError, Error as APIError, Webtile};
use itertools::Itertools;

/// Send the commands to the API to enable the wizard mode.
///
/// # Arguments
///
/// * `webtile` - A [dcss_api::Webtile] connection.
pub(crate) fn enable_wiz(webtile: &mut Webtile) -> Result<(), Error> {
    // Enable wizard mode
    webtile.write_key("&")?;

    // Will have an error because of the "yes" prompt
    if let Err(e) = webtile.read_until("", None, None) {
        match e {
            APIError::Blocking(BlockingError::TextInput) => {
                webtile.write_key("yes")?;
                webtile.write_key("key_enter")?;
            }
            _ => Err(e)?,
        }
    }

    // Wait for player message to be sent (updates wizard: 1)
    webtile.read_until("player", None, None)?;

    // Do not send any wizard commands
    webtile.write_key("key_esc")?;

    // Empty message queue
    while webtile.get_message().is_some() {}

    Ok(())
}

/// Function that sends the lua to DCSS to create the map. It moves the character around
/// the floors to apply the lua map, but returns the character to the provided @ location
/// in the YAML on D:1.
///
/// # Arguments
///
/// * `webtile` - A [dcss_api::Webtile] connection.
/// * `player_pos_d1` - the [Coord] for the characters final position on D:1.
/// * `levels` - the lua vector for each floor.
pub(crate) fn setup_map(
    webtile: &mut Webtile,
    player_pos_d1: Coord,
    levels: Vec<(String, String)>,
) -> Result<bool, Error> {
    for (branch, level_lua) in levels.iter() {
        let (branch_name, branch_key, branch_level) = branch_keys(branch)?;

        if !(branch_name == "D" && branch_level == "1") {
            webtile.write_key("&")?;
            webtile.read_until("input_mode", None, None)?;
            webtile.write_key("~")?;
            webtile.read_until("menu", None, None)?;

            webtile.write_key(&branch_key)?;

            if let Err(e) = webtile.read_until("map", Some("player_on_level"), None) {
                match e {
                    APIError::Blocking(BlockingError::TextInput) => {
                        webtile.write_key(&branch_level)?;
                        webtile.write_key("key_enter")?;
                        webtile.read_until("map", Some("player_on_level"), None)?;
                    }
                    _ => Err(e)?,
                }
            }
        }

        // Lua
        webtile.write_key("&")?;
        webtile.read_until("input_mode", None, None)?;
        webtile.write_key("key_ctrl_t")?;

        // Ignore "TextInput" error, enter the lua and run it (by chunk, max ~100 lines)
        for lua_chunk in &level_lua.lines().chunks(100) {
            if let Err(e) = webtile.read_until("", None, None) {
                match e {
                    APIError::Blocking(BlockingError::TextInput) => {
                        webtile.write_key(&lua_chunk.collect::<String>())?;
                        webtile.write_key("key_enter")?;
                    }
                    _ => Err(e)?,
                }
            }
        }

        // Ignore "TextInput" error, leave lua interpreter
        if let Err(e) = webtile.read_until("", None, None) {
            match e {
                APIError::Blocking(BlockingError::TextInput) => {
                    webtile.write_key("key_esc")?;
                }
                _ => Err(e)?,
            }
        }

        // Verify no error message in the lua interpreter
        while let Some(message) = webtile.get_message() {
            if message.as_object().unwrap().contains_key("msg")
                && message["msg"] == "msgs"
                && message.as_object().unwrap().contains_key("messages")
            {
                for m in message["messages"].as_array().unwrap() {
                    // Red problem, but ignore god/orb message
                    if m["text"].as_str().unwrap().contains("<lightred>")
                        && m["channel"].as_i64().unwrap() != 3
                        && m["channel"].as_i64().unwrap() != 31
                    {
                        println!("{}", m); // Replace when finalized
                    }
                }
            }
        }

        // Prevent "more" -- especially in the Abyss
        if let Err(e) = webtile.read_until("map", None, None) {
            match e {
                APIError::Blocking(BlockingError::More) => {
                    webtile.write_key(" ")?;
                    webtile.read_until("map", None, None)?;
                }
                _ => Err(e)?,
            }
        }

        // Forget as much as possible, impossible to forget everything.
        // Seems impossible to visit each floor without physically being there
        // therefore you automatically know some parts of the floor. The map
        // exploration features does not consider having been to the floors if
        // you did not take the stairs to get there, therefore impossible to view
        // all floors from the map exploration. Some information is leaked.
        //
        // Potential of spawning in empty space, but leaves an annoying 3x3 wall
        // block visible when arriving to the floor (uglier than having minimal
        // advanced knowledge) -- good floor design can reduce foreknowledge.
        webtile.write_key("X")?;
        webtile.read_until("cursor", None, None)?;
        webtile.write_key("key_ctrl_x")?;
        webtile.write_key("key_esc")?;
        webtile.read_until("msgs", None, None)?;
    }

    // Return to main floor
    if levels.len() != 1 {
        webtile.write_key("&")?;
        webtile.read_until("input_mode", None, None)?;
        webtile.write_key("~")?;
        webtile.read_until("menu", None, None)?;
        webtile.write_key("D")?;
        if let Err(e) = webtile.read_until("player", Some("place"), None) {
            match e {
                APIError::Blocking(BlockingError::TextInput) => {
                    webtile.write_key("1")?;
                    webtile.write_key("key_enter")?;
                    webtile.read_until("player", None, None)?;
                }
                _ => Err(e)?,
            }
        }
    }

    // Move to @ location
    webtile.write_key("&")?;
    webtile.read_until("input_mode", None, None)?;
    webtile.write_key("key_ctrl_t")?;

    if let Err(e) = webtile.read_until("", None, None) {
        match e {
            APIError::Blocking(BlockingError::TextInput) => {
                let lua_line = format!("you.moveto({}, {})\n", player_pos_d1.0, player_pos_d1.1);
                webtile.write_key(lua_line.as_ref())?;
                webtile.write_key("key_enter")?;
            }
            _ => Err(e)?,
        }
    }

    if let Err(e) = webtile.read_until("", None, None) {
        match e {
            APIError::Blocking(BlockingError::TextInput) => {
                webtile.write_key("key_esc")?;
            }
            _ => Err(e)?,
        }
    }

    // Forget first floor as much as possible
    webtile.write_key("X")?;
    webtile.read_until("cursor", None, None)?;
    webtile.write_key("key_ctrl_x")?;
    webtile.write_key("key_esc")?;
    webtile.read_until("msgs", None, None)?;

    // Leave game
    webtile.save_game()?;

    Ok(false)
}
