use std::{env, error::Error};

use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, update_listeners::webhooks, types::Me, RequestError};

type HandlerResult = Result<(), RequestError>;
type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Heroku ping-pong bot...");

    let bot = Bot::from_env();

    // Heroku auto defines a port value
    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");

    let addr = ([0, 0, 0, 0], port).into();

    // Heroku host example: "heroku-ping-pong-bot.herokuapp.com"
    let host = env::var("HOST").expect("HOST env variable is not set");
    let url = format!("https://{host}/webhook").parse().unwrap();

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(bot, handler, listener).await;
}

async fn handler(bot: Bot, msg: Message, me: Me) -> HandlerResult {
    println!("{:?}", me);
    bot.send_message(msg.chat.id, "It works").await?;
    Ok(())
}
