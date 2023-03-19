use flate2::{Decompress, FlushDecompress};
use log::trace;
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::error::Error;
use std::net::TcpStream;
use std::str;
use std::thread;
use std::time::{Duration, SystemTime};
use tungstenite::Message;
use tungstenite::{self, protocol::WebSocket, stream::MaybeTlsStream};
use url::Url;

/// Webtile connection, using websocket and a Deflate decoder.
#[derive(Debug)]
pub struct Webtile {
    /// Websocket (using [tungstenite::WebSocket]) to send and receive data from
    /// [DCSS Webtile](http://crawl.develz.org/wordpress/howto).
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    /// A [flate2::Decompress] decompression object (Deflate) to decompress data received
    /// by [DCSS Webtile](http://crawl.develz.org/wordpress/howto).
    decompressor: Decompress,
    /// List of messages that are being waited for from DCSS Webtiles.
    wait_list: Vec<(String, String)>,
    /// [SystemTime] of the last sent message. Used to limit the rate for
    /// running the bot on someone else's server.
    last_send: SystemTime,
    /// Speed limit in milliseconds between each command sent to DCSS Webtiles. Very
    /// important when using someone else's server.
    speed_ms: usize,
    /// VecDeque of messages received from DCSS.
    received_messages: VecDeque<Value>,
}

impl Webtile {
    /// Connects to a websocket URL, prepares the decompressor (using [flate2::Decompress]) and
    /// returns a DCSS object, with a [Webtile] connection object.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the ws:// URL
    ///
    /// # Examples
    ///
    /// ```ignore
    /// connect::connect("ws://localhost:8080/socket");
    /// ```
    pub fn connect(url: &str) -> Result<Self, Box<dyn Error>> {
        // Open connection
        let parsed_url = Url::parse(url)?;
        let (socket, _response) = tungstenite::connect(parsed_url)?;

        // Init decompressor (see https://rustpython.github.io/website/src/rustpython_vm/stdlib/zlib.rs.html)
        let wbits = 15; // Windows bits fixed (goes to -15 in flate2 because of zlib_header = false)
        let decompressor = Decompress::new_with_window_bits(false, wbits);

        let mut webtile = Self {
            socket,
            decompressor,
            wait_list: vec![],
            last_send: SystemTime::now(),
            speed_ms: 1000,
            received_messages: VecDeque::new(),
        };

        webtile.read_until("lobby_complete", "")?;

        Ok(webtile)
    }

    /// Read the websocket messages until a specified message is found. Process
    /// each message as they come in to store the updated data and verify that
    /// nothing is interrupting the expected message (e.g. a 'more' statement in
    /// the game messages).
    ///
    /// Will block forever if the expected message never comes.
    ///
    /// # Arguments
    ///
    /// * `search_message` - A string slice that holds the value in the "msg"
    /// field that is searched for.
    /// * `mode` - when the expected message is "input_mode", there are multiple
    /// possible modes. If true, it will search for "mode = 1" and only stop then.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// self.read_until("input_mode", "ready")?;
    /// ```
    pub fn read_until(&mut self, search_message: &str, mode: &str) -> Result<(), Box<dyn Error>> {
        // Send to message queue (in case one already is being waited for)
        // This queue is for when multiple parts of the system needs to wait for a message
        // usually for when unexpected things pop up (e.g. a pick-up menu). The system will
        // continue the bot when one is found, but the other will remain in the queue (unless
        // also found at the same time). This allows for multiple expected messages to be
        // packed into one received packet, without missing any of them. If the queue is empty
        // when the bot gets to this function, it will continue (it assumes the message was
        // received in another read_until).
        self.wait_list
            .push((search_message.to_owned(), mode.to_owned()));

        // loop until break (found expected results)
        let mut found = 0;
        while found == 0 {
            // Consider the value found if no message is being waited on, and break to not request
            // a message from the websocket
            if self.wait_list.is_empty() {
                break;
            }

            // Read the message from the socket into Vec<u8> -- it will be compressed
            let mut compressed_msg = self.socket.read_message()?.into_data();

            // Decompress the message and return JSON Value
            let msg = decode(&mut self.decompressor, &mut compressed_msg)?;

            // Will get array of message, go through them until what is expected is found
            for message in msg["msgs"].as_array().unwrap() {
                trace!("RECEIVED: {}", message.to_string());

                // Process the data and store the important parts
                self.received_messages.push_back(message.to_owned());

                // Get the "mode" if necessary (e.g. input_mode = 1)
                let mut found_mode = "";

                // TODO: Is a bit finicky (for a special case)
                if message.as_object().unwrap().contains_key("mode") {
                    if message["mode"].as_u64().unwrap() == 1 {
                        found_mode = "ready";
                    }
                } else if message.as_object().unwrap().contains_key("depth") {
                    if !mode.is_empty() {
                        // Only when specifically requested (the "ready" is always needed)
                        found_mode = "depth";
                    }
                }

                // Get the "msg" type
                let message_msg = message["msg"].as_str().unwrap().to_owned();

                // Find the index in the message being waited on (None if not found)
                let index = self.wait_list.iter().position(|r| r.0 == message_msg);

                // If found, (and correct found_mode), delete and identify as "found"
                // deleting will also prevent waiting in a parent loop.
                if let Some(ind) = index {
                    if self.wait_list[ind].1 == found_mode {
                        found = 1;
                        self.wait_list.remove(ind);
                    }
                }
            }
        }

        Ok(())
    }

