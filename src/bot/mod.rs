// public modules
pub mod commands;
pub mod configurations;

// private modules
mod callback_handlers;
mod dialogue;
mod inline_keyboards;
mod message_processor;

use callback_handlers::{default_callback_handler, receive_cocktail_name_callback_handler};
use commands::MainCommands;
use dialogue::State;
use message_processor::{
    GetCocktailsFilterByNameListCommand, GetMainMenuCommand, MessageProcessor,
};
use std::{error::Error, sync::OnceLock};
use teloxide::{
    adaptors::DefaultParseMode,
    dispatching::{
        dialogue::{GetChatId, InMemStorage},
        MessageFilterExt, UpdateFilterExt, UpdateHandler,
    },
    dptree::{self, case},
    prelude::{Dialogue, Dispatcher, LoggingErrorHandler, Requester, RequesterExt},
    types::{Message, ParseMode, Update},
    utils::markdown::escape,
    Bot as TBot,
};

use crate::{bot::configurations::BotConfig, shared::CommandHandler};

type BotDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

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
        Dispatcher::builder(bot_instance, self.schema())
            .dependencies(dptree::deps![InMemStorage::<State>::new()])
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

    fn schema(&self) -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
        let main_commands_handler = teloxide::filter_command::<MainCommands, _>()
            .branch(case![MainCommands::Menu].endpoint(main_commands_menu_handler));

        let text_handler = Message::filter_text().branch(
            case![State::ReceiveCocktailName]
                .endpoint(filter_cocktails_by_name_dialogue_receive_cocktail_name),
        );
        let message_handler = Update::filter_message()
            .branch(main_commands_handler)
            .branch(text_handler);

        let callback_query_handler = Update::filter_callback_query()
            .branch(case![State::Start].endpoint(default_callback_handler))
            .branch(case![State::ReceiveCocktailName].endpoint(default_callback_handler))
            .branch(
                case![State::ReveivedCocktailName { cocktail_name }]
                    .endpoint(receive_cocktail_name_callback_handler),
            );

        teloxide::dispatching::dialogue::enter::<Update, InMemStorage<State>, State, _>()
            .branch(message_handler)
            .branch(callback_query_handler)
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if .
async fn main_commands_menu_handler(
    msg: Message,
    _bot: Bot,
    _dialogue: BotDialogue,
) -> HandlerResult {
    let processor = MessageProcessor::new().await?;
    let user_id = msg
        .from
        .clone()
        .expect("Can't get user info from telegram message")
        .id;
    let chat_id = msg
        .chat_id()
        .expect("Can't get chat id from telegram message");
    processor
        .handle(GetMainMenuCommand {
            user_id,
            chat_id,
            message_id: msg.id,
            edit_message: false,
        })
        .await?;
    Ok(())
}

async fn filter_cocktails_by_name_dialogue_receive_cocktail_name(
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            let message_proc = MessageProcessor::new().await?;

            message_proc
                .handle(GetCocktailsFilterByNameListCommand {
                    chat_id: msg.chat_id().unwrap(),
                    message_id: None,
                    cocktail_name_for_filter: text.to_string(),
                    next_page: 0,
                })
                .await?;
            dialogue
                .update(State::ReveivedCocktailName {
                    cocktail_name: text.to_string(),
                })
                .await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                escape("Отправь мне название коктейля или его часть."),
            )
            .await?;
        }
    }
    Ok(())
}
