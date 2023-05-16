use crate::Error;
use crate::Webtile;
use serde_json::json;

impl Webtile {
    /// Login to the game, using a username and password. It returns a vector
    /// of all playable game IDs.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice of the user's username.
    /// * `password` - A string slice of the user's password.
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Login under the user "Username", with a password of "Password"
    /// webtile.login_with_credentials("Username", "Password")?;
    /// ```
    pub fn login_with_credentials(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<Vec<String>, Error> {
        self.write_json(json!({
            "msg": "login",
            "username": username,
            "password": password,
        }))?;

        self.read_until("login_success", None, None)?;

        self.write_json(json!({
            "msg": "go_lobby"
        }))?;
        self.read_until("go_lobby", None, None)?;

        Ok(self.get_playable_games())
    }

    /// Login to the game, using a cookie. It returns a vector of all playable
    /// game IDs.
    ///
    /// # Arguments
    ///
    /// * `cookie` - A string slice of the user's cookie (received previously).
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Login under the user "Username", with a cookie
    /// webtile.login_with_cookie("Username%123456789123456789123456789")?;
    /// ```
    pub fn login_with_cookie(&mut self, cookie: &str) -> Result<Vec<String>, Error> {
        self.write_json(json!({"msg": "token_login", "cookie": cookie}))?;

        self.read_until("login_success", None, None)?;

        self.write_json(json!({
            "msg": "go_lobby"
        }))?;
        self.read_until("go_lobby", None, None)?;

        Ok(self.get_playable_games())
    }

    /// Request a cookie from the DCSS Webtile.
    ///
    /// # Example
    ///
    /// ```no_run
    /// webtile.request_cookie()?;
    /// ```
    pub fn request_cookie(&mut self) -> Result<String, Error> {
        self.write_json(json!({"msg": "set_login_cookie"}))?;

        self.read_until("login_cookie", None, None)?;

        for message in self.read_only_messages() {
            let message_obj = message.as_object().unwrap();
            if message_obj["msg"] == "login_cookie" {
                return Ok(message_obj["cookie"].as_str().unwrap().to_owned());
            }
        }

        unreachable!()
    }

    /// Get the RC file content of a specific game ID.
    ///
    /// # Arguments
    ///
    /// * `game_id` - A string slice of the game's ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// webtile.get_rc_file("dcss-web-trunk")?;
    /// ```
    pub fn get_rc_file(&mut self, game_id: &str) -> Result<String, Error> {
        self.write_json(json!({"msg": "get_rc", "game_id": game_id}))?;

        self.read_until("rcfile_contents", None, None)?;

        for message in self.read_only_messages() {
            let message_obj = message.as_object().unwrap();
            if message_obj["msg"] == "rcfile_contents" {
                return Ok(message_obj["contents"].as_str().unwrap().to_owned());
            }
        }

        unreachable!()
    }

    /// Set the RC file content of a specific game ID.
    ///
    /// # Arguments
    ///
    /// * `game_id` - A string slice of the game's ID.
    /// * `content` - A string slice of the content to write to the RC file.
    ///
    /// # Example
    ///
    /// ```no_run
    /// webtile..set_rc_file("dcss-web-trunk", "show_more = false\nrest_delay = -1")?;
    /// ```
    pub fn set_rc_file(&mut self, game_id: &str, content: &str) -> Result<(), Error> {
        self.write_json(json!({"msg": "set_rc", "game_id": game_id, "contents": content}))?;

        Ok(())
    }

    /// Process the data received when successfully logging in, to extract the playable games
    fn get_playable_games(&self) -> Vec<String> {
        for message in self.read_only_messages() {
            let message_obj = message.as_object().unwrap();
            if message_obj["msg"] == "set_game_links" {
                return process_playable_game(message_obj["content"].as_str().unwrap());
            }
        }

        unreachable!()
    }
}

/// Process the data received when successfully logging in, to extract the playable games
fn process_playable_game(game_list_html: &str) -> Vec<String> {
    let mut game_list_vec = game_list_html
        .split('#')
        .map(|x| x.split('\"').next().unwrap_or("").to_owned())
        .map(|x| x[5..x.len()].to_owned())
        .collect::<Vec<String>>();

    game_list_vec.remove(0);

    game_list_vec
}
