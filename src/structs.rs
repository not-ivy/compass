use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub (crate) struct Client {
    pub webhook_url: String,
    pub username: String,
    pub avatar_url: String,
}

#[derive(Serialize)]
pub (crate) struct Message {
    pub username: String,
    pub avatar_url: String,
    pub content: String,
    pub embeds: Vec<Embed>,
}

#[derive(Serialize)]
pub (crate) struct Embed {
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
pub (crate) struct Author {
    pub name: String,
    pub url: String,
    pub icon_url: String,
}

#[derive(Serialize)]
pub (crate) struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize)]
pub (crate) struct Thumbnail {
    pub url: String,
}

#[derive(Serialize)]
pub (crate) struct Image {
    pub url: String,
}

#[derive(Serialize)]
pub (crate) struct Footer {
    pub text: String,
    pub icon_url: String,
}