use dcss_api::{Error, Webtile};
use serde_json::{from_str, Value};

#[test]
fn successful_credential_login() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let _ = webtile
        .login_with_credentials("Username", "Password")
        .expect("Login failed.");

    let _ = webtile.get_message();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn multiple_login_same_user() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let _ = webtile
        .login_with_credentials("Username", "Password")
        .expect("Login failed.");

    let _ = webtile.get_message();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let _ = webtile
        .login_with_credentials("Username", "Password")
        .expect("Login failed.");

    let mut found = 0;
    while let Some(message) = webtile.get_message() {
        let message_obj = message.as_object().unwrap();
        if message_obj["msg"] == "login_success" {
            found = 1;
            let json: Value = from_str(
                "{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}",
            )
            .unwrap();
            assert_eq!(json, message);
        }
    }
    if found == 0 {
        panic!("Failed to find a login success.")
    }

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn multiple_login_diff_user() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let _ = webtile
        .login_with_credentials("Username", "Password")
        .expect("Login failed.");

    let _ = webtile.get_message();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let _ = webtile
        .login_with_credentials("Username2", "Password")
        .expect("Login failed.");

    let mut found = 0;
    while let Some(message) = webtile.get_message() {
        let message_obj = message.as_object().unwrap();
        if message_obj["msg"] == "login_success" {
            found = 1;
            let json: Value = from_str(
                "{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username2\"}",
            )
            .unwrap();
            assert_eq!(json, message);
        }
    }
    if found == 0 {
        panic!("Failed to find a login success.")
    }

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn failed_credential_login() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let result = webtile.login_with_credentials("XXXXXXX", "XXXXXXX");

    assert!(matches!(result, Err(Error::LoginFailed)));

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn failed_credential_login_and_retry() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let result = webtile.login_with_credentials("XXXXXXX", "XXXXXXX");
    assert!(matches!(result, Err(Error::LoginFailed)));

    while webtile.get_message().is_some() {}

    let _ = webtile
        .login_with_credentials("Username", "Password")
        .unwrap();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn get_cookie_and_login() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _ = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    let _ = webtile.get_message();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());

    // Get cookie from the game
    let cookie = webtile.request_cookie().unwrap();

    assert_eq!("Username%", &cookie[0..9]);

    // Disconnect from DCSS Webtile
    webtile.disconnect().expect("Failed to disconnect.");

    // Connect (again) to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    let _ = webtile
        .login_with_cookie(cookie.as_str())
        .expect("Failed to login");

    let _ = webtile.get_message();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn failed_cookie_login() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    let result = webtile.login_with_cookie("Username%123456789123456789123456789");
    assert!(matches!(result, Err(Error::LoginFailed)));

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn using_old_cookie_login() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    let _ = webtile
        .login_with_credentials("Username", "Password")
        .expect("Failed to login");

    let _ = webtile.get_message();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());

    // Get cookie from the game
    let first_cookie = webtile.request_cookie().unwrap();

    assert_eq!("Username%", &first_cookie[0..9]);

    // Disconnect from DCSS Webtile
    webtile.disconnect().expect("Failed to disconnect.");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    let _ = webtile
        .login_with_cookie(first_cookie.as_str())
        .expect("Failed to login.");

    let _ = webtile.get_message();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());

    // Get cookie from the game
    let second_cookie = webtile.request_cookie().unwrap();

    assert_eq!("Username%", &second_cookie[0..9]);

    // Disconnect from DCSS Webtile
    webtile.disconnect().expect("Failed to disconnect.");

    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    let result = webtile.login_with_cookie(first_cookie.as_str());

    assert!(matches!(result, Err(Error::LoginFailed)));

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn credential_login_gameid() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Login failed.");

    println!("{:?}", gameid);

    let test_gameid = vec![
        "dcss-web-trunk".to_owned(),
        "seeded-web-trunk".to_owned(),
        "tut-web-trunk".to_owned(),
        "sprint-web-trunk".to_owned(),
    ];
    assert_eq!(gameid, test_gameid);

    webtile.disconnect().expect("Failed to disconnect");
}

#[test]
fn cookie_login_gameid() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let gameid = webtile
        .login_with_credentials("Username", "Password")
        .expect("Login failed.");

    let test_gameid = vec![
        "dcss-web-trunk".to_owned(),
        "seeded-web-trunk".to_owned(),
        "tut-web-trunk".to_owned(),
        "sprint-web-trunk".to_owned(),
    ];
    assert_eq!(gameid, test_gameid);

    // Get cookie from the game
    let cookie = webtile.request_cookie().unwrap();

    assert_eq!("Username%", &cookie[0..9]);

    // Disconnect from DCSS Webtile
    webtile.disconnect().expect("Failed to disconnect.");

    // Connect (again) to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 0, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    let gameid = webtile
        .login_with_cookie(cookie.as_str())
        .expect("Failed to login");

    let test_gameid = vec![
        "dcss-web-trunk".to_owned(),
        "seeded-web-trunk".to_owned(),
        "tut-web-trunk".to_owned(),
        "sprint-web-trunk".to_owned(),
    ];
    assert_eq!(gameid, test_gameid);

    webtile.disconnect().expect("Failed to disconnect");
}
