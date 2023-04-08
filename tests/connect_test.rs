use dcss_api::{Error, Webtile};
use serde_json::{from_str, Value};

#[test]
fn successful_connect() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect.");

    let json: Value = from_str("{\"msg\":\"ping\"}").unwrap();
    assert_eq!(Some(json), webtile.get_message());
    let json: Value = from_str("{\"msg\":\"lobby_clear\"}").unwrap();
    assert_eq!(Some(json), webtile.get_message());
    let json: Value = from_str("{\"msg\":\"lobby_complete\"}").unwrap();
    assert_eq!(Some(json), webtile.get_message());
    assert_eq!(None, webtile.get_message());
}

#[test]
fn failed_connect() {
    let webtile = Webtile::connect("ws://localhost:XXXX/socket", 100, "0.29");
    assert!(matches!(webtile, Err(Error::Url(_))));

    let webtile = Webtile::connect("ws://localhost:0000/socket", 100, "0.29");
    assert!(matches!(webtile, Err(Error::Websocket(_))));
}
