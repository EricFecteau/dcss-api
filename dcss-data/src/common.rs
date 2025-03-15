use rustc_hash::FxHashMap;
use serde_json::Value;

use crate::tiles::Tile;
use std::collections::VecDeque;

// Type used in the data, stands for (x, y) on a 2d grid
pub type AbsCoord = (usize, usize);
pub type RelCoord = (i32, i32);

/// Add a [i32] (including negative) to an [usize]. Returns a [usize].
///
/// # Arguments
///
/// * `usize_val` - A [usize] value.
/// * `i32_val` - A [i32] value.
///
/// # Example
///
/// ```ignore
/// let add_i32_to_usize = add_i32_to_usize(-10i32, 50usize);
/// ```
pub(crate) fn add_i32_to_usize(i32_val: i32, usize_val: usize) -> usize {
    if i32_val.is_negative() {
        usize_val - i32_val.wrapping_abs() as u64 as usize
    } else {
        usize_val + i32_val as usize
    }
}

/// Identify the character corresponding to the index (e.g. 0 -> a, 1 -> b...)
/// from 0 (a), to 51 (Z), twice over the alphabet (lower case (e.g. 25 (z)) and
/// upper case (e.g. 26 (A))).
///
/// # Arguments
///
/// * key - a [usize] value between 0 (a) and 51 (Z).
///
/// # Example
///
/// ```ignore
/// let capital_A = index_to_char(26);
/// ```
pub(crate) fn _index_to_char<'a>(key: usize) -> &'a str {
    let char_list = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    char_list[key]
}

/// Identify the index corresponding to the character (e.g. a -> 0, b -> 1...)
/// from a (0), to Z (51), twice over the alphabet (lower case (e.g. z (25)) and
/// upper case (e.g. A (26))).
///
/// # Arguments
///
/// * key - a [String] value between a (0) and Z (51).
///
/// # Example
///
/// ```ignore
/// let index_26 = char_to_index("A");
/// ```
pub(crate) fn char_to_index(letter: &str) -> usize {
    let char_list: Vec<&str> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    char_list.iter().position(|&r| r == letter).unwrap()
}

/// Extract parameter from a message, following a substring. Returns the
/// [String] between the `to_find` and the `extract_until` (or None).
///
/// # Arguments
///
/// * `text` - A &str with the full text to search in.
/// * `to_find` - A &str to find in `text`.
/// * `extract_until` - A Vec<char> as a stop character.
///
/// # Example
///
/// ```ignore
/// let text = "Base accuracy: +3  Base damage: 8  Base attack delay: 1.4";
/// let value = extract_param(&text, &"Base damage: ", &vec![' ', '\n']);
/// ```
pub(crate) fn extract_param(
    text: &str,
    to_find: &str,
    extract_until: &Vec<char>,
) -> Option<String> {
    // Copy string
    let mut searchable = text.to_owned();

    // Can't find things at end of strings
    searchable.push('\n');

    let start_to_find = searchable.find(to_find);

    start_to_find?; // Returns the none if none;

    let start_index = start_to_find.unwrap() + to_find.len();

    let mut end_index: usize = 1_000_000;
    for end in extract_until {
        let temp = searchable[start_index..].find(*end);
        if let Some(value) = temp {
            if value < end_index {
                end_index = value;
            }
        };
    }
    if end_index == 1_000_000 {
        return None;
    } else {
        end_index += start_index;
    }

    Some(searchable[start_index..end_index].trim().to_string())
}

pub(crate) fn structured_table(table: Value) -> FxHashMap<String, Vec<(u64, String)>> {
    let mut return_table = FxHashMap::default();
    let table_array = table.as_array().unwrap();

    let mut curr_level_name = String::new();
    let mut curr_level_table = vec![];
    let mut first = 0;
    for item in table_array {
        let item_obj = item.as_object().unwrap();
        let level = item_obj["level"].as_u64().unwrap();
        if level == 1 {
            if first > 0 {
                return_table.insert(curr_level_name.to_owned(), curr_level_table);
                curr_level_table = vec![];
            }
            first += 1;
            item_obj["text"]
                .as_str()
                .unwrap()
                .clone_into(&mut curr_level_name)
        } else {
            if !item_obj.contains_key("q") {
                continue;
            }
            let quantity = item_obj["q"].as_u64().unwrap();
            let calling = item_obj["text"]
                .as_str()
                .unwrap()
                .chars()
                .nth(1)
                .unwrap()
                .to_string();
            curr_level_table.push((quantity, calling));
        }
    }

    return_table.insert(curr_level_name, curr_level_table);
    return_table
}

