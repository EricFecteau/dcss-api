use std::cmp;

use crate::common::{pathfinding, CoordVec};
use crate::tiles::Tile;
use crate::CrawlData;
use regex::Regex;
use rustc_hash::FxHashMap;
use serde_json::Value;

use crate::common::Coord;

#[derive(Debug)]
pub(crate) struct Monsters {
    pub(crate) examine_loc: Option<Coord>,
    pub(crate) monsters: FxHashMap<u64, Monster>,
}

#[derive(Debug)]
pub(crate) struct Monster {
    pub(crate) name: String,
    pub(crate) threat: i32,
    pub(crate) pos: Option<Coord>,
    pub(crate) examined: bool,
    pub(crate) max_hp: Option<i32>,
    pub(crate) will: Option<i32>,
    pub(crate) ac: Option<i32>,
    pub(crate) ev: Option<i32>,
    pub(crate) fire: Option<i32>,
    pub(crate) cold: Option<i32>,
    pub(crate) poison: Option<i32>,
    pub(crate) negative: Option<i32>,
    pub(crate) electric: Option<i32>,
    pub(crate) class: Option<i32>,
    pub(crate) size: Option<i32>,
    pub(crate) int: Option<i32>,
    pub(crate) speed: Option<i32>,
    pub(crate) regen: Option<i32>,
    pub(crate) player_hit_monster_chance: Option<i32>,
    pub(crate) monster_hit_player_chance: Option<i32>,
    pub(crate) max_damage: Option<i32>,
}

impl Monsters {
    pub(crate) fn init() -> Self {
        Self {
            examine_loc: None,
            monsters: FxHashMap::default(),
        }
    }

    pub(crate) fn update(&mut self, mon_pos: Coord, monster: Value) {
        // If monster is "None", and monster still at that position in memory,
        // remove it from that location
        if monster.is_null() {
            for mon in self.monsters.values_mut() {
                if mon.pos.is_none() {
                    continue;
                }

                if mon.pos.unwrap() == mon_pos {
                    mon.update_pos(None);
                }
            }

            return;
        }

        let monster_obj = monster.as_object().unwrap();

        if !monster_obj.contains_key("id") {
            return;
        }

        let monster_id = monster["id"].as_u64().unwrap();

        if self.monsters.contains_key(&monster_id) {
            // If the monster moved on top of another monster,
            // most likely means that the other monster died or moved
            // therefore make that old monster have no current position
            for mon in self.monsters.values_mut() {
                if mon.pos.is_none() {
                    continue;
                }

                if mon_pos == mon.pos.unwrap() {
                    mon.update_pos(None)
                }
            }

            // Update the current monster's position to the new one
            self.monsters
                .get_mut(&monster_id)
                .unwrap()
                .update_pos(Some(mon_pos));
        } else if monster_obj.contains_key("name") {
            // If monster has name tag, likely a new monster - create new monster
            let name = monster["name"].as_str().unwrap().to_string();
            let mut threat = 0;
            if monster_obj.contains_key("threat") {
                threat = monster["threat"].as_i64().unwrap() as i32;
            }

            // Plant or friendly
            if monster_obj.contains_key("typedata") {
                let typedata_obj = monster_obj["typedata"].as_object().unwrap();
                if typedata_obj.contains_key("no_exp") && typedata_obj["no_exp"].as_bool().unwrap()
                {
                    threat = -1;
                }
            }
            self.monsters
                .insert(monster_id, Monster::new(name, threat, Some(mon_pos)));
        } else {
            // Sometimes the monster ID updates (if they disappear and come back,
            // probably to not reveal if it's the same monster or a new one)

            // But, sometimes, when many monsters are moved at once, the game may only give the
            // ID of a monster that takes the place of an old monster (same pos), I assume this
            // is true if the monster has the same exact characteristic than the monster that
            // used to be there. Therefore, create a copy of the old monster and give it it's
            // new ID. For the old monster, give it a location of None (will be updated if more
            // data comes).

            let mut found = false;

            // In case we need new monster (won't need to borrow Monsters twice)
            let mut new_mon = None;

            for mon in self.monsters.values_mut() {
                if mon.pos.is_none() {
                    continue;
                }

                // If mon found in same position, copy it to new position
                // and give old monster a None position
                if mon.pos.unwrap() == mon_pos {
                    found = true;
                    new_mon = Some((mon.name.clone(), mon.threat, mon.pos));
                    mon.update_pos(None);
                    break;
                }
            }

            // If a new monster is needed
            if let Some(mon) = new_mon {
                self.monsters
                    .insert(monster_id, Monster::new(mon.0, mon.1, mon.2));
            }

            if !self.monsters.is_empty() {
                if !found {
                    // Assume that it's just the same as the last one (unsafe?)
                    let last_mon_id = self.monsters.keys().max().unwrap();

                    self.monsters.insert(
                        monster_id,
                        Monster::new(
                            self.monsters[last_mon_id].name.clone(),
                            self.monsters[last_mon_id].threat,
                            self.monsters[last_mon_id].pos,
                        ),
                    );
                }
            } else {
                unreachable!(
                    "No monster in list, no ID found for monster data, should never happen."
                )
            }
        }
    }

