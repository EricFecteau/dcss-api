use dcss_api::{APIError, Webtile};
use serde_json::{from_str, Value};

#[test]
fn successful_credential_login() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    webtile
        .login_with_credentials("Username", "Password")
        .expect("Login failed.");

    let _ = webtile.get_message();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());
}

#[test]
fn failed_credential_login() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let result = webtile.login_with_credentials("XXXXXXX", "XXXXXXX");

    let e = result.err().unwrap().downcast::<APIError>().unwrap();

    assert!(matches!(e, APIError::LoginFailed));
}

#[test]
fn failed_credential_login_and_retry() {
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect.");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    let result = webtile.login_with_credentials("XXXXXXX", "XXXXXXX");
    let e = result.err().unwrap().downcast::<APIError>().unwrap();
    assert!(matches!(e, APIError::LoginFailed));

    while webtile.get_message().is_some() {}

    webtile
        .login_with_credentials("Username", "Password")
        .unwrap();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());
}

#[test]
fn get_cookie_and_login() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    webtile
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
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    webtile
        .login_with_cookie(cookie.as_str())
        .expect("Failed to login");

    let _ = webtile.get_message();

    let json: Value =
        from_str("{\"admin\": false, \"msg\": \"login_success\", \"username\": \"Username\"}")
            .unwrap();
    assert_eq!(Some(json), webtile.get_message());
}

#[test]
fn failed_cookie_login() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    let result = webtile.login_with_cookie("Username%123456789123456789123456789");
    let e = result.err().unwrap().downcast::<APIError>().unwrap();
    assert!(matches!(e, APIError::LoginFailed));
}

#[test]
fn using_old_cookie_login() {
    // Connect to DCSS Webtile
    let mut webtile =
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Log in (to a user called "Username", with a password "Password")
    webtile
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
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    webtile
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
        Webtile::connect("ws://localhost:8080/socket", 100, "0.29").expect("Failed to connect");

    // Empty message queue;
    while webtile.get_message().is_some() {}

    // Login with cookie
    let result = webtile.login_with_cookie(first_cookie.as_str());

    let e = result.err().unwrap().downcast::<APIError>().unwrap();
    assert!(matches!(e, APIError::LoginFailed));
}
