use std::str::Utf8Error;

use serde_json::Value;
use thiserror::Error;

/// Main errors types that can be raised while using the API.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Tungstenite error: {0}")]
    Websocket(#[from] tungstenite::Error),
    #[error("Decompress error: {0}")]
    Decompress(#[from] flate2::DecompressError),
    #[error("JSON utf8 error: {0}")]
    Utf8(#[from] Utf8Error),
    #[error("JSON error: {0}")]
    JSON(#[from] serde_json::Error),
    #[error("Blocking error: {0}")]
    Blocking(#[from] BlockingError),
    #[error("Failed to login (bad username, password or cookie).")]
    LoginFailed,
    #[error("Failed to register.")]
    RegisterFailed,
}

/// Errors that will block the game from processing normally. Since each read
/// of the websocket requires an expected "end of read", this is a list of
/// unexpected data that would prevent expected results from being sent.
///
/// # Example
///
/// When picking up an item (i.e. ","), normally a "input_mode" with a "mode" of 1
/// would be received, but if there is more than one item where the character
/// is standing a "menu" with a "pickup" tag will instead be sent. Since this
/// is unexpected, `dcss-api` will send a "Pickup" BlockingError.
#[derive(Error, Debug)]
pub enum BlockingError {
    #[error("Custom seed selection menu.")]
    SeedSelection,
    #[error("New game choice selection menu.")]
    NewGameChoice,
    #[error("Blocking due to 'more' message.")]
    More,
    #[error("Blocking due to text input necessary from user (likely for level up message).")]
    TextInput,
    #[error("Blocking due to a pickup menu popup.")]
    Pickup,
    #[error("Blocking due to a 'acquirement' menu popup.")]
    Acquirement(Value),
    #[error("Blocking due to a 'identify' menu popup.")]
    Identify(Value),
    #[error("Blocking due to a 'enchant weapon' menu popup.")]
    EnchantWeapon(Value),
    #[error("Blocking due to a 'brand item' menu popup.")]
    EnchantItem(Value),
    #[error("Blocking due to a 'brand weapon' menu popup.")]
    BrandWeapon(Value),
    #[error("Blocking due to a 'skills to train' txt menu.")]
    Skill,
    #[error("Blocking due to a 'blink' action.")]
    Blink,
    #[error("Blocking due to an 'equipping' action.")]
    Equipping,
    #[error("Blocking due to an 'disrobing' action.")]
    Disrobing,
    #[error("Blocking due to a 'scroll of noise' read prompt.")]
    Noise,
    #[error("Character died.")]
    Died,
}

/// This function will "pre-process" each received message and return an
/// error if a BlockingError type message is received, through various
/// message types received by the DCSS webtile.
///
/// # Arguments
///
/// * `message` - The message (as a [serde_json::Value]) received by the
///   DCSS webtile.
pub(crate) fn blocking_messages(message: &Value) -> Result<(), Error> {
    let msg = message["msg"].as_str().unwrap();

    match msg {
        "input_mode" => {
            if message["mode"].as_u64().unwrap() == 5 {
                Err(Error::Blocking(BlockingError::More))
            } else if message["mode"].as_u64().unwrap() == 7 {
                Err(Error::Blocking(BlockingError::TextInput))
            } else {
                Ok(())
            }
        }
        "menu" => {
            if message["tag"] == "pickup" {
                Err(Error::Blocking(BlockingError::Pickup))
            } else if message["tag"] == "acquirement" {
                Err(Error::Blocking(BlockingError::Acquirement(message.clone())))
            } else if message["tag"] == "use_item" {
                match message["title"]["text"].as_str().unwrap() {
                    x if x.contains("Identify which item?") => {
                        Err(Error::Blocking(BlockingError::Identify(message.clone())))
                    }
                    x if x.contains("Enchant which weapon?") => Err(Error::Blocking(
                        BlockingError::EnchantWeapon(message.clone()),
                    )),
                    x if x.contains("Enchant which item?") => {
                        Err(Error::Blocking(BlockingError::EnchantItem(message.clone())))
                    }
                    x if x.contains("Brand which weapon?") => {
                        Err(Error::Blocking(BlockingError::BrandWeapon(message.clone())))
                    }
                    _ => Ok(()),
                }
            } else {
                Ok(())
            }
        }
        "txt" => {
            let lines_obj = message.as_object().unwrap()["lines"].as_object().unwrap();

            if lines_obj.contains_key("0")
                && lines_obj[&("0".to_string())]
                    .as_str()
                    .unwrap()
                    .to_owned()
                    .contains("Select the skills to train")
            {
                return Err(Error::Blocking(BlockingError::Skill));
            }

            Ok(())
        }
        "msgs" => {
            if !message.as_object().unwrap().contains_key("messages") {
                Ok(())
            } else {
                for text_obj in message["messages"].as_array().unwrap() {
                    let text = text_obj["text"].as_str().unwrap();

                    if text.contains("You die...") {
                        return Err(Error::Blocking(BlockingError::Died));
                    }

                    if text.contains("Blink to where?") {
                        return Err(Error::Blocking(BlockingError::Blink));
                    }

                    if text.contains("Really read the scroll of noise?") {
                        return Err(Error::Blocking(BlockingError::Noise));
                    }

                    if text.contains("Keep equipping yourself?") {
                        return Err(Error::Blocking(BlockingError::Equipping));
                    }

                    if text.contains("Keep disrobing?") {
                        return Err(Error::Blocking(BlockingError::Disrobing));
                    }
                }
                Ok(())
            }
        }
        "login_fail" => Err(Error::LoginFailed),
        "register_fail" => Err(Error::RegisterFailed),
        "ui-push" => {
            if !message.as_object().unwrap().contains_key("type") {
                Ok(())
            } else {
                if message["type"] == "seed-selection" {
                    return Err(Error::Blocking(BlockingError::SeedSelection));
                } else if message["type"] == "newgame-choice" {
                    return Err(Error::Blocking(BlockingError::NewGameChoice));
                }
                Ok(())
            }
        }
        _ => Ok(()),
    }
}
