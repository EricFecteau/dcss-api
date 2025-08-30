mod common;

#[test]
fn no_monster() {
    let mut webtile = common::setup_webtile("Monsters1", "./tests/scenarios/tiles/box_7x7.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Number of monsters that can be reached
    assert!(data.monster_count_path() == 0);

    // Number of monsters that can be seen
    assert!(data.monster_count_fov() == 0);

    // Test nearest_monster_path
    assert!(data.nearest_monster_path().is_empty());

    // Test coord_nearest_monster
    let coord = None;
    assert!(data.coord_nearest_monster() == coord);

    // Touching a monster
    assert!(!data.monster_touching());

    // Get threat vector
    assert!(data.get_monster_threat_vec().is_empty());

    // No monster to care about their info
    assert!(data.get_battle_monster_info().is_empty());
    assert!(data.get_attacking_monster_info().is_empty());

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn monster_from_tiles() {
    let mut webtile = common::setup_webtile("Monsters2", "./tests/scenarios/monsters/kobold.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Number of monsters that can be reached
    assert!(data.monster_count_path() == 1);

    // Number of monsters that can be seen
    assert!(data.monster_count_fov() == 1);

    // Test nearest_monster_path
    let path = [(1, 1), (1, 1), (1, 1), (1, 0), (1, 0)];
    assert!(data.nearest_monster_path() == path.to_vec());

    // Test coord_nearest_monster
    let coord = (5, 3);
    assert!(data.coord_nearest_monster() == Some(coord));

    // Touching a monster
    assert!(!data.monster_touching());

    // Touching a monster
    assert!(!data.monster_touching());

    // Get threat vector
    assert!(data.get_monster_threat_vec() == [1]);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn glass_box() {
    let mut webtile =
        common::setup_webtile("Monsters3", "./tests/scenarios/monsters/glass_box.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Number of monsters that can be reached
    assert!(data.monster_count_path() == 1);

    // Number of monsters that can be seen
    assert!(data.monster_count_fov() == 2);

    // Test nearest_monster_path
    let path = [(0, 1), (0, 1), (0, 1)];
    assert!(data.nearest_monster_path() == path.to_vec());

    // Test coord_nearest_monster
    let coord = (0, 3);
    assert!(data.coord_nearest_monster() == Some(coord));

    // Touching a monster
    assert!(!data.monster_touching());

    // Get threat vector
    assert!(data.get_monster_threat_vec() == [1, 1]);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn touching() {
    let mut webtile =
        common::setup_webtile("Monsters4", "./tests/scenarios/monsters/touching.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Number of monsters that can be reached
    assert!(data.monster_count_path() == 1);

    // Number of monsters that can be seen
    assert!(data.monster_count_fov() == 1);

    // Test nearest_monster_path
    let path = [(1, 0)];
    assert!(data.nearest_monster_path() == path.to_vec());

    // Test coord_nearest_monster
    let coord = (1, 0);
    assert!(data.coord_nearest_monster() == Some(coord));

    // Touching a monster
    assert!(data.monster_touching());

    // Get threat vector
    assert!(data.get_monster_threat_vec() == [1]);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn kobold_info() {
    let mut webtile = common::setup_webtile("Monsters5", "./tests/scenarios/monsters/kobold.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Sleepy monster
    assert!(data.get_pos_of_incapacitated_monster().is_none());

    // Look at monster to get more info
    common::examine_monster(
        data.coord_nearest_monster().unwrap(),
        &mut webtile,
        &mut data,
    );

    // Sleepy monster
    assert!(data.get_pos_of_incapacitated_monster() == Some((5, 3)));

    // Note, this moster is asleep
    let mon_data = &data.get_battle_monster_info()[0];

    // * `threat` = threat level ("Minor" => 1 | "Low" => 2 | "High" => 4 | "Lethal" => 5)
    assert!(mon_data["threat"] == 2);

    // * `max_hp` = maximum hp
    assert!(mon_data["max_hp"] == 3);

    // * `will` = will
    assert!(mon_data["will"] == 0);

    // * `ac` = ac
    assert!(mon_data["ac"] == 1);

    // * `ev` = ev
    assert!(mon_data["ev"] == 3);

    // * `fire` = fire resistance
    assert!(mon_data["fire"] == 0);

    // * `cold` = cold resistance
    assert!(mon_data["cold"] == 0);

    // * `poison` = poison resistance
    assert!(mon_data["poison"] == 0);

    // * `negative` = negative resistance
    assert!(mon_data["negative"] == 0);

    // * `electric` = electric resistance
    assert!(mon_data["electric"] == 0);

    // * `class` = monster class ("Natural" => 1 | "Undead" => 2 | "Demonic" => 3 | "Nonliv." => 4 | "Plant" => 5)
    assert!(mon_data["class"] == 1);

    // * `size` = monster size ("Tiny" => 1 | "V. Small" => 2 | "Small" => 3 | "Medium" => 4 | "Large" => 5 | "Giant" => 6)
    assert!(mon_data["size"] == 3);

    // * `int` = monster intelligence ("Mindless" => 1 | "Animal" => 2 | "Human" => 3)
    assert!(mon_data["int"] == 3);

    // * `speed` = monster speed (% compared to player)
    assert!(mon_data["speed"] == 100);

    // * `regen` = monster regeneration
    assert!(mon_data["regen"] == 0);

    // * `player_hit_monster_chance` = chance to hit monster (%)
    assert!(mon_data["player_hit_monster_chance"] == 98);

    // * `monster_hit_player_chance` = chance the monster hits the player (%)
    assert!(mon_data["monster_hit_player_chance"] == 57);

    // * `max_damage` = max damage the monster can do to the player
    assert!(mon_data["max_damage"] == 5);

    // Add test if new ones are added
    assert!(mon_data.len() == 18);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn kobold_strong_info() {
    let mut webtile =
        common::setup_webtile("Monsters6", "./tests/scenarios/monsters/kobold_strong.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Look at monster to get more info
    common::examine_monster(
        data.coord_nearest_monster().unwrap(),
        &mut webtile,
        &mut data,
    );

    let mon_data = &data.get_battle_monster_info()[0];

    println!("{mon_data:?}");

    // * `max_hp` = maximum hp
    assert!(mon_data["max_hp"] == 20);

    // * `ac` = ac
    assert!(mon_data["ac"] == 2);

    // * `player_hit_monster_chance` = chance to hit monster (%)
    assert!(mon_data["player_hit_monster_chance"] == 58);

    // * `monster_hit_player_chance` = chance the monster hits the player (%)
    assert!(mon_data["monster_hit_player_chance"] == 38);

    // * `max_damage` = max damage the monster can do to the player
    assert!(mon_data["max_damage"] == 15);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn kobold_two_hits_info() {
    let mut webtile = common::setup_webtile(
        "Monsters7",
        "./tests/scenarios/monsters/kobold_two_hits.yaml",
    );
    let mut data = common::setup_data(&mut webtile);

    // Look at monster to get more info
    common::examine_monster(
        data.coord_nearest_monster().unwrap(),
        &mut webtile,
        &mut data,
    );

    let mon_data = &data.get_battle_monster_info()[0];

    // * `max_damage` = max damage the monster can do to the player
    assert!(mon_data["max_damage"] == 8);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn kobold_ranged_info() {
    let mut webtile =
        common::setup_webtile("Monsters8", "./tests/scenarios/monsters/kobold_ranged.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Look at monster to get more info
    common::examine_monster(
        data.coord_nearest_monster().unwrap(),
        &mut webtile,
        &mut data,
    );

    let mon_data = &data.get_battle_monster_info()[0];

    // * `max_damage` = max damage the monster can do to the player
    assert!(mon_data["max_damage"] == 7);

    let mon_data = &data.get_attacking_monster_info()[0];

    // * `max_damage` = max damage the monster can do to the player
    assert!(mon_data["max_damage"] == 7);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn kobold_polearm_info() {
    let mut webtile = common::setup_webtile(
        "Monsters9",
        "./tests/scenarios/monsters/kobold_polearm.yaml",
    );
    let mut data = common::setup_data(&mut webtile);

    // Look at monster to get more info
    while let Some(coord) = data.get_pos_of_unexamined_monster() {
        common::examine_monster(coord, &mut webtile, &mut data);
    }

    let mon_data = &data.get_battle_monster_info();

    println!("{mon_data:?}");

    assert!(mon_data.len() == 3);

    let mon_data = &data.get_attacking_monster_info();

    assert!(mon_data.len() == 2);

    println!("{mon_data:?}");

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn size_info() {
    let mut webtile = common::setup_webtile("Monsters10", "./tests/scenarios/monsters/size.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Look at monster to get more info
    while let Some(coord) = data.get_pos_of_unexamined_monster() {
        common::examine_monster(coord, &mut webtile, &mut data);
    }

    // * `size` = monster size ("Tiny" => 1 | "V. Small" => 2 | "Small" => 3 | "Medium" => 4 | "Large" => 5 | "Giant" => 6)
    let mon_data = &data.get_battle_monster_info();

    let mut sizes = mon_data.iter().map(|m| m["size"]).collect::<Vec<i32>>();
    sizes.sort();

    assert!(sizes == vec![1, 2, 3, 4, 5, 6]);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}
