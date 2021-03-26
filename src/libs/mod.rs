pub mod structs;

// TODO: Macros are better, but I'm too dumb to figure it out

use crate::libs::structs::{Author, Embed, Field, Footer, Image, Message, Thumbnail};

pub fn gen_message(username: &str, avatar_url: &str, content: &str, embeds: Vec<Embed>) -> Message {
    Message {
        username: username.to_string(),
        avatar_url: avatar_url.to_string(),
        content: content.to_string(),
        embeds,
    }
}

pub fn gen_embed(
    author: Author,
    title: &str,
    url: &str,
    description: &str,
    color: u64,
    fields: Vec<Field>,
    thumbnail_url: &str,
    image_url: &str,
    footer: Footer,
) -> Embed {
    Embed {
        author,
        title: title.to_string(),
        url: url.to_string(),
        description: description.to_string(),
        color,
        fields,
        thumbnail: Thumbnail {
            url: thumbnail_url.to_string(),
        },
        image: Image {
            url: image_url.to_string(),
        },
        footer,
    }
}

pub fn gen_field(name: &str, value: &str, inline: bool) -> Field {
    Field {
        name: name.to_string(),
        value: value.to_string(),
        inline,
    }
}

pub fn gen_author(name: &str, url: &str, icon_url: &str) -> Author {
    Author {
        name: name.to_string(),
        url: url.to_string(),
        icon_url: icon_url.to_string(),
    }
}

pub fn gen_footer(text: &str, icon_url: &str) -> Footer {
    Footer {
        text: text.to_string(),
        icon_url: icon_url.to_string(),
    }
}
