use crate::api_errors::BlockingError;
use crate::Error;
use crate::Webtile;
use serde_json::json;

impl Webtile {
    pub fn start_game(
        &mut self,
        game_id: &str,
        species: &str,
        background: &str,
        weapon: &str,
    ) -> Result<(), Error> {
        self.start_game_seeded(game_id, "0", false, species, background, weapon)
    }

    pub fn continue_game(&mut self, game_id: &str) -> Result<(), Error> {
        self.start_game_seeded(game_id, "", false, "", "", "")
    }

    pub fn start_game_seeded(
        &mut self,
        game_id: &str,
        seed: &str,
        pregenerate: bool,
        species: &str,
        background: &str,
        weapon: &str,
    ) -> Result<(), Error> {
        self.write_json(json!({"msg": "play", "game_id": game_id}))?;

        let mut newgame_count = 0;
        loop {
            match self.read_until("map", None, None) {
                Ok(_) => return Ok(()),
                Err(e) => match e {
                    Error::Blocking(BlockingError::SeedSelection) => {
                        self.write_key("-")?;
                        self.read_until("ui-state-sync", None, None)?;
                        self.write_key(seed)?;
                        if pregenerate {
                            self.write_key("\t\t\t \r")?;
                        } else {
                            self.write_key("\r")?;
                        }
                    }
                    Error::Blocking(BlockingError::NewGameChoice) => {
                        match newgame_count {
                            0 => self.write_key(species)?,
                            1 => self.write_key(background)?,
                            2 => self.write_key(weapon)?,
                            _ => unreachable!(),
                        }

                        newgame_count += 1;
                    }
                    _ => return Err(e),
                },
            };
        }
    }

    pub fn save_game(&mut self) -> Result<(), Error> {
        self.write_key("key_ctrl_s")?;

        self.read_until("go_lobby", None, None)?;

        Ok(())
    }

    pub fn quit_game(&mut self) -> Result<(), Error> {
        self.write_key("key_ctrl_q")?;

        match self.read_until("input_mode", Some("mode"), Some(7)) {
            Ok(_) => (),
            Err(e) => match e {
                Error::Blocking(BlockingError::TextInput) => {
                    let rom = self.read_only_messages();
                    if !(rom[rom.len() - 1]["msg"] == "init_input"
                        || rom[rom.len() - 2]["msg"] == "init_input")
                    {
                        self.read_until("init_input", None, None)?;
                    };
                    self.write_key("yes")?;
                    self.write_key("key_enter")?;
                }
                _ => return Err(e),
            },
        };

        match self.read_until("close_input", None, None) {
            Ok(_) => (),
            Err(e) => match e {
                Error::Blocking(BlockingError::More) => self.write_key("key_esc")?,
                _ => return Err(e),
            },
        };

        loop {
            self.write_key("key_esc")?;
            match self.read_until("go_lobby", None, None) {
                Ok(_) => return Ok(()),
                Err(e) => match e {
                    Error::Blocking(BlockingError::More) => (),
                    _ => return Err(e),
                },
            };
        }
    }
}
