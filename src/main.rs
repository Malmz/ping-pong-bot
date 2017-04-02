extern crate discord;

use std::env;
use discord::Discord;
use discord::model::Event;

fn main() {
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"))
        .expect("login failed");  
    
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("YEET IT WORKED!");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);
                if message.content == "!ping" {
                    let _ = discord.send_message(message.channel_id, "pong", "", false);
                } else if message.content.contains("yeet") {
                    let _ = discord.send_message(message.channel_id, "Fuckboi", "", false);
                } else if message.content == "!notping" {
                    let _ = discord.send_message(message.channel_id, "Amma shuttin' down naow!", "", false);
                    println!("Quiting");
                    break
                }
            },
            Ok(_) => {},
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, body);
				break
			},
			Err(err) => println!("Receive error: {:?}", err),
        }
    }
}