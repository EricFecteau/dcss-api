//! A crate to create scenarios in DCSS (wizmode) from a yaml file.

mod common;
mod lua_builder;
mod scenario_errors;
mod wizmode;

pub use scenario_errors::{Error, YamlParsingError};
use wizmode::{enable_wiz, setup_map};

use dcss_api::Webtile;
use std::fs;

use crate::lua_builder::process_scenario;

/// Start a game, using [dcss_api::Webtile::start_game_seeded], with a scenario yaml file.
///
/// # Arguments
///
/// * `webtile` - A [dcss_api::Webtile] connection.
/// * `game_id` - A string slice of the game's ID.
/// * `species` - A string slice for the character's species.
/// * `background` - A string slice for the character's background.
/// * `weapon` - A string slice for the character's weapon.
/// * `scenario_file` - a path to a YAML scenario file.
///
/// # Example
///
/// ```no_run
/// // Start a scenario game, for a Minotaur (b), Berserker (f), with a mace (b) using the
/// // branches.yaml scenario.
/// start_game_with_scenario(&mut webtile, "dcss-0.32", "b", "f", "b", "./scenarios/branches.yaml")?;
/// ```
pub fn start_game_with_scenario(
    webtile: &mut Webtile,
    game_id: &str,
    species: &str,
    background: &str,
    weapon: &str,
    scenario_file: &str,
) -> Result<(), Error> {
    // Process the scenario into Lua
    let (player_pos_d1, levels) = process_scenario(scenario_file)?;

    // Write RC File (prevent "more" error)
    webtile.set_rc_file(game_id, "show_more = false\nrest_delay = -1")?;

    // Start game
    webtile.start_game_seeded(game_id, "1", false, species, background, weapon)?;

    // Read the messages and if player["wizard"] == 1 it means that wizmode is already
    // set up, meaning this code has already been run -- skip the setup code
    while let Some(message) = webtile.get_message() {
        if message["msg"].as_str().unwrap() == "player"
            && message.as_object().unwrap().contains_key("wizard")
            && message["wizard"].as_u64().unwrap() == 1
        {
            return Ok(());
        }
    }

    // Enable the wizmode
    enable_wiz(webtile)?;

    // Set up the map
    setup_map(webtile, player_pos_d1, levels)?;

    // Empty message queue
    while webtile.get_message().is_some() {}

    // Since it's "start game with" -- the game should continue
    webtile.continue_game(game_id)?;

    Ok(())
}

/// Print the created lua from the scenario_file to the log, to help debug issues in the YAML.
///
/// # Arguments
///
/// * `scenario_file` - a path to a YAML scenario file.
///
/// # Example
///
/// ```no_run
/// // Print the lua using the `branches.yaml` scenario.
/// debug_print_lua("./scenarios/branches.yaml")?;
/// ```
pub fn debug_print_lua(scenario_file: &str) -> Result<(), Error> {
    let (_player_pos_d1, levels) = process_scenario(scenario_file)?;

    for level in levels {
        println!("\n\n========= {} ========\n\n{}", level.0, level.1);
    }

    Ok(())
}

/// Save a txt of the created lua from the scenario_file, to help debug issues in the YAML.
///
/// # Arguments
///
/// * `scenario_file` - a path to a YAML scenario file.
/// * `save_file` - a path to a the txt output file.
///
/// # Example
///
/// ```no_run
/// // Save the lua using the `branches.yaml` scenario, to `branches.txt`
/// debug_save_lua("./scenarios/branches.yaml", "branches.txt")?;
/// ```
pub fn debug_save_lua(scenario_file: &str, save_file: &str) -> Result<(), Error> {
    let (_player_pos_d1, levels) = process_scenario(scenario_file)?;

    let mut content = String::new();

    for level in levels {
        content.push_str(&format!(
            "\n\n========= {} ========\n\n{}",
            level.0, level.1
        ));
    }

    fs::write(save_file, content)?;

    Ok(())
}