    pub(crate) fn description(&mut self, description: Value, pos: Coord) {
        let mut desc_body = description["body"].as_str().unwrap().to_owned();
        desc_body.push_str("\n\n");

        // Max HP
        let re: Regex = Regex::new(r"(?:Max HP: ~|Max HP: )\s*([^<\n]*)").unwrap();
        let cap = re.captures(&desc_body).unwrap();
        let max_hp = cap[1].trim().parse::<i32>().unwrap();

        // Will
        let re: Regex = Regex::new(r"Will:\s*([^<\n]*)").unwrap();
        let cap = &re.captures(&desc_body).unwrap()[1];
        let will = cap
            .chars()
            .filter(|c| *c == '+')
            .count()
            .try_into()
            .unwrap();

        // AC
        let re: Regex = Regex::new(r"AC:\s*([^<\n]*)").unwrap();
        let cap = &re.captures(&desc_body).unwrap()[1];
        let ac = cap
            .chars()
            .filter(|c| *c == '+')
            .count()
            .try_into()
            .unwrap();

        // EV
        let re: Regex = Regex::new(r"EV:\s*([^<\n]*)").unwrap();
        let cap: &str = &re.captures(&desc_body).unwrap()[1];
        let ev = cap
            .chars()
            .filter(|c| *c == '+')
            .count()
            .try_into()
            .unwrap();

        // Fire resistance
        let re: Regex = Regex::new(r"rF:\s*([^<\n]*)").unwrap();
        let fire = decode_resistance(re, &desc_body);

        // Cold resistance
        let re: Regex = Regex::new(r"rC:\s*([^<\n]*)").unwrap();
        let cold = decode_resistance(re, &desc_body);

        // Poison resistance
        let re: Regex = Regex::new(r"rPois:\s*([^<\n]*)").unwrap();
        let poison = decode_resistance(re, &desc_body);

        // Negative resistance
        let re: Regex = Regex::new(r"rNeg:\s*([^<\n]*)").unwrap();
        let negative = decode_resistance(re, &desc_body);

        // Electric resistance
        let re: Regex = Regex::new(r"rElec:\s*([^<\n]*)").unwrap();
        let electric = decode_resistance(re, &desc_body);

        // Threat
        let re: Regex = Regex::new(r"Threat:\s*([^<\n]*)").unwrap();
        let cap: &str = &re.captures(&desc_body).unwrap()[1];
        let threat = match cap.trim() {
            "Minor" => 1,
            "Low" => 2,
            "High" => 4,
            "Lethal" => 5,
            _ => unimplemented!("Missing level"),
        };

        // Class
        let re: Regex = Regex::new(r"Class:\s*([^<\n]*)").unwrap();
        let cap: &str = &re.captures(&desc_body).unwrap()[1];
        let class = match cap.trim() {
            "Natural" => 1,
            "Undead" => 2,
            "Demonic" => 3,
            "Nonliv." => 4,
            "Plant" => 5,
            _ => unimplemented!("Missing class"),
        };

        // Size
        let re: Regex = Regex::new(r"Size:\s*([^<\n]*)").unwrap();
        let cap: &str = &re.captures(&desc_body).unwrap()[1];
        let size = match cap.trim() {
            "Tiny" => 1,
            "V. Small" => 2,
            "Small" => 3,
            "Medium" => 4,
            "Large" => 5,
            "Giant" => 6,
            _ => unimplemented!("Missing size"),
        };

        // Intelligence
        let re: Regex = Regex::new(r"Int:\s*([^<\n]*)").unwrap();
        let cap: &str = &re.captures(&desc_body).unwrap()[1];
        let int = match cap.trim() {
            "Mindless" => 1,
            "Animal" => 2,
            "Human" => 3,
            _ => unimplemented!("Missing intelligence"),
        };

        // Speed TODO: Swim speed
        let re: Regex = Regex::new(r"Speed:\s*([^%]*)").unwrap();
        let speed = if let Some(cap) = re.captures(&desc_body) {
            cap[1].parse::<i32>().unwrap()
        } else {
            100
        };

        // Regen
        let re: Regex = Regex::new(r"Regen:\s*([^/]*)").unwrap();
        let regen: i32 = if let Some(cap) = re.captures(&desc_body) {
            cap[1].parse::<i32>().unwrap_or(0)
        } else {
            0
        };

        // Chance to hit monster
        // TODO: Deal with multiple attack vectors)
        // TODO: If info collected while sleeping, chance to his might be wrong
        let re: Regex = Regex::new(r"You have about \s*([^%]*)").unwrap();
        let player_hit_monster_chance: i32 = if let Some(cap) = re.captures(&desc_body) {
            cap[1].parse::<i32>().unwrap_or(0)
        } else {
            0
        };

        // hance the monster hits the player
        let re: Regex = Regex::new(r"(?:He|She|It|They) (?:has|have) about \s*([^%]*)").unwrap();
        let monster_hit_player_chance: i32 = if let Some(cap) = re.captures(&desc_body) {
            cap[1].parse::<i32>().unwrap_or(0)
        } else {
            0
        };

        // Attack table
        let re: Regex = Regex::new(r"(?:Attacks|Attack) \s*([^\n]*)").unwrap();
        let max_damage = if let Some(found) = re.find(&desc_body) {
            let start_pos = found.end() + 1;
            let re: Regex = Regex::new(r"\n\n").unwrap();
            let end_pos = re.find(&desc_body[start_pos..]).unwrap().start();

            // Capture the 2x (for example) and the max damage
            let re_num_x = Regex::new(r"(\d+)x").unwrap();
            let re_max_damage = Regex::new(r"\s(\d+)(\s|$)").unwrap();
            desc_body[start_pos..][..end_pos]
                .split('\n')
                .map(|a| {
                    // Multiply the #x by the max damage
                    (if let Some(cap) = re_num_x.captures(a) {
                        cap[1].parse::<i32>().unwrap()
                    } else {
                        1
                    }) * (re_max_damage.captures(a).unwrap()[1]
                        .parse::<i32>()
                        .unwrap())
                })
                .max()
                .unwrap()
        } else {
            0
        };

        // TODO parse immunities and abilities (e.g. it is immune to acid, it can see invisible)

        // TODO: Get spells
        let _spellset = description["spellset"].as_array().unwrap();

        for mon in self.monsters.values_mut() {
            if mon.pos != Some(pos) {
                continue;
            }

            mon.examined = true;
            mon.threat = threat;
            mon.max_hp = Some(max_hp);
            mon.will = Some(will);
            mon.ac = Some(ac);
            mon.ev = Some(ev);
            mon.fire = Some(fire);
            mon.cold = Some(cold);
            mon.poison = Some(poison);
            mon.negative = Some(negative);
            mon.electric = Some(electric);
            mon.class = Some(class.to_owned());
            mon.size = Some(size);
            mon.int = Some(int.to_owned());
            mon.speed = Some(speed);
            mon.regen = Some(regen);
            mon.player_hit_monster_chance = Some(player_hit_monster_chance);
            mon.monster_hit_player_chance = Some(monster_hit_player_chance);
            mon.max_damage = Some(max_damage);
        }
    }

