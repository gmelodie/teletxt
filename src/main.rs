extern crate netxt;
use std::{error, result};
use tokio::time::{sleep, Duration};

use netxt::Todo;

use teloxide::prelude::*;
use teloxide::types::Recipient::Id;

static TODO_FILE: &str = "todo.txt";
static CHAT_ID: ChatId = ChatId(26289585); // only work with a single user

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting netxt bot...");

    let bot = Bot::from_env();

    // new todo
    let mut _todo = Todo::new(Some(TODO_FILE));

    loop {
        // print new today message
        let _ = bot
            .send_message(Id(CHAT_ID), format!("Today Message goes here").to_string())
            .await?;

        sleep(Duration::from_secs(5)).await;
        // sleep 1 day
        // sleep(Duration::from_secs(86400)).await;
        // parse today message
        // save today as yesterday
        break;
    }
    Ok(())
}
