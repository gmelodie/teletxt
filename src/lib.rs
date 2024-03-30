extern crate netxt;
use crate::util::{allowed_user, file_path_from_username, get_user};
use netxt::{Day, Todo};
use std::{error, result};
use teloxide::types::{MediaKind::Text, Message, MessageKind::Common};

pub mod util;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn error::Error + Send + Sync>::from(format!($($tt)*))) };
}

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
    let file_path = file_path_from_username(username);
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
