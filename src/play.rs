use crate::api_errors::BlockingError;
use crate::Error;
use crate::Webtile;
use serde_json::json;

impl Webtile {
    /// Start an unseeded game by selecting the game_id and the character's
    /// specifications.
    ///
    /// # Arguments
    ///
    /// * `game_id` - A string slice of the game's ID.
    /// * `species` - A string slice for the character's species.
    /// * `background` - A string slice for the character's background.
    /// * `weapon` - A string slice for the character's weapon.
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Start a game on "dcss-web-trunk", for a Minotaur (b), Berserker (i), with a mace (b)
    /// webtile.start_game("dcss-web-trunk", "b", "i", "b")?;
    /// ```
    pub fn start_game(
        &mut self,
        game_id: &str,
        species: &str,
        background: &str,
        weapon: &str,
    ) -> Result<(), Error> {
        self.start_game_seeded(game_id, "0", false, species, background, weapon)
    }

    /// Continue a saved game by selecting it's game ID.
    ///
    /// # Arguments
    ///
    /// * `game_id` - A string slice of the game's ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Continue a game on "dcss-web-trunk"
    /// webtile.continue_game("dcss-web-trunk")?;
    /// ```
    pub fn continue_game(&mut self, game_id: &str) -> Result<(), Error> {
        self.start_game_seeded(game_id, "", false, "", "", "")
    }

    /// Start an seeded game by selecting the game_id, the seed and the character's
    /// specifications.
    ///
    /// # Arguments
    ///
    /// * `game_id` - A string slice of the game's ID.
    /// * `seed` - A string slice of the game's seed.
    /// * `pregenerate` - A bool on if the pregeneration option should be selected.
    /// * `species` - A string slice for the character's species.
    /// * `background` - A string slice for the character's background.
    /// * `weapon` - A string slice for the character's weapon.
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Start a game on "dcss-web-trunk", for the "123" seed (pregenerated) for a
    /// // Minotaur (b), Berserker (i), with a mace (b)
    /// webtile.start_game_seeded("dcss-web-trunk", "123", true, "b", "i", "b")?;
    /// ```
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

    /// Save a game by sending the `CTRL + S` command.
    ///
    /// # Example
    ///
    /// ```no_run
    /// webtile.save_game()?;
    /// ```
    pub fn save_game(&mut self) -> Result<(), Error> {
        self.write_key("key_ctrl_s")?;

        self.read_until("go_lobby", None, None)?;

        Ok(())
    }

    /// Quit the game (same result as dying), by sending a `CTRL + Q` and
    /// answering `yes`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// webtile.quit_game()?;
    /// ```
    pub fn quit_game(&mut self) -> Result<(), Error> {
        self.write_key("key_ctrl_q")?;

        match self.read_until("input_mode", Some("mode"), Some(7)) {
            Ok(_) => (),
            Err(e) => match e {
                Error::Blocking(BlockingError::TextInput) => {
                    self.write_key("yes")?;
                    self.write_key("key_enter")?;
                    self.message_found = false; // Otherwise close_input will be skipped
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
