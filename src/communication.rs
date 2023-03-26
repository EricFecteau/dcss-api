use crate::api_errors;
use crate::deflate;
use crate::Webtile;

use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::thread;
use std::time::{Duration, SystemTime};
use tungstenite::Message;

impl Webtile {
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
    pub fn read_until(&mut self, msg: &str, key: Option<&str>, value: Option<u64>) -> Result<()> {
        // loop until break (found expected results, found a blocking type)
        let mut found = 0;
        while found == 0 {
            // Read the message from the socket into Vec<u8> -- it will be compressed
            let mut compressed_msg = self
                .socket
                .read_message()
                .map_err(|e| anyhow!(e))?
                .into_data();

            // Decompress the message and return JSON Value
            let messages = deflate::deflate_to_json(&mut self.decompressor, &mut compressed_msg)?;

            // Alert if blocking
            let mut blocking = Ok(());

            // Will get array of message, go through them until what is expected is found
            for message in messages["msgs"].as_array().unwrap() {
                // Send data to a VeqDeque to be pulled by user;
                self.received_messages.push_back(message.to_owned());

                // Pre-process the data to identify blocking
                if let Err(e) = api_errors::blocking_messages(message) {
                    match e.downcast_ref().unwrap() {
                        api_errors::BlockingError::Died => return Err(e), // Automatic return when death
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
    pub fn write_json(&mut self, json_val: Value) -> Result<()> {
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
            .map_err(|e| anyhow!(e))?;

        Ok(())
    }

    /// Write a string slice (passed through [keys]) to the websocket.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice to be sent to DCSS (passed through [keys]).
    ///
    pub fn write_key(&mut self, key: &str) -> Result<()> {
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
        self.socket
            .write_message(Message::Text(json_key.to_string()))?;

        Ok(())
    }
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
