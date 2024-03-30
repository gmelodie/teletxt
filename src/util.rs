use std::{env, error, fs, path::PathBuf, result};
use teloxide::types::{Message, MessageKind::Common, User};

static TODO_DIR: &str = "todos";
static ALLOWED_USERS_FILE: &str = "allowed-users.txt";

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn error::Error + Send + Sync>::from(format!($($tt)*))) };
}

pub type Result<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

/// Loads todo from file or create if doesnt exist
/// ./TODO_DIR/{username}.txt
pub fn file_path_from_username(username: &str) -> String {
    let todo_dir: &str = &env::var("TODO_DIR").unwrap_or(TODO_DIR.to_string());
    PathBuf::new()
        .join(todo_dir)
        .join(format!("{username}.txt"))
        .display()
        .to_string()
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

/// Gets user struct given a message
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
