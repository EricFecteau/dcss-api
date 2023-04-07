# Connection to DCSS

## Introduction

[DCSS Webtiles](https://crawl.develz.org/wordpress/howto) uses websocket to communicate between the client and the DCSS server. 

## Connecting to DCSS webtiles through websocket

You can connect to the websocket of the DCSS webtile using [tungstenite](https://docs.rs/tungstenite/latest/tungstenite/). The example below assumes a DCSS webtile on 'localhost:8080'. Running it will provide you some [ZLIB](https://www.zlib.net/) compressed data.

```rust
// URL to DCSS Webtiles
let url = "ws://localhost:8080/socket";

// Parse with url crate
let parsed_url = url::Url::parse(url).unwrap();

// Connect with tungstenite crate
let (mut socket, _response) = tungstenite::connect(parsed_url).unwrap();

// Read the socket until something is received
let mut compressed_msg = socket.read_message().unwrap().into_data();
```

The `compressed_msg` variable in the example above will likely contain `[170, 86, 202, 45, 78, 47, 86, 178, 138, 174, 6, 49, 148, 172, 20, 148, 10, 50, 243, 210, 149, 106, 99, 107, 1, 0]'`.

## Decoding the received data

Now that data has been received, it can be decoded. Before decoding the data, the 4 bytes removed by DCSS (prior to sending to the client) must be re-added (`[0u8, 0, 255, 255]`). Once added, the data can be decoded with [flate2](https://docs.rs/flate2/latest/flate2/) and then the data can be turned into a json object with [serde_json](https://docs.rs/serde_json/latest/serde_json/).

```rust
// Append removed bytes
compressed_msg.append(&mut vec![0u8, 0, 255, 255]);

// Create decompressor (https://github.com/rust-lang/flate2-rs/issues/312)
let wbits = 15; // Windows bits fixed (goes to -15 in flate2 because of zlib_header = false)
let mut decompressor = flate2::Decompress::new_with_window_bits(false, wbits);

let bufsize = 1024 * 1024; // Needs a buffer size to work (1mb)
let mut decompressed_bytes = Vec::with_capacity(bufsize);

// Use the decompressor to decompress the data
decompressor
    .decompress_vec(
        &compressed_msg[..],
        &mut decompressed_bytes,
        flate2::FlushDecompress::Sync,
    )
    .unwrap();

// Convert to str
let json_str = std::str::from_utf8(&decompressed_bytes).unwrap();

// Convert to json
let json_data: serde_json::Value = serde_json::from_str(json_str).unwrap();
```

The `json_data` variable in the example above will likely contain `{'msgs': [{'msg': 'ping'}]}` as a serde_json `Value` object. [Here](TBD) is a list of things that can be received.

## Sending data to DCSS webtiles

Sending data to DCSS is simpler than receiving it, but the correct objects must be sent. The example below connects to the webtiles and sends a login command for a user and a play command for DCSS. This assumes that a user called `Username`, that has a password equal to `Password`, exists on `localhost:8080`. It also assumes a game id of `dcss-web-trunk` is available.

```rust
// URL to DCSS Webtiles
let url = "ws://localhost:8080/socket";

// Parse with url crate
let parsed_url = url::Url::parse(url).unwrap();

// Connect with tungstenite crate
let (mut socket, _response) = tungstenite::connect(parsed_url).unwrap();

// Data packets to be sent
let message1 = serde_json::json!({"msg": "login", "username": "Username", "password": "Password"});
let message2 = serde_json::json!({"msg": "play", "game_id": "dcss-web-trunk"});

// Send messages
socket.write_message(tungstenite::Message::Text(message1.to_string())).unwrap();
socket.write_message(tungstenite::Message::Text(message2.to_string())).unwrap();

// Wait for 5 seconds
std::thread::sleep(std::time::Duration::from_millis(5000));
```

If you watch on http://localhost:8080/#lobby, you will see for 5 seconds the User named `Username` playing a game of `dcss-web-trunk`. See list of commands that can be sent [here](TBD). 