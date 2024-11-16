use std::error::Error;

use teloxide::{
    adaptors::DefaultParseMode, dispatching::dialogue::GetChatId, prelude::Requester, types::{CallbackQuery, UserId}, utils::markdown::escape, Bot as TBot
};

use crate::{
    bot::{
        commands::MenuCommands,
        dialogue::State,
        message_processor::{
            AddCocktailToFavoriteCommand, GetCocktailPageByIdCommand, GetCocktailPagesCommand,
            GetCocktailsFilterByNameListCommand, GetCocktailsListCommand,
            GetFavoriteCocktailsListCommand, GetMainMenuCommand, GetProfilePageCommand,
            GetRegisterUserConfigrationCommand, GetRemoveUserConfirmationCommand, MessageProcessor,
            RegisterUserCommand, RemoveCocktailFromFavoriteCommand, RemoveUserCommand,
        },
    },
    shared::CommandHandler,
};

use super::BotDialogue;

type Bot = DefaultParseMode<TBot>;
type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn default_callback_handler(
    bot: Bot,
    dialogue: BotDialogue,
    callback: CallbackQuery,
) -> HandlerResult {
    if let Some(ref callback_btn) = callback.data {
        let user_id = callback.from.id;

        log::debug!("User {} press menu button: {}", user_id, callback_btn);
        let menu_cmd = MenuCommands::parse(callback_btn);
        match menu_cmd {
            MenuCommands::MainMenu => {
                process_main_menu(callback, user_id, dialogue).await?;
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
                let message_id = callback.clone().message.unwrap().id();
                let edit_message_text = bot.edit_message_text(
                    callback.chat_id().unwrap(),
                    message_id,
                    escape("Напишите мне полное название коктейля или его часть."),
                );
                edit_message_text.await?;

                dialogue.update(State::ReceiveCocktailName).await?;
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
                process_search_by_id(callback, cocktail_id, prev_page, page_num).await?;
            }
            MenuCommands::CocktailsPages(total_pages, prev_page) => {
                process_cocktails_pages(callback, total_pages, prev_page).await?;
            }
            MenuCommands::AddToFavorite(coctail_id, prev_page, page_num) => {
                process_add_to_favorite(callback, coctail_id, prev_page, page_num).await?;
            }
            MenuCommands::RemoveFromFavorite(coctail_id, prev_page, page_num) => {
                process_remove_from_favorite(callback, coctail_id, prev_page, page_num).await?;
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
            MenuCommands::CocktailsListByName(_page) => todo!(),
        };
    }

    Ok(())
}

pub async fn receive_cocktail_name_callback_handler(
    _bot: Bot,
    dialogue: BotDialogue,
    callback: CallbackQuery,
    cocktail_name: String,
) -> HandlerResult {
    if let Some(ref callback_btn) = callback.data {
        let user_id = callback.from.id;

        log::debug!("User {} press menu button: {}", user_id, callback_btn);
        let menu_cmd = MenuCommands::parse(callback_btn);
        match menu_cmd {
            MenuCommands::MainMenu => {
                process_main_menu(callback, user_id, dialogue).await?;
            }
            MenuCommands::CocktailsListByName(page) => {
                let message_proc = MessageProcessor::new().await?;
                let message_id = callback.clone().message.unwrap().id();
                message_proc
                    .handle(GetCocktailsFilterByNameListCommand {
                        chat_id: callback.chat_id().unwrap(),
                        message_id: Some(message_id),
                        cocktail_name_for_filter: cocktail_name,
                        next_page: page,
                    })
                    .await?;
            }
            MenuCommands::CocktailsPages(total_pages, prev_page) => {
                process_cocktails_pages(callback, total_pages, prev_page).await?;
            }
            MenuCommands::SearchById(cocktail_id, prev_page, page_num) => {
                process_search_by_id(callback, cocktail_id, prev_page, page_num).await?;
            }
            MenuCommands::AddToFavorite(coctail_id, prev_page, page_num) => {
                process_add_to_favorite(callback, coctail_id, prev_page, page_num).await?;
            }
            MenuCommands::RemoveFromFavorite(coctail_id, prev_page, page_num) => {
                process_remove_from_favorite(callback, coctail_id, prev_page, page_num).await?;
            }
            _ => todo!(),
        };
    };
    Ok(())
}

async fn process_main_menu(
    callback: CallbackQuery,
    user_id: UserId,
    dialogue: BotDialogue,
) -> HandlerResult {
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
    dialogue.exit().await?;
    Ok(())
}

async fn process_cocktails_pages(
    callback: CallbackQuery,
    total_pages: u64,
    prev_page: String,
) -> HandlerResult {
    let message_proc = MessageProcessor::new().await?;
    message_proc
        .handle(GetCocktailPagesCommand {
            callback: callback.clone(),
            prev_page: MenuCommands::parse(&prev_page),
            total_pages,
        })
        .await?;
    Ok(())
}

async fn process_search_by_id(
    callback: CallbackQuery,
    cocktail_id: String,
    prev_page: String,
    page_num: Option<u64>,
) -> HandlerResult {
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
    Ok(())
}

async fn process_add_to_favorite(
    callback: CallbackQuery,
    cocktail_id: String,
    prev_page: String,
    page_num: Option<u64>,
) -> HandlerResult {
    let message_proc = MessageProcessor::new().await?;
    message_proc
        .handle(AddCocktailToFavoriteCommand {
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
    Ok(())
}

async fn process_remove_from_favorite(
    callback: CallbackQuery,
    cocktail_id: String,
    prev_page: String,
    page_num: Option<u64>,
) -> HandlerResult {
    let message_proc = MessageProcessor::new().await?;
    message_proc
        .handle(RemoveCocktailFromFavoriteCommand {
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
    Ok(())
}
