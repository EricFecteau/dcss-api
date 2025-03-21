use dcss_api::{Error, Webtile};
use serde_json::{from_str, Value};

#[test]
fn successful_connect() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.32").expect("Failed to connect.");

    let json: Value = from_str("{\"msg\":\"ping\"}").unwrap();
    assert_eq!(Some(json), webtile.get_message());
    let json: Value = from_str("{\"msg\":\"lobby_clear\"}").unwrap();
    assert_eq!(Some(json), webtile.get_message());
    let json: Value = from_str("{\"msg\":\"lobby_complete\"}").unwrap();
    assert_eq!(Some(json), webtile.get_message());
    assert_eq!(None, webtile.get_message());

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn failed_connect() {
    let webtile = Webtile::connect("ws://localhost:XXXX/socket", 0, "0.32");
    assert!(matches!(webtile, Err(Error::Websocket(_))));

    let webtile = Webtile::connect("ws://localhost:0000/socket", 0, "0.32");
    assert!(matches!(webtile, Err(Error::Websocket(_))));
}
