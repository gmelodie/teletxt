extern crate netxt;
use std::{
    error, fmt,
    io::{stdout, Write},
    result,
};

use netxt::{today, Day, Todo};

use teloxide::payloads::GetUpdatesSetters;
use teloxide::prelude::*;
use teloxide::types::{
    AllowedUpdate,
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

#[derive(Debug)]
enum UpdateError {
    NoUpdates,
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateError::NoUpdates => write!(f, "No new updates"),
        }
    }
}

impl error::Error for UpdateError {
    fn description(&self) -> &str {
        match *self {
            UpdateError::NoUpdates => "No new updates",
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting netxt bot...");

    let bot = Bot::from_env();
    let mut todo = Todo::new(Some(TODO_FILE))?;

    // TODO: command to force save a day (use update_todo)
    // TODO: maybe make this into a systemd thing
    // TODO: button to copy day

    update_todo(&bot, &mut todo).await?; // TODO: use NoUpdates error

    if todo.days.len() != 0 {
        let last_day = todo.days.iter().max_by_key(|d: &&Day| d.date).unwrap();
        if last_day.date < today() {
            todo.next_day();
            print_day_msg(bot, &todo.today).await?;
        }
    } else {
        todo.next_day();
        print_day_msg(bot, &todo.today).await?;
    }

    todo.save()?;
    Ok(())
}

async fn update_todo<'a>(bot: &Bot, todo: &mut Todo<'a>) -> Result<()> {
    let updates = bot
        .get_updates()
        .allowed_updates([AllowedUpdate::Message, AllowedUpdate::EditedMessage])
        .await?;
    if updates.len() == 0 {
        return Err(From::from(UpdateError::NoUpdates));
    }
    let mut last_update_id = -1;

    for u in updates {
        last_update_id = u.id;
        if let Ok(day) = is_day_msg(&u) {
            println!("{day}");
            if day.date == today() {
                todo.today = day;
            } else if !todo.days.contains(&day) && day.date < today() {
                todo.days.push(day);
            } else {
                // ack last processed update
                bot.get_updates()
                    .offset(last_update_id + 1)
                    .send()
                    .await
                    .unwrap();
                return err!("Unable to update a previous day that is already registered");
            }
        }
    }

    if last_update_id != -1 {
        // ack last processed update
        bot.get_updates()
            .offset(last_update_id + 1)
            .send()
            .await
            .unwrap();
    }

    Ok(())
}

fn is_day_msg(update: &Update) -> Result<Day> {
    match &update.kind {
        // use last update
        Message(msg) | EditedMessage(msg) => {
            if let Common(msg_common) = &msg.kind {
                if let Text(txt) = &msg_common.media_kind {
                    let day: Day = txt.text.parse()?;
                    return Ok(day);
                }
            }
            err!("Update is not Message or EditedMessage")
        }
        _ => {
            err!("Nothing to update today")
        }
    }
}

async fn print_day_msg(bot: Bot, day: &Day) -> Result<()> {
    let _msg = bot
        .send_message(Id(CHAT_ID), format!("{}", day).to_string())
        .await?;
    Ok(())
}