    pub(crate) fn count_path(&mut self, tiles: &[Vec<Tile>], player_pos: Coord, fov: u32) -> u32 {
        self.path_to_all_mons(tiles, player_pos, fov, true).len() as u32
    }

    fn path_to_all_mons(
        &self,
        tiles: &[Vec<Tile>],
        player_pos: Coord,
        fov: u32,
        ignore_blocked: bool, // Ignore blocking paths, better counts
    ) -> Vec<CoordVec> {
        let mut path_of_monsters = vec![];

        // How far is monster from char (max = fov)
        for mon in self.monsters.values() {
            // Harmless or plants
            if mon.threat < 0 {
                continue;
            }

            // Not on map
            if mon.pos.is_none() {
                continue;
            }

            // Don't bother to do pathfinding if known more than fov
            let abs_x = (player_pos.0 as i32 - mon.pos.unwrap().0 as i32).abs();
            let abs_y = (player_pos.1 as i32 - mon.pos.unwrap().1 as i32).abs();
            if cmp::max(abs_x, abs_y) > fov as i32 {
                continue;
            }

            let path = pathfinding(
                tiles,
                player_pos,
                mon.pos,
                None,
                None,
                fov as u64,
                ignore_blocked,
            );

            if !path.is_empty() {
                path_of_monsters.push(path)
            }
        }

        path_of_monsters
    }

