use crate::common::{Coord, CoordVec};
use serde_json::Value;
use std::error::Error;

use crate::common::add_i32_to_usize;

/// Max floor size
const MAX_FLOOR_SIZE: usize = 500;

/// Map Features (MF) received by the game (e.g. floor, wall).
///
/// # Data
///
/// 1) Walkable
/// 2) Explored
const MF: [[Option<bool>; 2]; 27] = [
    [Some(false), Some(false)], // 0: unexplored
    [Some(true), Some(true)],   // 1: floor
    [Some(false), Some(true)],  // 2: wall
    [Some(false), Some(false)], // 3: magic mapping floor
    [Some(false), Some(false)], // 4: magic mapping wall
    [Some(true), Some(true)],   // 5: door
    [Some(true), Some(true)],   // 6: item
    [None, None],               // 7: MF_MONS_FRIENDLY
    [None, None],               // 8: MF_MONS_PEACEFUL
    [None, None],               // 9: MF_MONS_NEUTRAL
    [None, None],               // 10: MF_MONS_HOSTILE
    [Some(false), Some(true)],  // 11: plant (MF_MONS_NO_EXP)
    [Some(true), Some(true)],   // 12: up stairs
    [Some(true), Some(true)],   // 13: down stairs
    [Some(true), Some(true)],   // 14: stair branch (e.g. temple)
    [Some(true), Some(true)],   // 15: feature (e.g. altar)
    [Some(true), Some(true)],   // 16: shallow water
    [Some(false), Some(true)],  // 17: lava
    [Some(false), Some(true)],  // 18: trap
    [None, None],               // 19: MF_EXCL_ROOT
    [None, None],               // 20: MF_EXCL
    [None, None],               // 21: MF_PLAYER
    [Some(false), Some(true)],  // 22: deep water
    [Some(true), Some(true)],   // 23: portal (sewer entrance)
    [Some(true), Some(true)],   // 24: portal (up or down)
    [Some(true), Some(true)],   // 25: portal (up or down)
    [Some(false), Some(false)], // 26: unexplored
];

#[derive(Debug)]
/// Stores the whole tile, path and monster data structures.
pub(crate) struct Tiles {
    /// Stores all the individual [Tile] in a vector (x) of vectors (y).
    /// The character is initialized at [[MAX_FLOOR_SIZE] / 2][[MAX_FLOOR_SIZE] / 2]
    ///
    /// # Navigation
    ///
    /// ```ignore
    /// Tiles [x, y]
    ///        [-y]
    ///       ↖ ↑ ↗
    ///  [-x] ← · → [+x]
    ///       ↙ ↓ ↘
    ///        [+y]
    /// ```
    pub(crate) tiles: Vec<Vec<Tile>>,
}

/// Info for each single tile on the specified floor.
#[derive(Copy, Clone, Debug)]
pub(crate) struct Tile {
    /// The number for the map feature.
    pub(crate) mf: usize,

    /// Bool on if the tile is walkable or not.
    pub(crate) walkable: bool,

    /// Temp blocked
    pub(crate) blocked: bool,

    /// Bool on if the tile is explored or not.
    pub(crate) explored: bool,
}

impl Tiles {
    /// Create [Tiles] object with a an initialized 2D vector
    /// based on [MAX_FLOOR_SIZE].
    pub(crate) fn init() -> Self {
        Self {
            tiles: vec![vec![Tile::new(); MAX_FLOOR_SIZE]; MAX_FLOOR_SIZE],
        }
    }

