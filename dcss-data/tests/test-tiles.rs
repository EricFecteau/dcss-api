mod common;

#[test]
fn box_7x7() {
    let mut webtile = common::setup_webtile("Tiles1", "./tests/scenarios/tiles/box_7x7.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Tiles [x, y]
    //        [-y]
    //       ↖ ↑ ↗
    //  [-x] ← · → [+x]
    //       ↙ ↓ ↘
    //        [+y]

    // Verify area is explored
    for x in -5..5 {
        for y in -5..5 {
            if (-4..=4).contains(&x) && (-4..=4).contains(&y) {
                assert!(data.tile_explored(x, y));
            } else {
                assert!(!data.tile_explored(x, y));
            }
        }
    }

    // Verify area is walkable
    for x in -5..5 {
        for y in -5..5 {
            if (-3..=3).contains(&x) && (-3..=3).contains(&y) {
                assert!(data.tile_walkable(x, y));
            } else {
                assert!(!data.tile_walkable(x, y));
            }
        }
    }

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn box_7x7_monster() {
    let mut webtile =
        common::setup_webtile("Tiles2", "./tests/scenarios/tiles/box_7x7_monster.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Tiles [x, y]
    //        [-y]
    //       ↖ ↑ ↗
    //  [-x] ← · → [+x]
    //       ↙ ↓ ↘
    //        [+y]

    // Verify area is explored
    for x in -5..5 {
        for y in -5..5 {
            if (-4..=4).contains(&x) && (-4..=4).contains(&y) {
                assert!(data.tile_explored(x, y));
            } else {
                assert!(!data.tile_explored(x, y));
            }
        }
    }

    // Verify area is walkable (except monster)
    for x in -5..5 {
        for y in -5..5 {
            if (-3..=3).contains(&x) && (-3..=3).contains(&y) {
                if x == 0 && y == 2 {
                    // Monster
                    assert!(!data.tile_walkable(x, y));
                } else {
                    assert!(data.tile_walkable(x, y));
                }
            } else {
                assert!(!data.tile_walkable(x, y));
            }
        }
    }

    // Verify area is walkable (in spite of monster)
    for x in -5..5 {
        for y in -5..5 {
            if (-3..=3).contains(&x) && (-3..=3).contains(&y) {
                assert!(data.tile_walkable_ignore_blocked(x, y));
            } else {
                assert!(!data.tile_walkable_ignore_blocked(x, y));
            }
        }
    }

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

#[test]
fn feature() {
    let mut webtile = common::setup_webtile("Tiles3", "./tests/scenarios/tiles/feature.yaml");
    let mut data = common::setup_data(&mut webtile);

    // Tiles [x, y]
    //        [-y]
    //       ↖ ↑ ↗
    //  [-x] ← · → [+x]
    //       ↙ ↓ ↘
    //        [+y]

    // Verify area is explored
    for x in -5..5 {
        for y in -5..5 {
            if (-4..=4).contains(&x) && (-4..=4).contains(&y) {
                assert!(data.tile_explored(x, y));
            } else {
                assert!(!data.tile_explored(x, y));
            }
        }
    }

    // Verify area is walkable (except monster)
    for x in -5..5 {
        for y in -5..5 {
            if (-3..=3).contains(&x) && (-3..=3).contains(&y) {
                if x == 0 && y == 2 {
                    // Lava
                    assert!(!data.tile_walkable(x, y));
                } else {
                    assert!(data.tile_walkable(x, y));
                }
            } else {
                assert!(!data.tile_walkable(x, y));
            }
        }
    }

    // Verify mf type
    for x in -5..5 {
        for y in -5..5 {
            if x == 5 || x == -5 || y == 5 || y == -5 {
                assert!(data.tile_mf(x, y) == 0); // unexplored
            } else if x == 4 || x == -4 || y == 4 || y == -4 {
                assert!(data.tile_mf(x, y) == 2); // wall
            } else if (-3..=3).contains(&x) && (-3..=3).contains(&y) {
                if x == 0 && y == 2 {
                    assert!(data.tile_mf(x, y) == 17); // lava
                } else {
                    assert!(data.tile_mf(x, y) == 1); // floor
                }
            } else {
                panic!();
            }
        }
    }

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}
