mod libs;

use std::fs;
use std::path::Path;
use crate::libs::*;
use crate::libs::structs::Config;

fn main() {
    let client = reqwest::blocking::Client::new();

    if !Path::new("config.json").exists() {
        eprintln!("Config file not detected; Generating one...");
        println!("Please enter your discord webhook url here:");
        let mut webhook_url = String::new();
        let _read = std::io::stdin().read_line(&mut webhook_url).unwrap();
        let config = Config {
            webhook_url: webhook_url.trim().to_string(),
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
    // println!("{}", serde_json::to_string(&gen_message()).expect("Error while serializing the message!"))
    let response = client.post(&config.webhook_url)
        .json(&gen_message("usrname", "assd", "ajfhslkfhlaskd", vec![gen_embed()]))
        .send()
        .expect("Error while sending request to webhook!");
    println!("{}", response.status())
}