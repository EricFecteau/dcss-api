use crate::Webtile;
use crate::{BlockingError, Error};

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pyclass]
/// Connects to a websocket URL, prepares the decompressor and
/// returns a Webtile connection object.
///
/// Parameters:
///     url (str):      String that holds the `ws://` or `wss://` URL.
///     speed_ms (int): An int that depicts the speed limit in
///                     milliseconds between each command sent to
///                     DCSS Webtiles.
///     _version (str): Currently a placeholder for the version
///                     number of DCSS, in case the API changes in
///                     the future.
///     
/// Example:
///     webtile = Webtile::connect("ws://localhost:8080/socket", 100, "0.29")
///
pub struct WebtilePy {
    webtile: Webtile,
}

pyo3::create_exception!(mymodule, APIErr, PyException);
pyo3::create_exception!(mymodule, BlockingErr, PyException);

#[pymethods]
impl WebtilePy {
    #[new]
    fn connect(url: &str, speed_ms: usize, version: &str) -> PyResult<Self> {
        let webtile = Webtile::connect(url, speed_ms, version);

        match webtile {
            Ok(t) => Ok(Self { webtile: t }),
            Err(e) => Err(PyErr::new::<APIErr, _>(e.to_string())),
        }
    }

    /// Close the websocket connection.
    ///
    /// Example:
    ///     webtile.disconnect()
    ///
    fn disconnect(&mut self) -> PyResult<()> {
        let result = self.webtile.disconnect();

        match result {
            Ok(t) => Ok(t),
            Err(e) => Err(PyErr::new::<APIErr, _>(e.to_string())),
        }
    }

    /// Read the websocket messages until a specified message is found.
    /// Stores the messages in a VecDeque that can be accessed by the
    /// user through the .get_message() function. Any known blocking
    /// message (e.g. a 'more' log statement) will return a BlockingError.
    ///
    /// Will block forever if the expected message never comes.
    ///
    /// Parameters:
    ///     msg (str):              holds the value expected in the "msg"
    ///                             field of any returned message.
    ///     key (optional str):     holds the name of the specific key in
    ///                             the json data to search for.
    ///     value (optional int):   value of the `key` (only if int). Specifically
    ///                             meant to distinguish between types of "modes"
    ///                             for the input_mode.
    ///
    /// Example:
    ///     # Read until the "close_all_menu" message is received
    ///     webtile.read_until("close_all_menus", None, None)
    ///
    ///     # Read until the "input_mode" message is received, with mode == 1
    ///     webtile.read_until("input_mode", Some("mode"), Some(1))
    #[pyo3(signature = (msg, key=None, value=None))]
    fn read_until(&mut self, msg: &str, key: Option<&str>, value: Option<u64>) -> PyResult<()> {
        let result = self.webtile.read_until(msg, key, value);

        match result {
            Ok(t) => Ok(t),
            Err(e) => match e {
                Error::Blocking(BlockingError::Pickup) => {
                    Err(PyErr::new::<BlockingErr, _>("Pickup"))
                }
                _ => Err(PyErr::new::<APIErr, _>(e.to_string())),
            },
        }
    }

