use anyhow::{anyhow, Result};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockingError {
    #[error("Blocking due to 'more' message.")]
    More,
    #[error("Blocking due to 'attributes' level up message (select 'S', 'I', 'D').")]
    Attributes,
    #[error("Blocking due to a pickup menu popup.")]
    Pickup,
    #[error("Blocking due to a 'acquirement' menu popup.")]
    Acquirement,
    #[error("Blocking due to a 'identify' menu popup.")]
    Identify,
    #[error("Blocking due to a 'enchant weapon' menu popup.")]
    EnchantWeapon,
    #[error("Blocking due to a 'brand item' menu popup.")]
    EnchantItem,
    #[error("Blocking due to a 'brand weapon' menu popup.")]
    BrandWeapon,
    #[error("Character died.")]
    Died,
}

pub(crate) fn blocking_messages(message: &Value) -> Result<()> {
    let msg = message["msg"].as_str().unwrap();

    match msg {
        "input_mode" => {
            if message["mode"].as_u64().unwrap() == 5 {
                Err(anyhow!(BlockingError::More))
            } else if message["mode"].as_u64().unwrap() == 7 {
                Err(anyhow!(BlockingError::Attributes))
            } else {
                Ok(())
            }
        }
        "menu" => {
            if message["tag"] == "pickup" {
                Err(anyhow!(BlockingError::Pickup))
            } else if message["tag"] == "acquirement" {
                Err(anyhow!(BlockingError::Acquirement))
            } else if message["tag"] == "use_item" {
                match message["title"]["text"].as_str().unwrap() {
                    x if x.contains("Identify which item?") => {
                        Err(anyhow!(BlockingError::Identify))
                    }
                    x if x.contains("Enchant which weapon?") => {
                        Err(anyhow!(BlockingError::EnchantWeapon))
                    }
                    x if x.contains("Enchant which item?") => {
                        Err(anyhow!(BlockingError::EnchantItem))
                    }
                    x if x.contains("Brand which weapon?") => {
                        Err(anyhow!(BlockingError::BrandWeapon))
                    }
                    _ => Ok(()),
                }
            } else {
                Ok(())
            }
        }
        "msgs" => {
            if !message.as_object().unwrap().contains_key("messages") {
                Ok(())
            } else {
                for text_obj in message["messages"].as_array().unwrap() {
                    let text = text_obj["text"].as_str().unwrap();

                    if text.contains("You die...") {
                        return Err(anyhow!(BlockingError::Died));
                    }
                }
                Ok(())
            }
        }
        _ => Ok(()),
    }
}
