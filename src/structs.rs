use serde_derive::{Deserialize, Serialize};

pub struct Client {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub webhook_url: String,
    pub username: String,
    pub avatar_url: String,
}

#[derive(Serialize)]
pub struct Message {
    pub username: String,
    pub avatar_url: String,
    pub content: String,
    pub embeds: Vec<Embed>,
}

#[derive(Serialize)]
pub struct Embed {
    pub author: Author,
    pub title: String,
    pub url: String,
    pub description: String,
    pub color: u64,
    pub fields: Vec<Field>,
    pub thumbnail: Thumbnail,
    pub image: Image,
    pub footer: Footer,
}

#[derive(Serialize)]
pub struct Author {
    pub name: String,
    pub url: String,
    pub icon_url: String,
}

#[derive(Serialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize)]
pub struct Thumbnail {
    pub url: String,
}

#[derive(Serialize)]
pub struct Image {
    pub url: String,
}

#[derive(Serialize)]
pub struct Footer {
    pub text: String,
    pub icon_url: String,
}