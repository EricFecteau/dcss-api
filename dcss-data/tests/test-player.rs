mod common;

#[test]
fn player_basic_mibe() {
    // "b" (Minotaur), "f" (Berserker) , "b" (Mace)
    let mut webtile = common::setup_webtile("Player1", "./tests/scenarios/tiles/box_7x7.yaml");
    let data = common::setup_data(&mut webtile);

    // Player HP
    assert!(data.player_hp() == 19);

    // Player max HP
    assert!(data.player_hp() == 19);

    // Player poison HP
    assert!(data.player_poison_hp() == 19);

    // Player AC
    assert!(data.player_ac() == 2);

    // Player EV
    assert!(data.player_ev() == 11);

    // Player SH
    assert!(data.player_sh() == 0);

    // Player Str
    assert!(data.player_str() == 21);

    // Player Int
    assert!(data.player_int() == 4);

    // Player Dex
    assert!(data.player_dex() == 9);

    webtile.quit_game().unwrap();

    webtile.disconnect().unwrap();
}

// Hurt player
// Poisoned player
// Lethally poisoned player
// Things chaning (test that)
