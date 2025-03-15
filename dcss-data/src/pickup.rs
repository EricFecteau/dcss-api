use crate::common::pathfinding;
use crate::common::AbsCoord;
use crate::tiles::Tile;
use std::cmp;

#[derive(Debug)]
pub(crate) struct Pickup {
    pub(crate) unknown: Vec<AbsCoord>,
    pub(crate) ignore: Vec<AbsCoord>,
}

impl Pickup {
    pub(crate) fn init() -> Self {
        Self {
            unknown: vec![],
            ignore: vec![],
        }
    }

    pub(crate) fn update(&mut self, item_coord: AbsCoord) {
        let already_listed = self.unknown.iter().any(|x: &AbsCoord| *x == item_coord);

        let ignore = self.ignore.iter().any(|x: &AbsCoord| *x == item_coord);

        if !already_listed && !ignore {
            self.unknown.push(item_coord);
        }
    }

    pub(crate) fn unknown_item_loc(&self, player_coord: AbsCoord) -> bool {
        let in_list = self.unknown.iter().any(|x: &AbsCoord| *x == player_coord);

        let ignore = self.ignore.iter().any(|x: &AbsCoord| *x == player_coord);

        if in_list && !ignore {
            return true;
        }

        false
    }

    pub(crate) fn remove_item_loc(&mut self, player_coord: AbsCoord) {
        let index = self.unknown.iter().position(|x| *x == player_coord);

        if let Some(i) = index {
            self.unknown.remove(i);
        }
    }

    pub(crate) fn new_ignore_item_loc(&mut self, player_coord: AbsCoord) {
        self.remove_item_loc(player_coord);

        let ignore = self.ignore.iter().any(|x: &AbsCoord| *x == player_coord);

        if !ignore {
            self.ignore.push(player_coord);
        }
    }

    fn path_to_all_items(
        &self,
        tiles: &[Vec<Tile>],
        player_coord: AbsCoord,
        fov: u32,
    ) -> Vec<Vec<AbsCoord>> {
        let mut path_of_items = vec![];

        // How far is monster from char (max = fov)
        for item_coord in self.unknown.clone() {
            // Don't bother to do pathfinding if known more than fov
            let abs_y = (player_coord.0 as i32 - item_coord.0 as i32).abs();
            let abs_x = (player_coord.1 as i32 - item_coord.1 as i32).abs();
            if cmp::max(abs_y, abs_x) > fov as i32 {
                continue;
            }

            let path = pathfinding(
                tiles,
                player_coord,
                Some(item_coord),
                None,
                None,
                fov as u64,
                false,
            );

            if !path.is_empty() {
                path_of_items.push(path)
            }
        }

        path_of_items
    }

    pub(crate) fn nearest(
        &mut self,
        tiles: &[Vec<Tile>],
        player_coord: AbsCoord,
        fov: u32,
    ) -> Vec<AbsCoord> {
        let mut shortest_path = vec![];

        for path in self.path_to_all_items(tiles, player_coord, fov) {
            if path.len() < shortest_path.len() || shortest_path.is_empty() {
                shortest_path.clone_from(&path);
            }
        }

        // TODO: Can improve by not cloning the path every time a shorter one is found,
        // just decide what is shortest and then clone it

        shortest_path
    }
}
