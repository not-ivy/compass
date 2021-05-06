# How To Use This Library

### Creating a Webhook Client
Then, you can create the client by:
```rust
Client::new("url".to_string(), "username".to_string(), "avatar_url".to_string())
```

Or, you can read from an existing file:
```rust
Client::from_file("./config.json")
```
This reads `config.json` file, creates and returns a new `Config`.

These two methods returns a Client, and you can send messages!

___
### Sending Messages
`client.send()` receives a `Message` struct, and returns a `StatusCode` from the webhook.
To create a `Message`, look at this code:
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
You can use the `new()` function from Embed. You can either fill in None or content.
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

#### Creating an `Author`
```rust
Author::new(
"name",
None, // url
None, // icon url
)
```

#### Creating a `Field`
```rust
Field::new(
"Field title",
"Field content",
false, // inline
)
```

#### Creating a `Footer`
```rust
Footer::new(
"Footer",
None, // icon url
)
```

___
### Working(I hope) Example:
```rust
fn main() {
    let client: Client = Client::new("https://discord.com/api/webhooks/1234567890/abcdefghijk".to_string(), "captain hook", "https://avatars.githubusercontent.com/u/47074495");
    // or
    let read_client: Client = Client::from_file("config.json");
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

~~For a full example, go to the `client` branch.~~ [OUTDATED]

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
