# How To Use This Library

### Creating a Webhook Client
To Create a webhook client, create the config first:  
```rust
Config::new("url".to_string(), "username".to_string(), "avatar_url".to_string())
```
This returns a `Config` and writes this config to `config.json`.  

Or, you can read from an existing file:
```rust
Config::from_file("./config.json")
```
This reads `config.json` file, creates and returns a new `Config`.  

Then, you can create the client by:
```rust
Client::new(config)
```
This returns a Client, and you can send messages!

___
### Sending Messages
`client.send()` receives a `Message` struct, and returns a `StatusCode` from the webhook.
To create a `Message` struct, you can do:
```rust
Message {
    username: config.username,
    avatar_url: config.avatar_url,
    content: "blah".to_string(),
    embeds: vec![]
}
```
Or, you can do
```rust
Message::new(
    "captain hook", // config.username.as_str()
    "https://avatars.githubusercontent.com/u/47074495", // config.webhook_url.as_str()
    "blah",
    embeds: vec![]
)
```

___
### Adding Embeds To Message
You can create an embed using the `Embed` struct, like this
```rust
Embed {
    author: Author,
    title: "hi".to_string(),
    url: "https://github.com/sourtaste000/webhook".to_string(),
    description: "hello".to_string(),
    color: 10195199u64,
    fields: vec![Field],
    thumbnail: Thumbnail,
    image: Image,
    footer: Footer,
}
```

#### Creating an `Author`
```rust
Author {
    name: "sourTaste000".to_string(),
    url: "https://github.com/sourtaste000/webhook".to_string(),
    icon_url: "https://avatars.githubusercontent.com/u/47074495".to_string(),
}
```

#### Creating a `Field`
```rust
Field {
    "Field Title".to_string(),
    "Field Content".to_string(),
    false, // or true for inline
}
```

#### Creating a `Thumbnail`
```rust
Field {
    "Field Title".to_string(),
    "Field Content".to_string(),
    false, // or true for inline
}
```

#### Creating an `Image`
```rust
Image {
    url: "https://avatars.githubusercontent.com/u/47074495".to_string()
}
```

#### Creating a `Footer`
```rust
Footer {
    text: "Footer Text".to_string(),
    icon_url: "https://github.com/sourtaste000/webhook".to_string()
}
```

You can also use the `new()` function from Embed, which is easier, and you can also fill in `None`:
```rust
Embed::new(None, // Author
          "Embed Title", // Title
          None, // URL
          None, // Description
          None, // Color
          None, // Fields
          None, //Thumbnail URL
          None, // Image URL
          None, // Footer URL
)
```

___
### Working(I hope) Example:
```rust
fn main() {
    let config: Config = Config::new("https://discord.com/api/webhooks/1234567890/abcdefghijk".to_string(), "captain hook", "https://avatars.githubusercontent.com/u/47074495");
    // or
    let read_config: Config = Config::from_file("config.json");
    let client: Client = Client::new(config.clone());
    // You can also use the structs to create a Message
    let message = Message::new(
        "captain hook", // config.username.as_str()
        "https://avatars.githubusercontent.com/u/47074495", // config.webhook_url.as_str()
        "blah",
        vec![Embed::new(
            Author::new("Author Name", 
                        "https://github.com/sourtaste000/webhook",
                        "https://cdn.discordapp.com/avatars/771003776617676830/247fafa69351450863baae74bd102867.png",
            ),
            "Embed Title",
            "https://github.com/sourtaste000/webhook",
            "Embed Description",
            10195199u64,
            vec![
                Field::new(
                    "Inline Field",
                    "Field Description",
                    true
                ),
                Field::new(
                    "Non-Inline Field",
                    "Field Description",
                    false
                )
            ],
            "https://cdn2.thecatapi.com/images/MjA2ODcyMg.jpg",
            "https://cdn2.thecatapi.com/images/MTc1NjgwOQ.jpg",
            Footer::new(
                "Footer Text",
                "https://cdn.discordapp.com/avatars/771003776617676830/247fafa69351450863baae74bd102867.png"
            )
        )]
    );
    client.send(message);
}
```
I know creating embeds is a pain, expect a rewrite coming soon™

___
### Extras
Provides a `input()` function.  
Prompts a message and asks for input, and stores the result in a variable.  
```rust
fn main() {
    let mut content = String::from("");
    input("Enter the message content: ", &mut content);
    // Suppose that the user entered "Hello World!"
    assert_eq(content, "Hello World!".to_string())
}
```