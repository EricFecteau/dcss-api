//! An API library to interact with [DCSS Webtile](http://crawl.develz.org/wordpress/howto).

mod api_errors;
mod common;
mod lobby;
mod play;
mod python_module;

pub use api_errors::{BlockingError, Error};

use api_errors::blocking_messages;
use flate2::Decompress;
use serde_json::Value;
use std::collections::VecDeque;
use std::net::TcpStream;
use std::result::Result;
use std::str;
use std::thread;
use std::time::{Duration, SystemTime};
use tungstenite::Message;
use tungstenite::{self, protocol::WebSocket, stream::MaybeTlsStream};
use url::Url;

/// Webtile connection, using websocket ([tungstenite]) and a Deflate decoder ([flate2]).
#[derive(Debug)]
pub struct Webtile {
    /// Websocket (using [tungstenite::WebSocket]) to send and receive data from
    /// [DCSS Webtile](http://crawl.develz.org/wordpress/howto).
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    /// A [flate2::Decompress] decompression object (Deflate) to decompress data received
    /// by [DCSS Webtile](http://crawl.develz.org/wordpress/howto).
    decompressor: Decompress,
    /// [SystemTime] of the last sent message. Used to limit the rate for
    /// running the bot on someone else's server.
    last_send: SystemTime,
    /// A [bool] of if the searched for data (in the websocket) has been found.
    message_found: bool,
    /// Speed limit in milliseconds between each command sent to DCSS Webtiles.
    speed_ms: usize,
    /// [VecDeque] of messages received from DCSS.
    received_messages: VecDeque<Value>,
}

impl Webtile {
    /// Connects to a websocket URL, prepares the decompressor (using [flate2::Decompress]) and
    /// returns a [Webtile] connection object.
    ///
    /// # Arguments
    ///
    /// * `url` - A [&str] that holds the `ws://` or `wss://` URL
    /// * `speed_ms` - A [usize] that depicts the speed limit in milliseconds between
    /// each command sent to DCSS Webtiles.
    /// * `_version` - Currently a placeholder for the version number of DCSS, in case
    /// the API changes in the future.
    ///     
    /// # Example
    ///
    /// ```no_run
    /// let mut webtile = Webtile::connect("ws://localhost:8080/socket", 100, "0.29")?;
    /// ```
    pub fn connect(url: &str, speed_ms: usize, _version: &str) -> Result<Self, Error> {
        // Open connection
        let parsed_url = Url::parse(url).map_err(Error::Url)?;
        let (socket, _response) = tungstenite::connect(parsed_url).map_err(Error::Websocket)?;

        // Init decompressor (see https://rustpython.github.io/website/src/rustpython_vm/stdlib/zlib.rs.html)
        let wbits = 15; // Windows bits fixed (goes to -15 in flate2 because of zlib_header = false)
        let decompressor = Decompress::new_with_window_bits(false, wbits);

        // Create webtile object
        let mut webtile = Self {
            socket,
            decompressor,
            last_send: SystemTime::now(),
            speed_ms,
            message_found: false,
            received_messages: VecDeque::new(),
        };

        // Wait until the "lobby_complete" message is received -- meaning a
        // successful connection
        webtile.read_until("lobby_complete", None, None)?;

        Ok(webtile)
    }

    /// Close the websocket connection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// webtile.disconnect()?;
    /// ```
    pub fn disconnect(&mut self) -> Result<(), Error> {
        self.socket.close(None).map_err(Error::Websocket)?;

        Ok(())
    }

