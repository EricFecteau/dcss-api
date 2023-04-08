//! A API library to interact with [DCSS Webtile](http://crawl.develz.org/wordpress/howto).

// https://github.com/crawl/crawl/blob/master/crawl-ref/source/webserver/webtiles/ws_handler.py#L319

mod api_errors;
mod common;
mod lobby;
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

/// Webtile connection, using websocket and a Deflate decoder.
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
    /// * `url` - A string slice that holds the `ws://` or `wss://` URL
    pub fn connect(url: &str, speed_ms: usize, _version: &str) -> Result<Self, Error> {
        // Open connection
        let parsed_url = Url::parse(url).map_err(Error::Url)?;
        let (socket, _response) = tungstenite::connect(parsed_url).map_err(Error::Websocket)?;

        // Init decompressor (see https://rustpython.github.io/website/src/rustpython_vm/stdlib/zlib.rs.html)
        let wbits = 15; // Windows bits fixed (goes to -15 in flate2 because of zlib_header = false)
        let decompressor = Decompress::new_with_window_bits(false, wbits);

        let mut webtile = Self {
            socket,
            decompressor,
            last_send: SystemTime::now(),
            speed_ms,
            received_messages: VecDeque::new(),
        };

        webtile.read_until("lobby_complete", None, None)?;

        Ok(webtile)
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        self.socket.close(None).map_err(Error::Websocket)?;

        Ok(())
    }

    /// Read the websocket messages until a specified message is found. Store the
    /// messages in a VeqDeque that the user can use to process the messages. Any
    /// known blocking message (e.g. a 'more' log statement) will return a [api_errors::BlockingError].
    ///
    /// Will block forever if the expected message never comes.
    ///
    /// # Arguments
    ///
    /// * `msg` - A string slice that holds the value in the "msg" field.
    /// * `key` - A Option<&str> with the name of the specific key in the json data to search for.
    /// * `value` - A Option<u64> with the value of the `key`, only if u64. Specifically meant
    /// distinguish between types of "modes" for the input_mode.
    pub fn read_until(
        &mut self,
        msg: &str,
        key: Option<&str>,
        value: Option<u64>,
    ) -> Result<(), Error> {
        // loop until break (found expected results, found a blocking type)
        let mut found = 0;
        while found == 0 {
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
                    found = 1;
                }
            }

            blocking?
        }

        Ok(())
    }

    /// Write a [serde_json::Value] to the websocket.
    ///
    /// # Arguments
    ///
    /// * `json_val` - A [serde_json::Value] to send to DCSS Webtiles.
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

    /// Write a string slice (passed through [keys]) to the websocket.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice to be sent to DCSS (passed through [keys]).
    ///
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

    pub fn get_message(&mut self) -> Option<Value> {
        self.received_messages.pop_front()
    }

    pub(crate) fn read_only_messages(&self) -> Vec<Value> {
        self.received_messages
            .clone()
            .into_iter()
            .collect::<Vec<Value>>()
    }
}
