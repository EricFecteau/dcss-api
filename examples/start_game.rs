extern crate dcss_api;

use dcss_api::Webtile;
use serde_json::json;

fn main() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    webtile
        .write_json(json!({
            "msg": "login",
            "username": "Username",
            "password": "Password",
        }))
        .unwrap();

    webtile.read_until("login_success", None, None).unwrap();

    webtile
        .write_json(json!({"msg": "play", "game_id": "seeded-web-trunk"}))
        .unwrap();
    webtile.read_until("player", None, None).unwrap();

    webtile.write_key(",").unwrap();
    webtile
        .read_until("input_mode", Some("mode"), Some(1))
        .unwrap();
}
