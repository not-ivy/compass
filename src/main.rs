mod libs;

use crate::libs::structs::{Client, Config, Embed, Field, Message};
use crate::libs::*;

fn main() {
    let webhook_config = Config::from_file("config.json");
    let webhook_client = Client::new(webhook_config.clone());
    webhook_client.send(guide(&webhook_config));
}

// TODO: Macros, not function
/// ```
/// MessageBuilder! {
///     Title -> ""
///     Author -> ""
///     etc...
/// }
/// ```
fn guide(config: &Config) -> Message {
    let mut content = String::new();
    let mut selection = String::new();
    let embed: Embed;

    println!("Please enter the message content;");
    let _read = std::io::stdin().read_line(&mut content).unwrap();

    println!("Embeds? (Y/n)");
    let _read = std::io::stdin().read_line(&mut selection).unwrap();
    return if selection.trim().to_lowercase() == "y" || selection.trim().is_empty() {
        embed = Embed::new(
            None,
            "test",
            None,
            Option::from("None"),
            None,
            Option::from(vec![Field::new("Field Name 1", "Field Value 1", false)]),
            None,
            None,
            None,
        );
        Message::new(
            config.username.as_str(),
            config.avatar_url.as_str(),
            content.as_str(),
            Option::from(vec![embed]),
        )
    } else {
        Message::new(
            config.username.as_str(),
            config.avatar_url.as_str(),
            content.as_str(),
            None,
        )
    };
}
