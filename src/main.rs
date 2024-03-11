extern crate netxt;
use std::{error, result};
use tokio::time::{sleep, Duration};

use netxt::{Day, Todo};

use teloxide::prelude::*;
use teloxide::types::{
    MediaKind::Text,
    MessageKind::Common,
    Recipient::Id,
    UpdateKind::{EditedMessage, Message},
};

static TODO_FILE: &str = "todo.txt";
static CHAT_ID: ChatId = ChatId(26289585); // only work with a single user

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting netxt bot...");

    let bot = Bot::from_env();

    let mut todo = Todo::new(Some(TODO_FILE))?;

    // TODO: command to force save a day
    // TODO: /stop command
    // TODO: /start command
    // TODO: maybe make this into a systemd thing

    loop {
        // print new today message
        let msg = bot
            .send_message(Id(CHAT_ID), format!("{}", todo.today).to_string())
            .await?;
        // TODO: button to copy day

        sleep(Duration::from_secs(15)).await;
        // TODO: sleep 1 day
        // sleep(Duration::from_secs(86400)).await;

        let updates = bot.get_updates().await?;
        if updates.len() != 0 {
            match &updates[updates.len() - 1].kind {
                // use last update
                Message(msg) | EditedMessage(msg) => {
                    if let Common(msg_common) = &msg.kind {
                        if let Text(txt) = &msg_common.media_kind {
                            println!("{}", txt.text);
                            let last_day: Day = txt.text.parse()?;
                            // ignore if date is incorrect (maybe from past update)
                            if last_day.date == todo.today.date {
                                todo.today = last_day;
                            }
                        }
                    }
                }
                _ => {
                    println!("nothing to update today");
                }
            }
        }

        // save today and create new day
        // create new today
        // TODO: next day should have all undone tasks of current day
        todo.next_day();
    }
    // Ok(())
}
