mod libs;

use crate::libs::structs::{Client, Config, Embed, Field, Message};
use crate::libs::*;
use std::path::Path;

fn main() {
    let webhook_config = if !Path::new("config.json").exists() {
        config_guide()
    } else {
        Config::from_file("config.json")
    };
    let webhook_client = Client::new(webhook_config.clone());
    let response = webhook_client.send(guide(&webhook_config));
    println!("{}", response.to_string())
}

fn config_guide() -> Config {
    let (mut webhook_url,mut username, mut avatar_url) = (String::from(""), String::from(""), String::from(""));

    input("Please enter the webhook url:", &mut webhook_url);

    input("Please enter username desired for webhook:", &mut username);

    input("Please enter desired avatar's url for webhook:", &mut avatar_url);

    if webhook_url.is_empty() || username.is_empty() || avatar_url.is_empty() {
        panic!("You did not enter something correctly!")
    }
    Config::new(webhook_url, username, avatar_url)
}

fn guide(config: &Config) -> Message {
    let (mut title, mut content, mut selection, mut url, mut thumbnail_url, mut image_url) = (String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""));
    let embed: Embed;

    input("Please enter the message content;", &mut content);

    input("Embeds? (Y/n)", &mut selection);
    return if selection.trim().to_lowercase() == "y" || selection.trim().is_empty() {
        embed = Embed::new(
            None,
            {
                input("Please enter the Embed title: ", &mut title);
                if title.is_empty() {
                    "Title not specified"
                } else {
                    title.as_str()
                }
            },
            {
                input("Please enter the Embed url(skip if none): ", &mut url);
                if url.is_empty() {
                    None
                } else {
                    Option::from(url.as_str())
                }
            },
            Option::from("None"),
            None,
            Option::from(vec![{
                let (mut field_title, mut field_content, mut inline) = (String::from(""), String::from(""), String::from(""));

                input("Please enter the field title:", &mut field_title);

                input("Please enter the field content:", &mut field_content);

                input("Inline? (Y/n)", &mut inline);

                Field {
                    name: field_title,
                    value: field_content,
                    inline: inline.trim().to_lowercase() == "y" || inline.is_empty(),
                }
            }]),
            {
                input("Please enter the thumbnail url for embed (skip if none): ", &mut thumbnail_url);
                if thumbnail_url.is_empty() {
                    None
                } else {
                    Option::from(thumbnail_url.as_str())
                }
            },
            {
                input("Please enter the image url for embed (skip if none): ", &mut image_url);
                if image_url.is_empty() {
                    None
                } else {
                    Option::from(image_url.as_str())
                }
            },
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