    /// Write a [serde_json::Value] to the websocket.
    ///
    /// # Arguments
    ///
    /// * `json_val` - A [serde_json::Value] to send to DCSS Webtiles.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// self.write_json(json!({"msg": "play", "game_id": "seeded-web-trunk"}))?;
    /// ```
    pub fn write_json(&mut self, json_val: Value) -> Result<(), Box<dyn Error>> {
        // Pause while min time not met
        while SystemTime::now()
            .duration_since(self.last_send)
            .expect("Time failed")
            .as_millis()
            < self.speed_ms as u128
        {
            thread::sleep(Duration::from_millis(10));
        }
        self.last_send = SystemTime::now();

        trace!("SENT: {}", json_val.to_string());

        self.socket
            .write_message(Message::Text(json_val.to_string()))?;

        Ok(())
    }

    /// Write a string slice (passed through [keys]) to the websocket.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice to be sent to DCSS (passed through [keys]).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// self.write_key("ctrl_a")?;
    /// ```
    pub fn write_key(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        // Pause while min time not met
        while SystemTime::now()
            .duration_since(self.last_send)
            .expect("Time failed")
            .as_millis()
            < self.speed_ms as u128
        {
            thread::sleep(Duration::from_millis(10));
        }
        self.last_send = SystemTime::now();

        let json_key = keys(key);
        trace!("SENT: {}", json_key.to_string());
        self.socket
            .write_message(Message::Text(json_key.to_string()))?;

        Ok(())
    }
}

/// Decompresses (deflate) a message from DCSS Webtiles. Returns a [serde_json::Value] object of the data.
///
/// # Arguments
///
/// * `decompressor` - A [flate2::Decompress] decompression object (Deflate) to decompress data received
/// * `compressed_msg` - the compressed message received from DCSS Webtiles.
fn decode(
    decompressor: &mut Decompress,
    compressed_msg: &mut Vec<u8>,
) -> Result<Value, Box<dyn Error>> {
    // DCSS Removes 4 bytes that have to be re-added
    compressed_msg.append(&mut vec![0u8, 0, 255, 255]);

    // Decompress (Deflate)
    let bufsize = 1024 * 1024; // Needs a buffer size to work (1mb) - known to fail at 124kb (too small)
    let mut decompressed_bytes = Vec::with_capacity(bufsize); //capacity necessary, unclear why
    decompressor.decompress_vec(
        &compressed_msg[..],
        &mut decompressed_bytes,
        FlushDecompress::Sync,
    )?;
    let json_str = str::from_utf8(&decompressed_bytes)?;

    let json_data: Value = serde_json::from_str(json_str).expect("Can't JSON");

    Ok(json_data)
}

/// Convert keyword to json key or input for the game, or send the key directly. Returns
/// a [serde_json::Value] to be sent to DCSS Webtiles.
///
/// /// # Arguments
///
/// * `key` - A string slice of the key, or keyword, to be sent.
fn keys(key: &str) -> Value {
    match key {
        "tab" => json!({"msg": "key", "keycode": 9}),
        "ctrl_a" => json!({"msg": "key", "keycode": 1}),
        "esc" => json!({"msg": "key", "keycode": 27}),
        "Dir_N" => json!({"msg": "input", "text": "8"}),
        "Dir_NE" => json!({"msg": "input", "text": "9"}),
        "Dir_E" => json!({"msg": "input", "text": "6"}),
        "Dir_SE" => json!({"msg": "input", "text": "3"}),
        "Dir_S" => json!({"msg": "input", "text": "2"}),
        "Dir_SW" => json!({"msg": "input", "text": "1"}),
        "Dir_W" => json!({"msg": "input", "text": "4"}),
        "Dir_NW" => json!({"msg": "input", "text": "7"}),
        "Down" => json!({"msg": "input", "text": ">"}),
        "Up" => json!({"msg": "input", "text": "<"}),
        "enter" => json!({"msg": "input", "text": "\r"}),
        _ => json!({"msg": "input", "text": key}),
    }
}