pub(crate) fn convert_coords_to_relative(pos: AbsCoord, coords: Vec<AbsCoord>) -> Vec<RelCoord> {
    // Each step is relative to the previous step (the first one relative
    // to the player)

    let mut relative_coords = vec![];
    let mut cum_change = (0, 0);

    let mut coords = coords;
    coords.reverse();

    for coord in coords {
        let relative_coord = (
            (coord.0 as i32 - cum_change.0 - pos.0 as i32),
            (coord.1 as i32 - cum_change.1 - pos.1 as i32),
        );

        cum_change = (
            cum_change.0 + relative_coord.0,
            cum_change.1 + relative_coord.1,
        );

        relative_coords.push(relative_coord);
    }

    relative_coords.reverse();

    relative_coords
}

pub(crate) fn convert_coord_to_absolute(pos: AbsCoord, coord: RelCoord) -> AbsCoord {
    (
        add_i32_to_usize(coord.0, pos.0),
        add_i32_to_usize(coord.1, pos.1),
    )
}

pub(crate) fn convert_coord_to_relative(pos: AbsCoord, coord: AbsCoord) -> RelCoord {
    (
        (coord.0 as i32 - pos.0 as i32),
        (coord.1 as i32 - pos.1 as i32),
    )
}

#[derive(Debug)]
/// Struct for each node for the A* algorithm.
struct Node {
    /// Cost of moving to this node, depends on path.
    cost: u64,
    /// Number of movement completed to get to this node.
    moves_done: u64,
    /// Number of estimated moves necessary to get to goal.
    moves_to_goal: u64,
    /// Coordinates (x, y) of the node in the &Vec<Vec<[Tile]>> object.
    location: AbsCoord,
    /// Coordinates (x, y) of the node that is being searched in the &Vec<Vec<[Tile]>> object.
    end_goal: Option<AbsCoord>,
    /// The location in the list of nodes (explored) that the parent node is found.
    parent_node: Option<usize>,
    /// Extra key to make ordering deterministic
    key_break: u64,
}

impl Node {
    /// Calculate the cost by estimating the maximum absolute distance (diagonally) to the target.
    fn calc_cost(&mut self) {
        if self.end_goal.is_some() {
            let x_max = self.location.0 as i32 - self.end_goal.unwrap().0 as i32;
            let y_max = self.location.1 as i32 - self.end_goal.unwrap().1 as i32;

            let x_max = x_max.wrapping_abs() as u64;
            let y_max = y_max.wrapping_abs() as u64;

            self.moves_to_goal = x_max.max(y_max);
        }

        self.cost = self.moves_done + self.moves_to_goal;
    }
}

