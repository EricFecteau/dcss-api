use crate::common::{branch_keys, Coord};
use crate::{Error, YamlParsingError};
use rustc_hash::FxHashMap;
use serde_yaml::{from_reader, Value};

/// Reads the scenario file, processes the input, identifies the features,
/// items and monsters and converts each floor into lua commands to be
/// sent to DCSS.
///
/// # Arguments
///
/// * `scenario_file` - a path to a YAML scenario file.
///
/// # Example
///
/// ```no_run
/// process_scenario("./scenarios/branches.yaml")?;
/// ```
pub(crate) fn process_scenario(
    scenario_file: &str,
) -> Result<(Coord, Vec<(String, String)>), Error> {
    // Vector to return with all the lua code (bottom up)
    let mut levels = vec![];

    // Read in the yaml file and process it (Value)
    let file_content = std::fs::File::open(scenario_file).map_err(Error::IOError)?;
    let from_reader: Value = from_reader(file_content).map_err(Error::YamlError)?;

    let default_feature = from_reader["options"]["default_feature"].as_str().unwrap();

    let mut coord: Option<Coord> = None;

    // For each level
    for level in from_reader["levels"].as_sequence().unwrap().iter() {
        let level_data = &level["level"];

        let level_name = level_data["name"].as_str().unwrap();

        // Verify the name
        let _ = branch_keys(level_name)?;

        // Process the features
        let features = level_data["features"]
            .as_sequence()
            .map(|x| process_glyphs(x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>()));

        // Process the items
        let items = level_data["items"]
            .as_sequence()
            .map(|x| process_glyphs(x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>()));

        // Process the monsters
        let monsters = level_data["monsters"]
            .as_sequence()
            .map(|x| process_glyphs(x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>()));

        // Use all glyphs to process the map data
        let (temp, map) = process_map(
            features.unwrap(),
            items.as_ref(),
            monsters.as_ref(),
            level_data["map"].as_str().unwrap(),
            default_feature,
        )?;

        if temp.is_some() {
            coord = temp;
        }

        // setup the level vec
        let mut lua_scenario = vec![];

        // Go to the level to modify
        // Second argument is the floor from the parent branch (e.g. 1 for Lair spawns you
        // back on D:1 and 2 on Slime Pit will spawn you back on Lair:2)
        // Must be from real parent (e.g. can put slime pit entrance in dungeon, but leaving
        // will place you in a real dungeon)
        lua_scenario.push(format!("debug.goto_place(\"{}\", 1)\n", level_name).to_owned());

        lua_scenario.push("you.moveto(1,1)\n".to_owned());

        lua_scenario.push("dgn.reset_level()\n".to_owned());

        lua_scenario.push(map.to_owned());

        levels.push((level_name.to_owned(), lua_scenario.concat()))
    }

    if coord.is_none() {
        Err(Error::YamlParsingError(YamlParsingError::MissingChar))?
    }

    Ok((coord.unwrap(), levels))
}

/// Maps the glyphs to the features, items or monsters in the YAML.
///
/// # Arguments
///
/// * `glyphs` - a vector of glyphs and equivalent text from the YAML that
///              can be understood by DCSS.
fn process_glyphs(glyphs: Vec<&str>) -> FxHashMap<String, String> {
    glyphs
        .iter()
        .map(|d| d.trim().split_once('=').unwrap())
        .map(|(a, b)| (a.trim().to_owned(), b.trim().to_owned()))
        .collect::<FxHashMap<_, _>>()
}

/// Converts each map glyphs (provided in the yaml) to lua based
/// on the values of the provided features, items and monsters.
/// Returns the LUA and the character's coordinates according to
/// the `@` on D:1.
///
/// # Arguments
///
/// * `features` - [FxHashMap] of the mapping between the glyphs
///                in the map and the feature understandable by
///                DCSS.
/// * `items` - [FxHashMap] of the mapping between the glyphs
///             in the map and the item understandable by
///             DCSS.
/// * `monsters` - [FxHashMap] of the mapping between the glyphs
///                in the map and the monster understandable by
///                DCSS.
/// * `map` - the map from the YAML.
/// * `default_feature` - the default feature for missing glyphs.
fn process_map(
    features: FxHashMap<String, String>,
    items: Option<&FxHashMap<String, String>>,
    monsters: Option<&FxHashMap<String, String>>,
    map: &str,
    default_feature: &str,
) -> Result<(Option<Coord>, String), Error> {
    let mut lua_map = vec![];

    let map_coords: Vec<Vec<String>> = map
        .split('\n')
        .map(|x| {
            x.trim()
                .split_terminator("")
                .skip(1)
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut player_coord_d1: Option<Coord> = None;

    for (y, inner) in map_coords.iter().enumerate() {
        for (x, glyph) in inner.iter().enumerate() {
            if x > 80 {
                Err(Error::YamlParsingError(YamlParsingError::MapTooWide))?
            }

            if y > 69 {
                Err(Error::YamlParsingError(YamlParsingError::MapTooLong))?
            }

            if glyph == "@" {
                player_coord_d1 = Some((x, y));
                let lua_feat_line = format!("dgn.terrain_changed({}, {}, \"{}\")\n", x, y, "floor");
                lua_map.push(lua_feat_line);
            } else if glyph == " " {
                let lua_feat_line = format!(
                    "dgn.terrain_changed({}, {}, \"{}\")\n",
                    x,
                    y,
                    "unseen".to_owned()
                );
                lua_map.push(lua_feat_line);
            } else {
                let glyph_feature = if !features.contains_key(glyph) {
                    default_feature
                } else {
                    &features[glyph]
                };

                let lua_feat_line =
                    format!("dgn.terrain_changed({}, {}, \"{}\")\n", x, y, glyph_feature);
                lua_map.push(lua_feat_line);

                if items.is_some() && items.unwrap().contains_key(glyph) {
                    let lua_item_line = format!(
                        "dgn.create_item({}, {}, \"{}\")\n",
                        x,
                        y,
                        &items.unwrap()[glyph]
                    );
                    lua_map.push(lua_item_line);
                }

                if monsters.is_some() && monsters.unwrap().contains_key(glyph) {
                    let lua_mons_line = format!(
                        "dgn.create_monster({}, {}, \"{}\")\n",
                        x,
                        y,
                        &monsters.unwrap()[glyph]
                    );
                    lua_map.push(lua_mons_line);
                }
            }
        }
    }

    Ok((player_coord_d1, lua_map.concat()))
}
