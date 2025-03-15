use crate::{common::AbsCoord, convert_coord_to_absolute, CrawlData};
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

    /// Bool on if the tile is walkable.
    pub(crate) walkable: bool,

    /// Bool on if the tile is temporarily blocked.
    pub(crate) blocked: bool,

    /// Bool on if the tile is explored.
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
    ) -> Result<
        (
            Vec<(AbsCoord, Value)>,
            Vec<AbsCoord>,
            (Vec<AbsCoord>, Vec<AbsCoord>),
        ),
        Box<dyn Error>,
    > {
        // x = moving left and right through a row
        // y = moving up and down in a column

        // Sometimes sends empty map (especially on altar levels) and best thing
        // is to just ignore it.
        if cells.is_null() {
            return Ok((Vec::new(), Vec::new(), (Vec::new(), Vec::new())));
        }

        let mut coord = (0, 0);

        // Monster list, to be returned to be processed by the monsters module
        let mut monsters: Vec<(AbsCoord, serde_json::Value)> = Vec::new();
        let mut invisible_monsters: Vec<AbsCoord> = Vec::new();
        let mut remove_invisible_monsters: Vec<AbsCoord> = Vec::new();

        // List of items on the ground
        let mut itemlist: Vec<AbsCoord> = Vec::new();

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
                    self.tiles[x_pos][y_pos].unblock(); // Make it walkable again
                } else if tile_object["g"] == "@" {
                    // If character on monster tile, delete invisible (means no longer there)
                    remove_invisible_monsters.push((x_pos, y_pos));
                }

                if tile_object["g"] == "§"
                    || tile_object["g"] == "☼"
                    || tile_object["g"] == "○"
                    || tile_object["g"] == "°"
                // Cloud (unsure if this will cause an issue for non-toxic clouds)
                {
                    self.tiles[x_pos][y_pos].block();
                } else if !tile_object.contains_key("mon") {
                    self.tiles[x_pos][y_pos].unblock(); // Remove if previously cloud
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

impl CrawlData {
    /// Is the tile at `x_pos` and `y_pos` (relative to the player's position)
    /// explored. Returns a [bool].
    ///
    /// <pre>
    /// Tiles [x, y]
    ///        -y
    ///       ↖ ↑ ↗
    ///   -x  ← · → +x
    ///       ↙ ↓ ↘
    ///        +y
    /// </pre>
    ///
    /// # Arguments
    ///
    /// * `x_pos` - A [i32] containing `x` position of the tile.
    /// * `y_pos` - A [i32] containing `y` position of the tile.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let explored = tile_explored(3, -5);
    /// ```
    pub fn tile_explored(&mut self, x_pos: i32, y_pos: i32) -> bool {
        let pos = self.player.pos;

        let x_adj = add_i32_to_usize(x_pos, pos.0);
        let y_adj = add_i32_to_usize(y_pos, pos.1);

        self.tiles.tiles[x_adj][y_adj].explored
    }

    /// Is the tile at `x_pos` and `y_pos` (relative to the player's position)
    /// walkable. Returns false if the tile is temporarily blocked (e.g. has a
    /// monster on the tile). Returns a [bool].
    ///
    /// <pre>
    /// Tiles [x, y]
    ///        -y
    ///       ↖ ↑ ↗
    ///   -x  ← · → +x
    ///       ↙ ↓ ↘
    ///        +y
    /// </pre>
    ///
    /// # Arguments
    ///
    /// * `x_pos` - A [i32] containing `x` position of the tile.
    /// * `y_pos` - A [i32] containing `y` position of the tile.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let walkable = tile_walkable(3, -5);
    /// ```
    pub fn tile_walkable(&mut self, x_pos: i32, y_pos: i32) -> bool {
        let pos = self.player.pos;

        let x_adj = add_i32_to_usize(x_pos, pos.0);
        let y_adj = add_i32_to_usize(y_pos, pos.1);

        self.tiles.tiles[x_adj][y_adj].walkable && !self.tiles.tiles[x_adj][y_adj].blocked
    }

    /// Is the tile at `x_pos` and `y_pos` (relative to the player's position)
    /// walkable. Returns true even if the tile is temporarily blocked (e.g. has a
    /// monster on the tile). Returns a [bool].
    ///
    /// <pre>
    /// Tiles [x, y]
    ///        -y
    ///       ↖ ↑ ↗
    ///   -x  ← · → +x
    ///       ↙ ↓ ↘
    ///        +y
    /// </pre>
    ///
    /// # Arguments
    ///
    /// * `x_pos` - A [i32] containing `x` position of the tile.
    /// * `y_pos` - A [i32] containing `y` position of the tile.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let walkable = tile_walkable_ignore_blocked(3, -5);
    /// ```
    pub fn tile_walkable_ignore_blocked(&mut self, x_pos: i32, y_pos: i32) -> bool {
        let pos = self.player.pos;

        let x_adj = add_i32_to_usize(x_pos, pos.0);
        let y_adj = add_i32_to_usize(y_pos, pos.1);

        self.tiles.tiles[x_adj][y_adj].walkable
    }

    /// What is the Map Feature (MF) of the tile at `x_pos` and `y_pos`
    /// (relative to the player's position). This is generally used for
    /// debugging. Returns a [usize].
    ///
    /// <pre>
    /// Tiles [x, y]
    ///        -y
    ///       ↖ ↑ ↗
    ///   -x  ← · → +x
    ///       ↙ ↓ ↘
    ///        +y
    /// </pre>
    ///
    /// These are the possible MFs. Some are never uses in the game:
    /// * 0: unexplored
    /// * 1: floor
    /// * 2: wall
    /// * 3: magic mapping floor
    /// * 4: magic mapping wall
    /// * 5: door
    /// * 6: item
    /// * 7: MF_MONS_FRIENDLY
    /// * 8: MF_MONS_PEACEFUL
    /// * 9: MF_MONS_NEUTRAL
    /// * 10: MF_MONS_HOSTILE
    /// * 11: plant (MF_MONS_NO_EXP)
    /// * 12: up stairs
    /// * 13: down stairs
    /// * 14: stair branch (e.g. temple)
    /// * 15: feature (e.g. altar)
    /// * 16: shallow water
    /// * 17: lava
    /// * 18: trap
    /// * 19: MF_EXCL_ROOT
    /// * 20: MF_EXCL
    /// * 21: MF_PLAYER
    /// * 22: deep water
    /// * 23: portal (sewer entrance)
    /// * 24: portal (up or down)
    /// * 25: portal (up or down)
    /// * 26: unexplored
    ///
    /// # Arguments
    ///
    /// * `x_pos` - A [i32] containing `x` position of the tile.
    /// * `y_pos` - A [i32] containing `y` position of the tile.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let explored = tile_mf(3, -5);
    /// ```
    pub fn tile_mf(&mut self, x_pos: i32, y_pos: i32) -> usize {
        let pos = self.player.pos;

        let x_adj = add_i32_to_usize(x_pos, pos.0);
        let y_adj = add_i32_to_usize(y_pos, pos.1);

        self.tiles.tiles[x_adj][y_adj].mf
    }

    /// Get the position according to DCSS (from game start or stairs)
    /// from a relative position (from the player). Necessary for a
    /// few functions, such as the "click_cell" function. Returns an
    /// (i32, i32).
    ///
    /// <pre>
    /// Tiles [x, y]
    ///        -y
    ///       ↖ ↑ ↗
    ///   -x  ← · → +x
    ///       ↙ ↓ ↘
    ///        +y
    /// </pre>
    ///
    /// # Arguments
    ///
    /// * `x_pos` - A [i32] containing `x` position of the tile.
    /// * `y_pos` - A [i32] containing `y` position of the tile.
    ///     
    /// # Example
    ///
    /// ```ignore
    /// let explored = tile_mf(3, -5);
    /// ```
    pub fn get_dcss_coord(&mut self, x_pos: i32, y_pos: i32) -> (i32, i32) {
        let coord = convert_coord_to_absolute(self.player.pos, (x_pos, y_pos));

        (
            coord.0 as i32 - (MAX_FLOOR_SIZE / 2) as i32,
            coord.1 as i32 - (MAX_FLOOR_SIZE / 2) as i32,
        )
    }
}
