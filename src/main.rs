mod libs;

use crate::libs::structs::{Client, Config, Embed, Field, Message};
use crate::libs::*;
use std::path::Path;

fn main() {
    let webhook_config = if !Path::new("config.json").exists() {
        // fill this in (webhook_url, username, avatar_url)
        config_guide()
    } else {
        Config::from_file("config.json")
    };
    let webhook_client = Client::new(webhook_config.clone());
    let response = webhook_client.send(guide(&webhook_config));
    println!("{}", response.to_string())
}

fn config_guide() -> Config {
    let mut webhook_url: String = String::from("");
    let mut username: String = String::from("");
    let mut avatar_url: String = String::from("");

    println!("Please enter the webhook url:");
    let _read = std::io::stdin().read_line(&mut webhook_url).unwrap();

    println!("Please enter username desired for webhook:");
    let _read = std::io::stdin().read_line(&mut username).unwrap();

    println!("Please enter desired avatar's url for webhook:");
    let _read = std::io::stdin().read_line(&mut avatar_url).unwrap();

    if webhook_url.is_empty() || username.is_empty() || avatar_url.is_empty() {
        panic!("You did not enter something correctly!")
    }
    Config::new(webhook_url, username, avatar_url)
}

fn guide(config: &Config) -> Message {
    let mut title: String = String::from("");
    let mut content: String = String::from("");
    let mut selection: String = String::from("");
    let mut url: String = String::from("");
    let mut thumbnail_url: String = String::from("");
    let mut image_url: String = String::from("");
    let embed: Embed;

    println!("Please enter the message content;");
    let _read = std::io::stdin().read_line(&mut content).unwrap();

    println!("Embeds? (Y/n)");
    let _read = std::io::stdin().read_line(&mut selection).unwrap();
    return if selection.trim().to_lowercase() == "y" || selection.trim().is_empty() {
        embed = Embed::new(
            None,
            {
                println!("Please enter the Embed title: ");
                let _read = std::io::stdin().read_line(&mut title).unwrap();
                if title.is_empty() {
                    "Title not specified"
                } else {
                    title.as_str()
                }
            },
            {
                println!("Please enter the Embed url(skip if none): ");
                let _read = std::io::stdin().read_line(&mut url).unwrap();
                if url.is_empty() {
                    None
                } else {
                    Option::from(url.as_str())
                }
            },
            Option::from("None"),
            None,
            Option::from(vec![{
                let mut field_title: String = String::from("");
                let mut field_content: String = String::from("");
                let mut inline: String = String::from("");

                println!("Please Enter your field title");
                let _read = std::io::stdin().read_line(&mut field_title).unwrap();

                println!("Please enter field content:");
                let _read = std::io::stdin().read_line(&mut field_content).unwrap();

                println!("Inline? (Y/n)");
                let _read = std::io::stdin().read_line(&mut inline).unwrap();

                Field {
                    name: field_title,
                    value: field_content,
                    inline: inline.trim().to_lowercase() == "y" || inline.is_empty(),
                }
            }]),
            {
                println!("Please enter the thumbnail url for embed (skip if none): ");
                let _read = std::io::stdin().read_line(&mut thumbnail_url).unwrap();
                if thumbnail_url.is_empty() {
                    None
                } else {
                    Option::from(thumbnail_url.as_str())
                }
            },
            {
                println!("Please enter the image url for embed (skip if none): ");
                let _read = std::io::stdin().read_line(&mut image_url).unwrap();
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