    /// Return monsters that are withing FOV, regardless of path (since some monster
    /// can block the path to other monsters)
    pub(crate) fn monsters_in_fov(&self, player_pos: Coord, fov: u32) -> Vec<&Monster> {
        // TODO Deal with Plants (threat = -1)

        self.monsters
            .iter()
            .map(|mon| mon.1)
            .filter(|mon| mon.pos.is_some())
            .filter(|mon| mon.threat >= 0)
            .filter(|mon| {
                cmp::max(
                    (player_pos.0 as i32 - mon.pos.unwrap().0 as i32).abs(),
                    (player_pos.1 as i32 - mon.pos.unwrap().1 as i32).abs(),
                ) <= fov as i32
            })
            .collect::<Vec<&Monster>>()
    }

    pub(crate) fn monster_in_battle(
        &self,
        player_pos: Coord,
        fov: u32,
    ) -> Vec<FxHashMap<&str, i32>> {
        let monsters = self.monsters_in_fov(player_pos, fov);

        monsters
            .iter()
            .filter(|mon| mon.name != "invisible")
            .map(|mon| {
                let mut hash = FxHashMap::default();

                hash.insert("threat", mon.threat);
                hash.insert("max_hp", mon.max_hp.unwrap());
                hash.insert("will", mon.will.unwrap());
                hash.insert("ac", mon.ac.unwrap());
                hash.insert("ev", mon.ev.unwrap());
                hash.insert("fire", mon.fire.unwrap());
                hash.insert("cold", mon.cold.unwrap());
                hash.insert("poison", mon.poison.unwrap());
                hash.insert("negative", mon.negative.unwrap());
                hash.insert("electric", mon.electric.unwrap());
                hash.insert("class", mon.class.unwrap());
                hash.insert("size", mon.size.unwrap());
                hash.insert("int", mon.int.unwrap());
                hash.insert("speed", mon.speed.unwrap());
                hash.insert("regen", mon.regen.unwrap());
                hash.insert(
                    "player_hit_monster_chance",
                    mon.player_hit_monster_chance.unwrap(),
                );
                hash.insert(
                    "monster_hit_player_chance",
                    mon.monster_hit_player_chance.unwrap(),
                );
                hash.insert("max_damage", mon.max_damage.unwrap());

                hash
            })
            .collect()
    }

