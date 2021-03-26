mod libs;

use crate::libs::structs::{Config, Embed, Message};
use crate::libs::*;
use std::fs;
use std::path::Path;

fn main() {
    // TODO: Use non-blocking client
    let client = reqwest::blocking::Client::new();

    if !Path::new("config.json").exists() {
        let mut webhook_url = String::new();
        let mut username = String::new();
        let mut avatar_url = String::new();

        eprintln!("Config file not detected; Generating one...");
        println!("Please enter your discord webhook url here:");
        let _read = std::io::stdin().read_line(&mut webhook_url).unwrap();

        println!("Please enter your desired username for the webhook");
        let _read = std::io::stdin().read_line(&mut username).unwrap();

        println!("Please enter the desired avatar url for the webhook");
        let _read = std::io::stdin().read_line(&mut avatar_url).unwrap();

        let config = Config {
            webhook_url: webhook_url.trim().to_string(),
            username: username.trim().to_string(),
            avatar_url: avatar_url.trim().to_string(),
        };
        let _write = fs::write(
            "config.json",
            serde_json::to_string(&config)
                .expect("Failed to serialize to json!")
                .trim(),
        )
        .expect("Failed to write to file!");
    }

    let config: Config = serde_json::from_str(
        fs::read_to_string("config.json")
            .expect("Failed while reading the config!")
            .as_str(),
    )
    .expect("Error while parsing the config!");

    // println!("{}", config.webhook_url);
    // println!("{}", serde_json::to_string().expect("Error while serializing the message!"))
    let response = client.post(&config.webhook_url)
        .json(&menu(&config))
        .send()
        .expect("Error while sending request to webhook!");
    println!("The Webhook responded with: {}", response.status());
    println!("{}", response.text().unwrap());
}

fn menu(config: &Config) -> Message {
    println!(
        r#"
        1. Send Message
        2. Setting
    "#
    );
    let mut selection = String::new();
    let _read = std::io::stdin().read_line(&mut selection).unwrap();

    if selection.trim() == String::from("1") {
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
            )
        }

        return gen_message(
            config.username.as_str(),
            config.avatar_url.as_str(),
            content.as_str(),
            vec![embed],
        )
    } else {
        unimplemented!("hi")
    }
}