    /// Read the websocket messages until a specified message is found. Stores the
    /// messages in a [VecDeque] that can be accessed by the user through the
    /// [`Webtile::get_message()`] function. Any known blocking message (e.g.
    /// a 'more' log statement) will return a [api_errors::BlockingError].
    ///
    /// Will block forever if the expected message never comes.
    ///
    /// # Arguments
    ///
    /// * `msg` - A [&str] that holds the value expected in the "msg" field of any returned message.
    /// * `key` - A optional [&str] with the name of the specific key in the json data to search for.
    /// * `value` - A optional [u64] with the value of the `key`, only if u64. Specifically meant to
    /// distinguish between types of "modes" for the input_mode.
    ///
    /// # Example
    ///
    /// ```no_run
    ///
    /// // Read until the "close_all_menu" message is received
    /// webtile.read_until("close_all_menus", None, None)
    ///
    /// // Read until the "input_mode" message is received, with mode == 1
    /// webtile.read_until("input_mode", Some("mode"), Some(1))
    /// ```
    pub fn read_until(
        &mut self,
        msg: &str,
        key: Option<&str>,
        value: Option<u64>,
    ) -> Result<(), Error> {
        // loop until break (found expected results or found a blocking type)
        // use self variable in order to retain the info when there is a blocking error
        while !self.message_found {
            // Read the message from the socket into Vec<u8> -- it will be compressed
            let mut compressed_msg = self
                .socket
                .read_message()
                .map_err(Error::Websocket)?
                .into_data();

            // Decompress the message and return JSON Value
            let messages = common::deflate_to_json(&mut self.decompressor, &mut compressed_msg)?;

            // Alert if blocking
            let mut blocking = Ok(());

            // Will get array of message, go through them until what is expected is found
            for message in messages["msgs"].as_array().unwrap() {
                // Send data to a VeqDeque to be pulled by user;
                self.received_messages.push_back(message.to_owned());

                // Pre-process the data to identify blocking
                if let Err(e) = blocking_messages(message) {
                    match e {
                        Error::Blocking(BlockingError::Died) => return Err(e), // Automatic return when death
                        _ => blocking = Err(e),
                    }
                };

                // If searching for key-value
                let message_msg = message["msg"].as_str().unwrap().to_owned();

                // Same message
                if msg == message_msg &&
                    // And no key
                    (key.is_none() ||
                    // or And contains key
                    (message.as_object().unwrap().contains_key(&key.unwrap().to_owned()) &&
                        // And no value
                        (value.is_none() ||
                        // or And value correct
                        message[key.unwrap()].as_u64().unwrap() == value.unwrap())))
                {
                    self.message_found = true;
                }
            }

            blocking?
        }

        self.message_found = false;

        Ok(())
    }

    /// Write a [serde_json::Value] to the websocket. Will only send if sufficient time has
    /// elapsed since the last sent data, according to the [`Webtile::connect`] speed_ms option.
    ///
    /// # Arguments
    ///
    /// * `json_val` - A [serde_json::Value] to send to DCSS Webtiles.
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Send the login command
    /// webtile.write_json(json!({
    ///     "msg": "login",
    ///     "username": "Username",
    ///     "password": "Password",
    /// }))?;
    /// ```
    pub fn write_json(&mut self, json_val: Value) -> Result<(), Error> {
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

        self.socket
            .write_message(Message::Text(json_val.to_string()))
            .map_err(Error::Websocket)?;

        Ok(())
    }

    /// Write a string slice (passed through [common::keys]) to the websocket. Special
    /// characters starting with `key_` will be sent as a keycode (e.g. `key_esc` will
    /// send the `esc` character). Will only send if sufficient time has elapsed since
    /// the last sent data, according to the [`Webtile::connect`] speed_ms option.
    ///
    /// Special keys:
    /// * CTRL+char = `key_ctrl_a` to `key_ctrl_z`
    /// * Special chars = `key_tab`, `key_esc` and `key_enter`
    /// * Cardinal directions: `key_dir_n`, `key_dir_ne`, `key_dir_e`, `key_dir_se`,
    /// `key_dir_s`, `key_dir_sw`, `key_dir_w` and `key_dir_nw`
    /// * Stairs: `key_stair_down` and `key_stair_up`
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice to be sent to DCSS (passed through [common::keys]).
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Send the `esc` key
    /// webtile.write_key("key_esc")
    ///
    /// // Send the 'a' key
    /// webtile.write_key("a")
    ///
    /// // Send a string of keys that will move left, open a menu and drop an item (slot a)
    /// webtile.write_key("6iad")
    /// ```
    pub fn write_key(&mut self, key: &str) -> Result<(), Error> {
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

        let json_key = common::keys(key);
        self.socket
            .write_message(Message::Text(json_key.to_string()))
            .map_err(Error::Websocket)?;

        Ok(())
    }

    /// Get the messages received by the DCSS Webtile (as [serde_json::Value]), in
    /// order of reception. Will return [None] if the queue is empty.
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Print the messages received, until the queue is empty
    /// while let Some(message) = webtile.get_message() {
    ///     println!("{:?}", message)
    /// }
    /// ```
    pub fn get_message(&mut self) -> Option<Value> {
        self.received_messages.pop_front()
    }

    /// Get a copy of the messages currently in the received queue.
    pub(crate) fn read_only_messages(&self) -> Vec<Value> {
        self.received_messages
            .clone()
            .into_iter()
            .collect::<Vec<Value>>()
    }
}
