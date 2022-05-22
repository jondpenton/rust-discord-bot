use discord::{model::Event, Discord};
use std::env;

fn main() {
  let discord = Discord::from_bot_token(
    &env::var("DISCORD_TOKEN").expect("Expected environment variable DISCORD_TOKEN."),
  )
  .expect("Login failed.");

  let (mut connection, _) = discord.connect().expect("Websocket connection failed.");

  println!("Connected.");

  loop {
    match connection.recv_event() {
      Ok(Event::MessageCreate(message)) if message.content == "ping" => {
        let _ = discord.send_message_ex(message.channel_id, |send_message| {
          send_message.reply(message.id, true).content("pong")
        });
      }
      Ok(_) => {}
      Err(err) => println!("Error received: {:?}", err),
    }
  }
}