    /// Write a string to the websocket. Special characters starting
    /// with `key_` will be sent as a keycode (e.g. `key_esc` will
    /// send the `esc` character). Will only send if sufficient time
    /// has elapsed since the last sent data, according to the speed_ms
    /// option.
    ///
    /// Special keys:
    ///     CTRL+char =           key_ctrl_a to key_ctrl_z
    ///     Special chars =       key_tab, key_esc and key_enter
    ///     Cardinal directions = key_dir_n, key_dir_ne, key_dir_e,
    ///                           key_dir_se, key_dir_s, key_dir_sw,
    ///                           key_dir_w and key_dir_nw
    ///     Stairs =              key_stair_down and key_stair_up
    ///
    /// Arguments:
    ///     key (str): A string to be sent to DCSS.
    ///
    /// Example:
    ///     # Send the `esc` key
    ///     webtile.write_key("key_esc")
    ///
    ///     # Send the 'a' key
    ///     webtile.write_key("a")
    ///
    ///     # Send a string of keys that will move left, open a menu
    ///     # and drop an item (slot a)
    ///     webtile.write_key("6iad")
    fn write_key(&mut self, key: &str) -> PyResult<()> {
        self.webtile
            .write_key(key)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Write a json string to the websocket. Will only send if sufficient
    /// time has elapsed since the last sent data, according to the speed_ms
    /// option.
    ///
    /// Arguments:
    ///     json - A json string to send to DCSS Webtiles.
    ///
    /// Example:
    ///     # Send the login command
    ///     webtile.write_json('{
    ///         "msg": "login",
    ///         "username": "Username",
    ///         "password": "Password",
    ///     }')
    fn write_json(&mut self, json: &str) -> PyResult<()> {
        self.webtile
            .write_json(serde_json::from_str(json).unwrap())
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Login to the game, using a username and password. It returns a list
    /// of all playable game IDs.
    ///
    /// Arguments:
    ///     username (str): A string of the user's username.
    ///     password (str): A string of the user's password.
    ///
    /// Example:
    ///     # Login under the user "Username", with a password of "Password"
    ///     webtile.login_with_credentials("Username", "Password")
    fn login_with_credentials(&mut self, username: &str, password: &str) -> PyResult<Vec<String>> {
        self.webtile
            .login_with_credentials(username, password)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Login to the game, using a cookie. It returns a list of all
    /// playable game IDs.
    ///
    /// Arguments:
    ///     cookie (str): A string of the user's cookie (received
    ///                   previously).
    ///
    /// Example:
    ///     # Login under the user "Username", with a cookie
    ///     webtile.login_with_cookie("Username%123456789123456789123456789")
    fn login_with_cookie(&mut self, cookie: &str) -> PyResult<Vec<String>> {
        self.webtile
            .login_with_cookie(cookie)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Request a cookie from the DCSS Webtile.
    ///
    /// Example:
    ///     webtile.request_cookie()
    fn request_cookie(&mut self) -> PyResult<String> {
        self.webtile
            .request_cookie()
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Get the messages received by the DCSS Webtile, in order of
    /// reception. Will return `None` if the queue is empty.
    ///
    /// Example:
    ///     # Print the messages received, until the queue is empty
    ///     while (message := webtile.get_message()) != None:
    ///         print(message)
    fn get_message(&mut self) -> Option<String> {
        let value = self.webtile.received_messages.pop_front();
        value.map(|v| v.to_string())
    }

    /// Set the RC file content of a specific game ID.
    ///
    /// Arguments:
    ///     game_id (str): A string of the game's ID.
    ///     content (str): A string of the content to write to the RC file.
    ///
    /// Example:
    ///     webtile.set_rc_file("dcss-web-trunk", "show_more = false\nrest_delay = -1")
    fn set_rc_file(&mut self, game_id: &str, content: &str) -> PyResult<()> {
        self.webtile
            .set_rc_file(game_id, content)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Get the RC file content for a specific game ID.
    ///
    /// Arguments:
    ///     game_id (str): A string of the game's ID.
    ///
    /// Example:
    ///     webtile.get_rc_file("dcss-web-trunk")
    fn get_rc_file(&mut self, game_id: &str) -> PyResult<String> {
        self.webtile
            .get_rc_file(game_id)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Start an unseeded game by selecting the game_id and the character's
    /// specifications.
    ///
    /// Arguments:
    ///     game_id (str): A string of the game's ID.
    ///     species (str): A string for the character's species.
    ///     background (str): A string for the character's background.
    ///     weapon (str): A string for the character's weapon.
    ///
    /// Example:
    ///     # Start a game on "dcss-web-trunk", for a Minotaur (b), Berserker (i), with a mace (b)
    ///     webtile.start_game("dcss-web-trunk", "b", "i", "b")
    fn start_game(
        &mut self,
        game_id: &str,
        species: &str,
        background: &str,
        weapon: &str,
    ) -> PyResult<()> {
        self.webtile
            .start_game(game_id, species, background, weapon)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Start an seeded game by selecting the game_id, the seed and the character's
    /// specifications.
    ///
    /// Arguments:
    ///     game_id (str): A string of the game's ID.
    ///     seed (str): A string of the game's seed.
    ///     pregenerate (bool): A bool on if the pregeneration option should be selected.
    ///     species (str): A string for the character's species.
    ///     background (str): A string for the character's background.
    ///     weapon (str): A string for the character's weapon.
    ///
    /// Example:
    ///     # Start a game on "dcss-web-trunk", for the "123" seed (pregenerated) for a
    ///     # Minotaur (b), Berserker (i), with a mace (b)
    ///     webtile.start_game_seeded("dcss-web-trunk", "123", true, "b", "i", "b")
    fn start_game_seeded(
        &mut self,
        game_id: &str,
        seed: &str,
        pregenerate: bool,
        species: &str,
        background: &str,
        weapon: &str,
    ) -> PyResult<()> {
        self.webtile
            .start_game_seeded(game_id, seed, pregenerate, species, background, weapon)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Save a game by sending the `CTRL + S` command.
    ///
    /// Example:
    ///     webtile.save_game()
    fn save_game(&mut self) -> PyResult<()> {
        self.webtile
            .save_game()
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Continue a saved game by selecting it's game ID.
    ///
    /// Arguments:
    ///     game_id (str): A string of the game's ID.
    ///
    /// Example:
    ///     # Continue a game on "dcss-web-trunk"
    ///     webtile.continue_game("dcss-web-trunk")
    fn continue_game(&mut self, game_id: &str) -> PyResult<()> {
        self.webtile
            .continue_game(game_id)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Quit the game (same result as dying), by sending a `CTRL + Q` and
    /// answering `yes`.
    ///
    /// Example:
    ///     webtile.quit_game()
    fn quit_game(&mut self) -> PyResult<()> {
        self.webtile
            .quit_game()
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }
}

#[pymodule]
pub fn dcss_api(py: Python<'_>, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WebtilePy>()?;
    m.add("BlockingErr", py.get_type_bound::<BlockingErr>())?;
    m.add("APIErr", py.get_type_bound::<APIErr>())?;
    Ok(())
}
