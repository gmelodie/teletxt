use std::{error, result};

use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
};

use teletxt::{is_valid_msg, update_todo};

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
    // use dptree::case;

    // let command_handler = teloxide::filter_command::<Command, _>()
    //     .branch(
    //         case![State::Start]
    //             .branch(case![Command::Help].endpoint(help))
    //             .branch(case![Command::Start].endpoint(start)),
    //     )
    //     .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message().branch(dptree::endpoint(receive_message));
    let edited_message_handler =
        Update::filter_edited_message().branch(dptree::endpoint(receive_message));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(edited_message_handler)
    // .branch(command_handler)
}

async fn receive_message(_bot: Bot, _dialogue: MyDialogue, msg: Message) -> Result<()> {
    log::info!("Got new message ID: {}", msg.id);
    log::debug!("{:#?}", msg);

    if let Ok((username, day)) = is_valid_msg(&msg) {
        if let Err(err) = update_todo(&username, &day).await {
            return err!("Could not update TODO for message {}: {err}", msg.id);
        }
    }

    Ok(())
}
