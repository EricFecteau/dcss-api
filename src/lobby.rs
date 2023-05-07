use crate::Error;
use crate::Webtile;
use serde_json::json;

impl Webtile {
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

    pub fn login_with_cookie(&mut self, cookie: &str) -> Result<Vec<String>, Error> {
        self.write_json(json!({"msg": "token_login", "cookie": cookie}))?;

        self.read_until("login_success", None, None)?;

        self.write_json(json!({
            "msg": "go_lobby"
        }))?;
        self.read_until("go_lobby", None, None)?;

        Ok(self.get_playable_games())
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

    pub fn set_rc_file(&mut self, game_id: &str, content: &str) -> Result<(), Error> {
        self.write_json(json!({"msg": "set_rc", "game_id": game_id, "contents": content}))?;

        Ok(())
    }

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

fn process_playable_game(game_list_html: &str) -> Vec<String> {
    let mut game_list_vec = game_list_html
        .split('#')
        .map(|x| x.split('\"').next().unwrap_or("").to_owned())
        .map(|x| x[5..x.len()].to_owned())
        .collect::<Vec<String>>();

    game_list_vec.remove(0);

    game_list_vec
}
