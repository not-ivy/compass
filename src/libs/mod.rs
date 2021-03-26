pub mod structs;

// TODO: Macros are better, but I'm too dumb to figure it out

use crate::libs::structs::{Message, Embed, Author, Image, Thumbnail, Footer, Field};

pub fn gen_message(username: &str, avatar_url: &str, content: &str, embeds: Vec<Embed>) -> Message {
    Message {
        username: username.to_string(),
        avatar_url: avatar_url.to_string(),
        content: content.to_string(),
        embeds,
    }
}

pub fn gen_embed() -> Embed {
    Embed {
        author: Author {
            name: "Author Name".to_string(),
            url: "https://github.com/sourTaste000".to_string(),
            icon_url: "https://cdn.discordapp.com/avatars/556312626058231828/afa950d0270ef001685d1f4c75df8d16.png".to_string()
        },
        title: "Embed Title".to_string(),
        url: "https://github.com/sourTaste000".to_string(),
        description: "Embed Description".to_string(),
        color: 10195199,
        fields: vec![gen_field("Inline Field", "Field value", true), gen_field("Not Inline Field", "Field Value", false)],
        thumbnail: Thumbnail {
            url: "https://cdn.discordapp.com/avatars/556312626058231828/afa950d0270ef001685d1f4c75df8d16.png".to_string()
        },
        image: Image {
            url: "https://cdn.discordapp.com/avatars/556312626058231828/afa950d0270ef001685d1f4c75df8d16.png".to_string()
        },
        footer: Footer {
            text: "footer text".to_string(),
            icon_url: "https://cdn.discordapp.com/avatars/556312626058231828/afa950d0270ef001685d1f4c75df8d16.png".to_string()
        }
    }
}

pub fn gen_field(name: &str, value: &str, inline: bool) -> Field {
    Field {
        name: name.to_string(),
        value: value.to_string(),
        inline
    }
}

// fn gen_author() -> Author {
//     Author {}
// }