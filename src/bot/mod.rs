// public modules
pub mod commands;
pub mod configurations;
pub mod message_processor;

// private modules
mod inline_keyboards;

use commands::MainCommands;
use message_processor::MessageProcessor;
use std::{error::Error, sync::OnceLock};
use teloxide::{
    adaptors::DefaultParseMode,
    dispatching::{dialogue::GetChatId, HandlerExt, UpdateFilterExt},
    dptree,
    prelude::{Dispatcher, LoggingErrorHandler, RequesterExt},
    types::{Message, ParseMode, Update},
    Bot as TBot,
};

use crate::bot::configurations::BotConfig;

pub type Bot = DefaultParseMode<TBot>;
pub static INSTANCE: OnceLock<TgBotProvider> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct TgBotProvider {
    bot: Bot,
}

impl TgBotProvider {
    pub fn new(config: &BotConfig) -> Self {
        TgBotProvider {
            bot: TBot::new(&config.bot_token).parse_mode(ParseMode::MarkdownV2),
        }
    }

    pub fn global() -> &'static TgBotProvider {
        INSTANCE
            .get()
            .expect("Can't get TgBotProvider from global instance")
    }
}

impl TgBotProvider {
    pub async fn start_receive_messages(&self) {
        let bot_instance = self.bot.clone();
        let handler = dptree::entry().branch(
            Update::filter_message().branch(
                dptree::entry()
                    .filter_command::<MainCommands>()
                    .endpoint(main_commands_handler),
            ),
        );

        Dispatcher::builder(bot_instance, handler)
            .default_handler(|upd| async move {
                log::warn!("Unhandled update: {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if .
async fn main_commands_handler(
    msg: Message,
    _bot: Bot,
    cmd: MainCommands,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cmd {
        MainCommands::Menu => {
            let processor = MessageProcessor::new().await;
            let user_id = msg
                .from
                .clone()
                .expect("Can't get user info from telegram message")
                .id;
            let chat_id = msg
                .chat_id()
                .expect("Can't get chat id from telegram message");
            processor.send_menu_to_user(&user_id, &chat_id).await?;
        }
    };
    Ok(())
}