    pub(crate) fn pos_unexamined_monster(&self, player_pos: Coord, fov: u32) -> Option<Coord> {
        let mons = self
            .monsters
            .iter()
            .map(|mon| mon.1)
            .filter(|mon| mon.pos.is_some())
            .filter(|mon| mon.threat >= 0)
            .filter(|mon| !mon.examined)
            .filter(|mon| mon.name != "invisible")
            .filter(|mon| {
                cmp::max(
                    (player_pos.0 as i32 - mon.pos.unwrap().0 as i32).abs(),
                    (player_pos.1 as i32 - mon.pos.unwrap().1 as i32).abs(),
                ) <= fov as i32
            })
            .collect::<Vec<&Monster>>();

        if !mons.is_empty() {
            return mons[0].pos;
        }

        None
    }

    pub(crate) fn nearest(&mut self, tiles: &[Vec<Tile>], player_pos: Coord, fov: u32) -> CoordVec {
        let mut shortest_path = vec![];

        for path in self.path_to_all_mons(tiles, player_pos, fov, false) {
            if path.len() < shortest_path.len() || shortest_path.is_empty() {
                shortest_path.clone_from(&path);
            }
        }

        // TODO: Can improve by not cloning the path every time a shorter one is found,
        // just decide what is shortest and then clone it

        shortest_path
    }

    pub(crate) fn invisible_monster(&mut self, mon_pos: Coord) {
        self.monsters
            .insert(9999, Monster::new("invisible".to_owned(), 0, Some(mon_pos)));
    }

    pub(crate) fn invisible_removed(&mut self, mon_pos: Coord) {
        if self.monsters.contains_key(&9999) && self.monsters[&9999].pos == Some(mon_pos) {
            self.monsters.remove(&9999);
        }
    }
}

impl Monster {
    fn new(name: String, threat: i32, pos: Option<Coord>) -> Self {
        Self {
            name,
            threat,
            pos,
            examined: false,
            max_hp: None,
            will: None,
            ac: None,
            ev: None,
            fire: None,
            cold: None,
            poison: None,
            negative: None,
            electric: None,
            class: None,
            size: None,
            int: None,
            speed: None,
            regen: None,
            player_hit_monster_chance: None,
            monster_hit_player_chance: None,
            max_damage: None,
        }
    }

    fn update_pos(&mut self, pos: Option<Coord>) {
        self.pos = pos;
    }
}

impl CrawlData {
    pub fn ready_examine_monster(&mut self, coord: Coord) {
        self.monsters.examine_loc = Some(coord);
    }

    pub fn monster_count_path(&mut self) -> u32 {
        let pos: Coord = self.player_pos();
        self.monsters.count_path(&self.tiles.tiles, pos, self.fov)
    }

    pub fn monster_count_fov(&mut self) -> u32 {
        let pos: Coord = self.player_pos();
        self.monsters.monsters_in_fov(pos, self.fov).len() as u32
    }

    pub fn monster_touching(&mut self) -> bool {
        let pos: Coord = self.player_pos();
        self.monsters.monsters_in_fov(pos, 1).is_empty()
    }

    pub fn nearest_monster_path(&mut self) -> Vec<Coord> {
        let pos = self.player_pos();
        self.monsters.nearest(&self.tiles.tiles, pos, self.fov)
    }

    pub fn get_battle_monster_info(&self) -> Vec<FxHashMap<&str, i32>> {
        let pos = self.player_pos();
        let fov = self.fov;

        self.monsters.monster_in_battle(pos, fov)
    }

    pub fn get_attacking_monster_info(&self) -> Vec<FxHashMap<&str, i32>> {
        let pos = self.player_pos();

        self.monsters.monster_in_battle(pos, 1)
    }

    pub fn get_monster_threat_vec(&mut self) -> Vec<i32> {
        let pos = self.player_pos();
        self.monsters
            .monsters_in_fov(pos, self.fov)
            .iter()
            .map(|mon| mon.threat)
            .collect::<Vec<i32>>()
    }
}

fn decode_resistance(re: Regex, text: &str) -> i32 {
    if let Some(cap) = &re.captures(text) {
        if cap[1].starts_with('∞') {
            4
        } else {
            cap[1].chars().filter(|c| *c == '+').count() as i32
                - cap[1].chars().filter(|c| *c == 'x').count() as i32
        }
    } else {
        0
    }
}
