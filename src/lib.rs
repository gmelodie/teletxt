extern crate netxt;
use netxt::{Day, Todo};
use std::{env, error, fs, path::PathBuf, result};
use teloxide::types::{MediaKind::Text, Message, MessageKind::Common, User};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn error::Error + Send + Sync>::from(format!($($tt)*))) };
}

pub static TODO_DIR: &str = "todos";
pub static ALLOWED_USERS_FILE: &str = "allowed-users.txt";

type Result<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

pub async fn update_todo(username: &str, day: &Day) -> Result<()> {
    let mut todo = get_todo(username)?;

    // if user is not allowed, ignore this update
    if !allowed_user(username) {
        return err!("Username not allowed");
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

/// Gets the todo for a user given a username
pub fn get_todo(username: &str) -> Result<Todo> {
    // load todo from file or create if doesnt exist
    // ./TODO_DIR/{username}.txt
    let todo_dir: &str = &env::var("TODO_DIR").unwrap_or(TODO_DIR.to_string());
    let file_path = PathBuf::new()
        .join(todo_dir)
        .join(format!("{username}.txt"))
        .display()
        .to_string();
    let todo = Todo::new(Some(&file_path))?;
    Ok(todo)
}

/// Validates and, if valid, parses a Day from an Update message
// TODO: maybe this could be a From trait implementation
pub fn is_valid_msg(msg: &Message) -> Result<(String, Day)> {
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

pub fn allowed_user(username: &str) -> bool {
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

pub fn get_user(msg: &Message) -> Result<User> {
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
                if let Ok(day) = txt.text.parse() {
                    return Ok(day);
                } else {
                    return err!("Could not parse Day");
                }
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
