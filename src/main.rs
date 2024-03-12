extern crate netxt;
use std::{error, result};

use netxt::{today, Day, Todo};

use teloxide::prelude::*;
use teloxide::types::{
    MediaKind::Text,
    MessageKind::Common,
    Recipient::Id,
    UpdateKind::{EditedMessage, Message},
};

static TODO_FILE: &str = "todo.txt";
static CHAT_ID: ChatId = ChatId(26289585); // only work with a single user

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn error::Error>::from(format!($($tt)*))) };
}

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
    // TODO: button to copy day

    let mut today_present = false;
    if todo.today.sections.len() != 0 {
        today_present = true;
    }

    let updates = bot.get_updates().await?;
    let mut last_update_id = -1;

    for u in updates {
        if let Ok(day) = is_day_msg(&u) {
            if day.date == today() {
                todo.today = day;
                today_present = true;
            } else if !todo.days.contains(&day) && day.date < today() {
                todo.days.push(day);
            } else {
                return err!("wrong date");
            }
        }
        last_update_id = u.id;
    }

    if last_update_id != -1 {
        // ack update
        bot.get_updates()
            .offset(last_update_id + 1)
            .send()
            .await
            .unwrap();
    }

    if !today_present {
        todo.next_day();
        print_day_msg(bot, &todo.today).await?;
    }
    todo.save()?;

    Ok(())
}
fn is_day_msg(update: &Update) -> Result<Day> {
    match &update.kind {
        // use last update
        Message(msg) | EditedMessage(msg) => {
            if let Common(msg_common) = &msg.kind {
                if let Text(txt) = &msg_common.media_kind {
                    println!("{}", txt.text);
                    let day: Day = txt.text.parse()?;
                    return Ok(day);
                }
            }
            err!("Update is not Message or EditedMessage")
        }
        _ => {
            err!("nothing to update today")
        }
    }
}

async fn print_day_msg(bot: Bot, day: &Day) -> Result<()> {
    let _msg = bot
        .send_message(Id(CHAT_ID), format!("{}", day).to_string())
        .await?;
    Ok(())
}
