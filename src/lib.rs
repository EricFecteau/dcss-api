//! A API library to interact with [DCSS Webtile](http://crawl.develz.org/wordpress/howto).

mod api_errors;
mod communication;
mod deflate;
mod python_module;

use anyhow::{anyhow, Result};
use flate2::Decompress;
use serde_json::Value;
use std::collections::VecDeque;
use std::net::TcpStream;
use std::str;
use std::time::SystemTime;
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
    pub received_messages: VecDeque<Value>,
}

impl Webtile {
    /// Connects to a websocket URL, prepares the decompressor (using [flate2::Decompress]) and
    /// returns a DCSS object, with a [Webtile] connection object.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the `ws://` or `wss://` URL
    pub fn connect(url: &str, speed_ms: usize) -> Result<Self> {
        // Open connection
        let parsed_url = Url::parse(url)?;
        let (socket, _response) = tungstenite::connect(parsed_url)?;

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

        webtile
            .read_until("lobby_complete", None, None)
            .map_err(|e| anyhow!(e))?;

        Ok(webtile)
    }
}
