mod libs;

use crate::libs::structs::{Config, Embed, Message};
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
    let selection = String::new();
    let embed: Embed;

    println!("Please enter the message content;");
    let _read = std::io::stdin().read_line(&mut content).unwrap();

    println!("Embeds? (Y/n)");
    let _read = std::io::stdin().read_line(&mut content).unwrap();
    if selection.trim().to_lowercase() == "y" || selection.trim().is_empty() {
        embed = gen_embed(
            gen_author("", "", ""),
            "test",
            "https://example.com",
            "hello",
            10195199,
            vec![gen_field("Field 1", "Value 1", false)],
            "",
            "",
            gen_footer("Footer", ""),
        )
    } else {
        return gen_message(
            config.username.as_str(),
            config.avatar_url.as_str(),
            content.as_str(),
            vec![],
        );
    }

    return gen_message(
        config.username.as_str(),
        config.avatar_url.as_str(),
        content.as_str(),
        vec![embed],
    );
}