/// Computes the path to a known coordinate or unknown object using A* Algorithm and return
/// the path (vector of (x, y) coordinates).
///
/// # Arguments
///
/// * `tiles` - a &Vec<Vec<[Tile]>>, indexed by y and x coordinates.
/// * `start_location` - (x, y) location in the &Vec<Vec<[Tile]>> to start the algorithm.
/// * `end_location` (optional) - stop when (x, y) is reached.
/// * `end_search_mf` (optional) - stop when Tile.mf == end_search_mf is reached.
/// * `end_search_type` - stop when specified condition is reached (specific to this implementation).
/// * `max_path` - maximum path steps (made to speed up when looking dozens of time per action).
pub(crate) fn pathfinding(
    tiles: &[Vec<Tile>],
    start_location: AbsCoord,
    end_location: Option<AbsCoord>,
    end_search_mf: Option<usize>,
    end_search_type: Option<&'static str>,
    max_path: u64,
    ignore_blocked: bool,
) -> Vec<AbsCoord> {
    // Create a list of all nodes
    let mut explored: Vec<Node> = vec![];

    // Has the final_path been found
    let mut final_path = None;

    // Open is a list of Nodes not yet checked by A*.
    let mut open: VecDeque<Node> = VecDeque::new();

    // Extra key to make ordering deterministic
    let mut key_break = 0;

    // Add the starting Node (since they are nested meant to be nodes)
    let mut first_node = Node {
        cost: 0,
        moves_done: 0,
        moves_to_goal: 0,
        location: start_location,
        end_goal: end_location,
        parent_node: None,
        key_break,
    };
    first_node.calc_cost();
    open.push_back(first_node);
    key_break += 1;

    // Create a closed set of Nodes already checked by A*.
    let mut closed: Vec<AbsCoord> = Vec::new();
    closed.push(start_location);

    let dir_list = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut end_found = false;
    while !open.is_empty() && !end_found {
        explored.push(open.pop_front().unwrap());
        let index = explored.len() - 1;
        let parent = &explored[index];

        // Limit how many moves the algorithm does
        if parent.moves_done > max_path {
            break;
        }

        // Look at all 8 directions
        for direction in dir_list {
            let curr_x = add_i32_to_usize(direction.0, parent.location.0);
            let curr_y = add_i32_to_usize(direction.1, parent.location.1);

            // Don't look at nodes already explored, else add them to the closed Nodes
            if closed.contains(&(curr_x, curr_y)) {
                continue;
            }
            closed.push((curr_x, curr_y));

            // Has the end been found (all types)
            if let Some(end_type) = end_search_mf {
                if tiles[curr_x][curr_y].mf == end_type {
                    end_found = true;
                }
            }
            if let Some(end_type) = end_search_type {
                if end_type == "unexplored" && !tiles[curr_x][curr_y].explored {
                    end_found = true;
                }
            }
            if let Some(end_type) = end_location {
                if (curr_x, curr_y) == end_type {
                    end_found = true;
                }
            }

            let walkable = if ignore_blocked {
                tiles[curr_x][curr_y].walkable
            } else {
                tiles[curr_x][curr_y].walkable && !tiles[curr_x][curr_y].blocked
            };

            // If end found or the tile is walkable, add end to end of Nodes
            if end_found || walkable {
                let mut child = Node {
                    cost: 0,
                    moves_done: parent.moves_done + 1,
                    moves_to_goal: 0,
                    location: (curr_x, curr_y),
                    end_goal: end_location,
                    parent_node: Some(index),
                    key_break,
                };
                child.calc_cost();
                key_break += 1;

                if end_found {
                    final_path = Some(child);
                    break;
                } else {
                    // Add child to opened list;
                    open.push_back(child);
                };
            }
        }
        open.make_contiguous()
            .sort_by_key(|node| (node.cost, node.key_break));
    }

    // Unravel the nested nodes - will give shortest path
    let mut path = vec![];
    if let Some(fp) = final_path {
        let mut curr_node = &fp;
        path.push(curr_node.location);
        while curr_node.parent_node.is_some() {
            curr_node = &explored[curr_node.parent_node.unwrap()];
            path.push(curr_node.location);
        }
        path.pop(); // Remove the starting element
    }

    path
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_i32_to_usize() {
        let test_val = add_i32_to_usize(10i32, 10usize);
        assert_eq!(test_val, 20);
        let test_val = add_i32_to_usize(50, 50);
        assert_eq!(test_val, 100);
        let test_val = add_i32_to_usize(-50, 50);
        assert_eq!(test_val, 0);
    }

    #[test]
    fn test_extract_param() {
        let text = "Base accuracy: +3  Base damage: 8  Base attack delay: 1.4";

        // Normal
        let test_val = extract_param(text, "Base damage: ", &vec![' ', '\n']);
        assert_eq!(test_val, Some("8".to_string()));

        let test_val = extract_param(text, "Base attack delay: ", &vec![' ', '\n']);
        assert_eq!(test_val, Some("1.4".to_string()));

        let test_val = extract_param(text, "Base accuracy: ", &vec![' ', '\n']);
        assert_eq!(test_val, Some("+3".to_string()));

        // String not found
        let test_val = extract_param(text, "Base coolness: ", &vec![' ', '\n']);
        assert_eq!(test_val, None);

        // End char not found
        let test_val = extract_param(text, "Base accuracy: ", &vec!['X', 'Y']);
        assert_eq!(test_val, None);
    }
}
