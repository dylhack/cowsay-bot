pub mod images;
use crate::types::Result;
use charasay::{format_character, Chara::Builtin};

pub fn cowsay(character: &str, msg: &str) -> Result<String> {
    let cow = Builtin(String::from(character));
    let result = format_character(msg, &cow, 80, charasay::bubbles::BubbleType::Round);

    if let Err(why) = result {
        Err(format!("Failed to create cowsay, {}", why))
    } else {
        Ok(result.unwrap())
    }
}
