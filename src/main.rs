extern crate netxt;
use std::{env, error, fmt, fs, path::Path, result};

use teloxide::payloads::GetUpdatesSetters;
use teloxide::prelude::*;
use teloxide::types::{
    AllowedUpdate, MediaKind::Text, Message, MessageKind::Common, UpdateKind, User,
};

use netxt::{Day, Todo};

static TODO_DIR: &str = "todos";
static ALLOWED_USERS_FILE: &str = "allowed-users.txt";

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

    // TODO: command to force save a day (use update_todo)
    // TODO: maybe make this into a systemd thing
    // TODO: button to copy day

    let updates = bot
        .get_updates()
        .allowed_updates([AllowedUpdate::Message, AllowedUpdate::EditedMessage])
        .await?;
    if updates.len() == 0 {
        return Err(From::from(UpdateError::NoUpdates));
    }

    let mut last_update_id = -1;

    for u in updates {
        log::debug!("{:#?}", u);
        if let Ok((username, day)) = is_valid_update(&u) {
            log::info!("Received update from user '{username}'");
            if let Err(err) = update_todo(&username, &day).await {
                // TODO: use NoUpdates error (add variant)
                log::error!("{err}, leaving update {} without ack", u.id);
                continue; // do not ack this update (leave it to retry since it seems to be a valid day if it got here)
            }
        }
        last_update_id = u.id;
    }

    if last_update_id != -1 {
        // ack last processed update
        bot.get_updates()
            .offset(last_update_id + 1)
            .send()
            .await
            .expect("Unable to ack last_update_id={last_update_id}");
    }

    Ok(())
}

async fn update_todo(username: &str, day: &Day) -> Result<()> {
    // load todo from file or create if doesnt exist
    // ./TODO_DIR/{username}.txt
    let todo_dir: &str = &env::var("TODO_DIR").unwrap_or(TODO_DIR.to_string());
    let file_path = Path::new(todo_dir)
        .join(format!("{username}.txt"))
        .display()
        .to_string();
    let mut todo = Todo::new(Some(&file_path))?;

    // if user is not allowed, ignore this update
    if !allowed_user(username) {
        return err!("Username not whitelisted");
    }

    // remove old day and put new one in place
    if todo.days.iter().any(|d| d.date == day.date) {
        let index = todo.days.iter().position(|x| x.date == day.date).unwrap(); // this should always be possible since we are sure it .contains(&day)
        todo.days.remove(index);
    }
    todo.days.push(day.clone());

    // if day in question is not today, create today (next_day does nothing in case today already exists)
    todo.next_day();

    todo.save()?;

    Ok(())
}

/// Validates and, if valid, parses a Day from an Update message
// TODO: maybe this could be a From trait implementation
fn is_valid_update(update: &Update) -> Result<(String, Day)> {
    // unwrap msg
    let msg = get_msg(update)?;

    // unwrap user
    let user = get_user(&msg)?;

    // unwrap day
    let day = is_day_msg(msg)?;

    // get username
    if let Some(username) = user.username {
        Ok((username, day))
    } else {
        err!("No username found in message")
    }
}

fn allowed_user(username: &str) -> bool {
    // open ALLOWED_USERS_FILE, panic if it doesn't exist
    let allowed_users_file: &str =
        &env::var("ALLOWED_USERS_FILE").unwrap_or(ALLOWED_USERS_FILE.to_string());
    match fs::read_to_string(allowed_users_file) {
        Ok(users_file) => {
            for line in users_file.lines() {
                if username == line.trim() {
                    return true;
                }
            }
        }
        Err(err) => {
            panic!(
                "Unable to read allowed users file {}: {err}",
                ALLOWED_USERS_FILE
            );
        }
    }
    false
}

fn get_msg(update: &Update) -> Result<&Message> {
    match &update.kind {
        UpdateKind::Message(msg) | UpdateKind::EditedMessage(msg) => Ok(msg),
        _ => {
            err!("Nothing to update today")
        }
    }
}

fn get_user(msg: &Message) -> Result<User> {
    match &msg.kind {
        Common(message_common) => {
            let user = message_common
                .from
                .clone()
                .expect("Unable to find user in message");
            Ok(user)
        }
        _ => {
            err!("Not a common type message")
        }
    }
}

fn is_day_msg(msg: &Message) -> Result<Day> {
    match &msg.kind {
        Common(msg_common) => {
            if let Text(txt) = &msg_common.media_kind {
                let day: Day = txt.text.parse()?;
                return Ok(day);
            }
            err!("Update is not Message or EditedMessage")
        }
        _ => {
            err!("Nothing to update today")
        }
    }
}

// async fn print_day_msg(bot: Bot, day: &Day) -> Result<()> {
//     let _msg = bot
//         .send_message(Id(CHAT_ID), format!("{}", day).to_string())
//         .await?;
//     Ok(())
// }
