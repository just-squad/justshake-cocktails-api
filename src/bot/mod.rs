// public modules
pub mod commands;
pub mod configurations;
pub mod message_processor;

// private modules
mod inline_keyboards;

use commands::{MainCommands, MenuCommands};
use message_processor::{
    AddCocktailToFavoriteCommand, GetCocktailPageByIdCommand, GetCocktailPagesCommand,
    GetCocktailsListCommand, GetFavoriteCocktailsListCommand, GetMainMenuCommand,
    GetProfilePageCommand, GetRegisterUserConfigrationCommand, GetRemoveUserConfirmationCommand,
    MessageProcessor, RegisterUserCommand, RemoveCocktailFromFavoriteCommand, RemoveUserCommand,
};
use std::{error::Error, sync::OnceLock};
use teloxide::{
    adaptors::DefaultParseMode,
    dispatching::{dialogue::GetChatId, HandlerExt, UpdateFilterExt},
    dptree,
    prelude::{Dispatcher, LoggingErrorHandler, Requester, RequesterExt},
    types::{
        CallbackQuery, InlineQuery, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Message, ParseMode, Update,
    },
    Bot as TBot,
};

use crate::{bot::configurations::BotConfig, shared::CommandHandler};

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
        let handler = dptree::entry()
            .branch(Update::filter_callback_query().endpoint(callback_handler))
            .branch(Update::filter_inline_query().endpoint(inline_query_handler))
            .branch(
                Update::filter_message()
                    .filter_command::<MainCommands>()
                    .endpoint(main_commands_handler),
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
        }
    };
    Ok(())
}

async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let choose_debian_version = InlineQueryResultArticle::new(
        "0",
        "Chose debian version",
        InputMessageContent::Text(InputMessageContentText::new("Debian versions:")),
    )
    .reply_markup(inline_keyboards::get_main_menu_keyboard(&true));

    bot.answer_inline_query(q.id, vec![choose_debian_version.into()])
        .await?;

    Ok(())
}

async fn callback_handler(
    _me: teloxide::types::Me,
    _update: Update,
    callback: CallbackQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(ref callback_btn) = callback.data {
        let user_id = callback.from.id;

        log::debug!("User {} press menu button: {}", user_id, callback_btn);
        let menu_cmd = MenuCommands::parse(callback_btn);
        match menu_cmd {
            MenuCommands::MainMenu => {
                let message_proc = MessageProcessor::new().await?;
                let message_id = callback.clone().message.unwrap().id();
                message_proc
                    .handle(GetMainMenuCommand {
                        user_id,
                        chat_id: callback.chat_id().unwrap(),
                        message_id,
                        edit_message: true,
                    })
                    .await?;
            }
            MenuCommands::CocktailsList(page) => {
                let message_proc = MessageProcessor::new().await?;
                message_proc
                    .handle(GetCocktailsListCommand {
                        callback: callback.clone(),
                        next_page: page,
                    })
                    .await?;
            }
            MenuCommands::SearchByName => {
                let message_proc = MessageProcessor::new().await?;
                message_proc
                    .send_cocktails_paged_filter_by_name(&user_id, &callback.chat_id().unwrap())
                    .await?;
            }
            MenuCommands::Register => {
                let message_proc = MessageProcessor::new().await?;
                message_proc
                    .handle(RegisterUserCommand {
                        callback: callback.clone(),
                    })
                    .await?;
            }
            MenuCommands::ProfilePage => {
                let message_proc = MessageProcessor::new().await?;
                let message_id = callback.clone().message.unwrap().id();
                message_proc
                    .handle(GetProfilePageCommand {
                        chat_id: callback.chat_id().unwrap(),
                        message_id,
                    })
                    .await?;
            }
            MenuCommands::SearchById(cocktail_id, prev_page, page_num) => {
                let message_proc = MessageProcessor::new().await?;
                message_proc
                    .handle(GetCocktailPageByIdCommand {
                        callback: callback.clone(),
                        prev_page: MenuCommands::parse(
                            format!(
                                "{} {}",
                                &prev_page,
                                if let Some(page_num) = page_num {
                                    page_num.to_string()
                                } else {
                                    "".to_string()
                                }
                            )
                            .as_str(),
                        ),
                        cocktail_id: uuid::Uuid::parse_str(cocktail_id.as_str()).unwrap(),
                    })
                    .await?;
            }
            MenuCommands::CocktailsPages(total_pages, prev_page) => {
                let message_proc = MessageProcessor::new().await?;
                message_proc
                    .handle(GetCocktailPagesCommand {
                        callback: callback.clone(),
                        prev_page: MenuCommands::parse(&prev_page),
                        total_pages,
                    })
                    .await?;
            }
            MenuCommands::AddToFavorite(coctail_id, prev_page) => {
                let message_proc = MessageProcessor::new().await?;
                message_proc
                    .handle(AddCocktailToFavoriteCommand {
                        callback: callback.clone(),
                        prev_page: MenuCommands::parse(&prev_page),
                        cocktail_id: uuid::Uuid::parse_str(coctail_id.as_str()).unwrap(),
                    })
                    .await?;
            }
            MenuCommands::RemoveFromFavorite(coctail_id, prev_page) => {
                let message_proc = MessageProcessor::new().await?;
                message_proc
                    .handle(RemoveCocktailFromFavoriteCommand {
                        callback: callback.clone(),
                        prev_page: MenuCommands::parse(&prev_page),
                        cocktail_id: uuid::Uuid::parse_str(coctail_id.as_str()).unwrap(),
                    })
                    .await?;
            }
            MenuCommands::RegisterConfirmation => {
                let message_proc = MessageProcessor::new().await?;
                let message_id = callback.clone().message.unwrap().id();
                message_proc
                    .handle(GetRegisterUserConfigrationCommand {
                        chat_id: callback.chat_id().unwrap(),
                        message_id,
                    })
                    .await?;
            }
            MenuCommands::RemoveAccount => {
                let message_proc = MessageProcessor::new().await?;
                message_proc
                    .handle(RemoveUserCommand {
                        callback: callback.clone(),
                    })
                    .await?;
            }
            MenuCommands::RemoveAccountConfirmation => {
                let message_proc = MessageProcessor::new().await?;
                let message_id = callback.clone().message.unwrap().id();
                message_proc
                    .handle(GetRemoveUserConfirmationCommand {
                        chat_id: callback.chat_id().unwrap(),
                        message_id,
                    })
                    .await?;
            }
            MenuCommands::ShowFavorites(page) => {
                let message_proc = MessageProcessor::new().await?;
                message_proc
                    .handle(GetFavoriteCocktailsListCommand {
                        callback: callback.clone(),
                        next_page: page,
                    })
                    .await?;
            }
            MenuCommands::Unknown => todo!(),
        };
    }

    Ok(())
}
