use std::{error, result};

use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    net::Download,
    prelude::*,
    types::{Document, InputFile},
    utils::command::BotCommands,
};
use tokio::fs::{read_to_string, File};

use teletxt::{
    is_valid_msg, update_todo,
    util::{allowed_user, file_path_from_username, get_user},
};

use netxt::Todo;

type Result<T> = result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;

type MyDialogue = Dialogue<State, InMemStorage<State>>;

// TODO: do not have macro and the result type redefined here and in lib.rs
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn error::Error + Send + Sync + 'static>::from(format!($($tt)*))) };
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveNewMessage,
    ReceiveEditedMessage,
}

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display this text.
    Help,
    /// Lets user know that bot is running (doesn't actually start bot)
    Start,
    /// Downloads todo file for current user from bot
    Download,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting netxt bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    // TODO: command to force save a day (use update_todo)
    // TODO: button to copy day

    Ok(())
}
fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Start].endpoint(start))
        .branch(case![Command::Download].endpoint(download));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(receive_message));

    let edited_message_handler =
        Update::filter_edited_message().branch(dptree::endpoint(receive_message));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(edited_message_handler)
}

async fn help(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn start(bot: Bot, msg: Message) -> Result<()> {
    let txt = "[2024-12-31]
- this is a task with no section

Interview
- interview candidate A
- write review on candidate B

Groceries
- tomatoes
- lettuce

Done
- this is a task that is done
";
    bot.send_message(
        msg.chat.id,
        format!("Bot is running!\nHere's an example of a day:\n\n{txt}").to_string(),
    )
    .await?;
    Ok(())
}

async fn download(bot: Bot, msg: Message) -> Result<()> {
    let user = get_user(&msg)?;
    match user.username {
        None => {
            bot.send_message(
                msg.chat.id,
                "Username not found in telegram message".to_string(),
            )
            .await?;
        }
        Some(username) => {
            // if user is in allow list and file for this user exists, send it
            if !allowed_user(&username) {
                bot.send_message(msg.chat.id, "Username not allowed".to_string())
                    .await?;
                return err!("Username not allowed");
            }

            let file_path = file_path_from_username(&username);
            let file = InputFile::file(file_path);
            // Send file to user
            bot.send_document(msg.chat.id, file).await?;
        }
    }
    Ok(())
}

async fn upload(bot: Bot, msg: &Message, document: &Document) -> Result<()> {
    let user = get_user(msg)?;
    match user.username {
        None => {
            bot.send_message(
                msg.chat.id,
                "Username not found in telegram message".to_string(),
            )
            .await?;
        }
        Some(username) => {
            // if user is in allow list and file for this user exists, send it
            if !allowed_user(&username) {
                bot.send_message(msg.chat.id, "Username not allowed".to_string())
                    .await?;
                return err!("Username not allowed");
            }

            // get file from user
            let file_id = document.file.id.clone();
            let file = bot.get_file(file_id).await?;
            let mut dst = File::create(format!("/tmp/{username}.txt")).await?;
            bot.download_file(&file.path, &mut dst).await?;

            // check if file is valid todo
            let mut todo: Todo = read_to_string(format!("/tmp/{username}.txt"))
                .await
                .unwrap()
                .parse()?;

            // set new path to todo
            todo.file_path = file_path_from_username(&username).into();
            todo.save()?;

            log::info!(
                "File {} saved for user {}",
                file_path_from_username(&username),
                username
            );

            // let user know that everything is ok
            bot.send_message(
                msg.chat.id,
                format!("File saved for user {username}").to_string(),
            )
            .await?;
        }
    }
    Ok(())
}

async fn receive_message(bot: Bot, _dialogue: MyDialogue, msg: Message) -> Result<()> {
    log::info!("Got new message ID: {}", msg.id);
    log::debug!("{:#?}", msg);

    // if it is an upload message
    if let Some(document) = msg.document() {
        upload(bot, &msg, document).await?;
        return Ok(());
    }

    if let Ok((username, day)) = is_valid_msg(&msg) {
        if let Err(err) = update_todo(&username, &day).await {
            // TODO: better error handling, only send to user when there's no other way to fix
            bot.send_message(
                msg.chat.id,
                format!("Could not update TODO: {err}").to_string(),
            )
            .await?;
            return err!("Could not update TODO for message {}: {err}", msg.id);
        }
    }

    Ok(())
}