    /// Update the tiles 2D vector based on the data received from the game (as
    /// a [serde_json::Value]). Return info on monsters and floor items.
    ///
    /// # Arguments
    ///
    /// * `cells` - A [serde_json::Value] received by DCSS Webtiles.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// self.tiles.update(&message["cells"])?;
    /// ```
    #[allow(clippy::type_complexity)]
    pub(crate) fn update(
        &mut self,
        cells: &Value,
    ) -> Result<(Vec<(Coord, Value)>, CoordVec, (CoordVec, CoordVec)), Box<dyn Error>> {
        // x = moving left and right through a row
        // y = moving up and down in a column

        // Sometimes sends empty map (especially on altar levels) and best thing
        // is to just ignore it.
        if cells.is_null() {
            return Ok((Vec::new(), Vec::new(), (Vec::new(), Vec::new())));
        }

        let mut coord = (0, 0);

        // Monster list, to be returned to be processed by the monsters module
        let mut monsters: Vec<(Coord, serde_json::Value)> = Vec::new();
        let mut invisible_monsters: CoordVec = Vec::new();
        let mut remove_invisible_monsters: CoordVec = Vec::new();

        // List of items on the ground
        let mut itemlist: CoordVec = Vec::new();

        for tile in cells.as_array().unwrap() {
            let tile_object = tile.as_object().unwrap();

            // if y in cell, then both y and x will be there
            // y will always be available at the start of new row of tiles
            // y will sometimes be available after a gap (space that is not reachable)
            if tile_object.contains_key("y") {
                coord.0 = tile_object["x"].as_i64().unwrap() as i32;
                coord.1 = tile_object["y"].as_i64().unwrap() as i32;
            } else {
                // if y is not there, add x and y (from last tile + 1)
                coord.0 += 1
            }

            // Set x and y
            let offset = MAX_FLOOR_SIZE / 2;
            let x_pos = add_i32_to_usize(coord.0, offset);
            let y_pos = add_i32_to_usize(coord.1, offset);

            // Decode MF into walkable and explored
            // Will not contain MF if MF not updated
            if tile_object.contains_key("mf") {
                let mf = usize::try_from(tile_object["mf"].as_u64().unwrap())?;
                assert!(MF[mf][0].is_some(), "MF {mf} not implemented.");
                let walkable = MF[mf][0].unwrap();
                let explored = MF[mf][1].unwrap();

                // Update the tile
                self.tiles[x_pos][y_pos].update(mf, walkable, explored);

                // Add items to the items list (mf = 6)
                if self.tiles[x_pos][y_pos].mf == 6 {
                    itemlist.push((x_pos, y_pos));
                }
            }

            // Add monster data, and update the "walkability of tile"
            if tile_object.contains_key("mon") {
                monsters.push(((x_pos, y_pos), tile_object["mon"].clone()));

                if tile_object["mon"].is_null() {
                    // Update the tile to original walkability
                    self.tiles[x_pos][y_pos].unblock();
                } else {
                    // Update the tile to not walkable
                    self.tiles[x_pos][y_pos].block();
                }
            }

            // Identify invisible monsters
            if tile_object.contains_key("g") {
                // Tiles have the "{" glyph when invisible monsters are on them (for a period of time)
                if tile_object["g"] == "{" {
                    invisible_monsters.push((x_pos, y_pos));
                } else if tile_object["g"] == "@" {
                    // If character on monster tile, delete invisible (means no longer there)
                    remove_invisible_monsters.push((x_pos, y_pos));
                } else if tile_object["g"] == "§"
                    || tile_object["g"] == "☼"
                    || tile_object["g"] == "○"
                    || tile_object["g"] == "°"
                // Cloud (unsure if this will cause an issue for non-toxic clouds)
                {
                    self.tiles[x_pos][y_pos].block();
                } else {
                    self.tiles[x_pos][y_pos].unblock();
                }
            }
        }

        Ok((
            monsters,
            itemlist,
            (invisible_monsters, remove_invisible_monsters),
        ))
    }
}

impl Tile {
    /// Create new Tile object (normally created for every tile in the game).
    pub(crate) fn new() -> Self {
        Self {
            mf: 0,
            walkable: false,
            blocked: false,
            explored: false,
        }
    }

    /// Update the tile, with the information contained in the [Tile] object.
    /// # Arguments
    ///
    /// * `mf` - A u32 that will be looked up in the [MF] const.
    /// * `walkable` - A bool on if the tile is walkable or not.
    /// * `explored` - A bool on if the tile is explored or not.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// self.tiles[x_pos][y_pos].update(1, True, True, False);
    /// ```
    fn update(&mut self, mf: usize, walkable: bool, explored: bool) {
        self.mf = mf;
        self.walkable = walkable;
        self.explored = explored;
    }

    fn block(&mut self) {
        self.blocked = true;
    }

    fn unblock(&mut self) {
        self.blocked = false
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    #[test]
    fn test_tiles() {
        // Find picture of this tiles in same folder
        let path = "./test_files/cells_value/tiles_1.json";

        let data = fs::read_to_string(path).expect("Unable to read file");
        let json = serde_json::from_str(&data).expect("Unable to parse");

        let mut tiles = Tiles::init();
        tiles.update(&json).expect("Failed to updated the data");

        let mid = MAX_FLOOR_SIZE / 2;

        // Tiles [x, y]
        //        [-y]
        //       ↖ ↑ ↗
        //  [-x] ← · → [+x]
        //       ↙ ↓ ↘
        //        [+y]

        assert_eq!(tiles.tiles[mid][mid].mf, 1); // floor
        assert!(tiles.tiles[mid][mid].walkable);
        assert!(tiles.tiles[mid][mid].explored);
        assert_eq!(tiles.tiles[mid][mid - 1].mf, 1); // floor
        assert_eq!(tiles.tiles[mid][mid + 1].mf, 1); // floor
        assert_eq!(tiles.tiles[mid - 2][mid].mf, 0); // inside-a-wall
        assert!(!tiles.tiles[mid - 2][mid].walkable);
        assert!(!tiles.tiles[mid - 2][mid].explored);
        assert_eq!(tiles.tiles[mid - 1][mid].mf, 2); // wall
        assert_eq!(tiles.tiles[mid + 1][mid].mf, 2); // wall
        assert_eq!(tiles.tiles[mid + 2][mid].mf, 2); // wall
        assert_eq!(tiles.tiles[mid + 3][mid].mf, 1); // floor
        assert_eq!(tiles.tiles[mid + 5][mid + 7].mf, 13); // down stairs
        assert_eq!(tiles.tiles[mid - 4][mid - 1].mf, 6); // item
        assert_eq!(tiles.tiles[mid - 12][mid - 6].mf, 6); // item
    }
}
