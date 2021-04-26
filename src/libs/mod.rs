pub mod structs;

use crate::libs::structs::{
    Author, Client, Config, Embed, Field, Footer, Image, Message, Thumbnail,
};
use reqwest::StatusCode;
use std::fs;

pub(crate) trait Send {
    fn send(&self, message: Message) -> StatusCode;
}

impl Client {
    pub fn new(config: Config) -> Client {
        Client { config }
    }
}

impl Send for Client {
    fn send(&self, message: Message) -> StatusCode {
        let response = reqwest::blocking::Client::new()
            .post(&self.config.webhook_url)
            .json(&message)
            .send()
            .expect("Error while sending request to webhook!");
        response.status()
    }
}

impl Config {
    pub fn new(webhook_url: String, username: String, avatar_url: String) -> Config {
        let config = Config {
            webhook_url: webhook_url.trim().to_string(),
            username: username.trim().to_string(),
            avatar_url: avatar_url.trim().to_string(),
        };

        let _write = fs::write(
            "config.json",
            serde_json::to_string(&config).expect("Failed to serialize to json!"),
        )
        .expect("Failed to write to file!");

        config
    }

    pub fn from_file(path: &str) -> Config {
        let config: Config = serde_json::from_str(
            fs::read_to_string(path)
                .expect("Failed while reading the config!")
                .as_str(),
        )
        .expect("Error while parsing the config!");
        config
    }
}

impl Message {
    pub fn new(
        username: &str,
        avatar_url: &str,
        content: &str,
        embeds: Option<Vec<Embed>>,
    ) -> Message {
        Message {
            username: username.to_string(),
            avatar_url: avatar_url.to_string(),
            content: content.to_string(),
            embeds: match embeds {
                Some(embeds) => embeds,
                None => vec![],
            },
        }
    }
}

impl Embed {
    pub fn new(
        author: Option<Author>,
        title: &str,
        url: Option<&str>,
        description: Option<&str>,
        color: Option<u64>,
        fields: Option<Vec<Field>>,
        thumbnail_url: Option<&str>,
        image_url: Option<&str>,
        footer: Option<Footer>,
    ) -> Embed {
        Embed {
            author: match author {
                Some(author) => author,
                None => Author::new("", None, None),
            },
            title: title.to_string(),
            url: match url {
                Some(url) => url.to_string(),
                None => String::from(""),
            },
            description: match description {
                Some(description) => description.to_string(),
                None => String::from(""),
            },
            color: match color {
                Some(color) => color,
                None => 0_u64,
            },
            fields: match fields {
                Some(field) => field,
                None => vec![],
            },
            thumbnail: Thumbnail {
                url: match thumbnail_url {
                    Some(thumbnail_url) => thumbnail_url.to_string(),
                    None => String::from(""),
                },
            },
            image: Image {
                url: match image_url {
                    Some(image_url) => image_url.to_string(),
                    None => String::from(""),
                },
            },
            footer: match footer {
                Some(footer) => footer,
                None => Footer {
                    text: String::from(""),
                    icon_url: String::from(""),
                },
            },
        }
    }
}

impl Field {
    pub fn new(name: &str, value: &str, inline: bool) -> Field {
        Field {
            name: name.to_string(),
            value: value.to_string(),
            inline,
        }
    }
}

impl Author {
    pub fn new(name: &str, url: Option<&str>, icon_url: Option<&str>) -> Author {
        Author {
            name: name.to_string(),
            url: match url {
                Some(url) => url.to_string(),
                None => String::from(""),
            },
            icon_url: match icon_url {
                Some(icon_url) => icon_url.to_string(),
                None => String::from(""),
            },
        }
    }
}

impl Footer {
    pub fn new(text: &str, icon_url: Option<&str>) -> Footer {
        Footer {
            text: text.to_string(),
            icon_url: match icon_url {
                Some(icon_url) => icon_url.to_string(),
                None => String::from(""),
            },
        }
    }
}

// "wow such magic"
//              - Doge
pub fn input(prompt: &str, var: &mut String) {
    println!("{}", prompt);
    let _ = std::io::stdin().read_line(var).unwrap();
}