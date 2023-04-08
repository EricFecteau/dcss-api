use crate::Error;
use crate::Webtile;
use serde_json::json;

impl Webtile {
    pub fn login_with_credentials(&mut self, username: &str, password: &str) -> Result<(), Error> {
        self.write_json(json!({
            "msg": "login",
            "username": username,
            "password": password,
        }))?;

        self.read_until("login_success", None, None)?;

        Ok(())
    }

    pub fn login_with_cookie(&mut self, cookie: &str) -> Result<(), Error> {
        self.write_json(json!({"msg": "token_login", "cookie": cookie}))?;

        self.read_until("login_success", None, None)?;

        Ok(())
    }

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

    pub fn get_rc_file(&mut self, game_id: &str) -> Result<(), Error> {
        self.write_json(json!({"msg": "get_rc", "game_id": game_id}))?;

        self.read_until("rcfile_contents", None, None)?;

        Ok(())
    }

    pub fn set_rc_file(&mut self, game_id: &str, content: &str) -> Result<(), Error> {
        self.write_json(json!({"msg": "set_rc", "game_id": game_id, "contents": content}))?;

        self.read_until("set_game_links", None, None)?;

        Ok(())
    }
}
