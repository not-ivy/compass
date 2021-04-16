mod libs;

use crate::libs::structs::{Author, Config, Embed, Field, Footer, Message};
use crate::libs::*;
use std::fs;
use std::path::Path;

fn main() {
    // TODO: Use non-blocking client
    let client = reqwest::blocking::Client::new();

    if !Path::new("config.json").exists() {
        eprintln!("Config file not detected; Generating one...");
        gen_config()
    }

    let config: Config = serde_json::from_str(
        fs::read_to_string("config.json")
            .expect("Failed while reading the config!")
            .as_str(),
    )
    .expect("Error while parsing the config!");

    let response = client
        .post(&config.webhook_url)
        .json(&guide(&config))
        .send()
        .expect("Error while sending request to webhook!");
    println!("The Webhook responded with: {}", response.status());
}

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
