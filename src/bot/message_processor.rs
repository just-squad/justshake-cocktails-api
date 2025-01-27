use anyhow::{Context, Result};
use async_trait::async_trait;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::utils::markdown::escape;
use uuid::Uuid;

use super::commands::MenuCommands;
use super::inline_keyboards::{self, ListCocktailsSource};
use crate::bot::inline_keyboards::PageNumber;
use crate::domain::aggregates::cocktail::{CocktailFilter, CocktailsPaged};
use crate::domain::aggregates::user::User;
use crate::shared::CommandHandler;
use crate::{
    bot::TgBotProvider,
    domain::{
        aggregates::{
            cocktail::CocktailRepo,
            user::UserRepo,
        },
        Pagination,
    },
    infrastructure,
};
use teloxide::payloads::{AnswerCallbackQuerySetters, EditMessageTextSetters};
use teloxide::types::{CallbackQuery, MessageId};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatId, UserId},
};

#[derive(Debug, Clone)]
pub struct MessageProcessor<TUserRepo, TCocktailRepo> {
    bot_provider: TgBotProvider,
    user_repo: TUserRepo,
    cocktail_repo: TCocktailRepo,
}

impl MessageProcessor<(), ()> {
    /// .
    pub async fn new() -> Result<MessageProcessor<impl UserRepo, impl CocktailRepo>> {
        let bt_prvdr = TgBotProvider::global().clone();
        let repository_factory = infrastructure::RepositoryFactory::global().clone();
        let user_repository = repository_factory
            .get_user_repository()
            .await
            .context("failed to create user repo in message processor")?;
        let cocktail_repository = repository_factory
            .get_cocktails_repository()
            .await
            .context("failed to create cocktail repo in message processor")?;

        Ok(MessageProcessor {
            bot_provider: bt_prvdr,
            user_repo: user_repository,
            cocktail_repo: cocktail_repository,
        })
    }
}

pub struct GetMainMenuCommand {
    pub user_id: UserId,
    pub chat_id: ChatId,
    pub message_id: MessageId,
    pub edit_message: bool,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<GetMainMenuCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: GetMainMenuCommand) -> Result<()> {
        let user_registered = self
            .user_repo
            .is_exist_by_telegram_id(&command.user_id.0)
            .await?;
        let keyboard = inline_keyboards::get_main_menu_keyboard(&user_registered);

        if command.edit_message {
            let mut edit_message_text = self.bot_provider.bot.edit_message_text(
                command.chat_id,
                command.message_id,
                "Основное меню: ",
            );
            edit_message_text = edit_message_text.reply_markup(keyboard.clone());
            edit_message_text.await?;
        } else {
            self.bot_provider
                .bot
                .send_message(command.chat_id, "Основное меню:")
                .reply_markup(keyboard)
                .await?;
        }
        Ok(())
    }
}

pub struct GetCocktailsListCommand {
    pub callback: CallbackQuery,
    pub next_page: u64,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<GetCocktailsListCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: GetCocktailsListCommand) -> Result<()> {
        let page_size: u64 = 10;
        let cocktails_filter = CocktailFilter {
            ids: Some(vec![]),
            names: None,
            russian_names: None,
            pagination: Pagination {
                page: command.next_page,
                items_per_page: page_size,
            },
        };
        let _cocktails_names = self.cocktail_repo.get_names(&cocktails_filter).await?;
        let keyboard = inline_keyboards::get_cocktails_list_keyboard(
            &_cocktails_names,
            &PageNumber(command.next_page),
            &page_size,
            ListCocktailsSource::CocktailList,
        );
        let callback_cloned = command.callback.clone();
        let chat_id = callback_cloned.chat_id().unwrap();
        let message_id = callback_cloned.message.unwrap().id();
        let mut edit_message_text =
            self.bot_provider
                .bot
                .edit_message_text(chat_id, message_id, "Коктейли: ");
        edit_message_text = edit_message_text.reply_markup(keyboard.clone());
        edit_message_text.await?;

        Ok(())
    }
}

pub struct GetCocktailsFilterByNameListCommand {
    pub chat_id: ChatId,
    pub message_id: Option<MessageId>,
    pub cocktail_name_for_filter: String,
    pub next_page: u64,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<GetCocktailsFilterByNameListCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: GetCocktailsFilterByNameListCommand) -> Result<()> {
        let page_size: u64 = 10;
        let cocktails_filter = CocktailFilter {
            ids: None,
            names: Some(vec![command.cocktail_name_for_filter.clone()]),
            russian_names: Some(vec![command.cocktail_name_for_filter.clone()]),
            pagination: Pagination {
                page: command.next_page,
                items_per_page: page_size,
            },
        };
        let _cocktails_names = self.cocktail_repo.get_by_filter(&cocktails_filter).await?;
        let keyboard = inline_keyboards::get_cocktails_list_keyboard(
            &_cocktails_names,
            &PageNumber(command.next_page),
            &page_size,
            ListCocktailsSource::CocktailListByName,
        );
        let _ = if let Some(message_id) = command.message_id {
            let mut send_message = self.bot_provider.bot.edit_message_text(
                command.chat_id,
                message_id,
                escape("Коктейли: "),
            );
            send_message = send_message.reply_markup(keyboard.clone());
            send_message.await?;
        } else {
            let mut send_message = self
                .bot_provider
                .bot
                .send_message(command.chat_id, escape("Коктейли: "));
            send_message = send_message.reply_markup(keyboard.clone());
            send_message.await?;
        };

        Ok(())
    }
}

pub struct GetFavoriteCocktailsListCommand {
    pub callback: CallbackQuery,
    pub next_page: u64,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<GetFavoriteCocktailsListCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: GetFavoriteCocktailsListCommand) -> Result<()> {
        let user_id = command.callback.from.id;
        let chat_id = command.callback.chat_id().unwrap();
        let message_id = command.callback.message.unwrap().id();

        let user = self.user_repo.get_by_telegram_id(&user_id.0).await?;
        if let Some(user) = user {
            let page_size: u64 = 10;
            let cocktails_names = if user.favorite_cocktails.is_empty() {
                CocktailsPaged {
                    items: vec![],
                    total_count: 0,
                }
            } else {
                let cocktails_filter = CocktailFilter {
                    ids: Some(user.favorite_cocktails),
                    names: None,
                    russian_names: None,
                    pagination: Pagination {
                        page: command.next_page,
                        items_per_page: page_size,
                    },
                };
                self.cocktail_repo.get_names(&cocktails_filter).await?
            };
            let keyboard = inline_keyboards::get_cocktails_list_keyboard(
                &cocktails_names,
                &PageNumber(command.next_page),
                &page_size,
                ListCocktailsSource::Favorites,
            );
            let mut edit_message_text =
                self.bot_provider
                    .bot
                    .edit_message_text(chat_id, message_id, "Коктейли: ");
            edit_message_text = edit_message_text.reply_markup(keyboard.clone());
            edit_message_text.await?;
        };

        Ok(())
    }
}

pub struct GetCocktailPagesCommand {
    pub callback: CallbackQuery,
    pub prev_page: MenuCommands,
    pub total_pages: u64,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<GetCocktailPagesCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: GetCocktailPagesCommand) -> Result<()> {
        let callback_cloned = command.callback.clone();
        let chat_id = callback_cloned.chat_id().unwrap();
        let message_id = callback_cloned.message.unwrap().id();

        let keyboard =
            inline_keyboards::get_cocktail_pages_keyboard(&command.total_pages, &command.prev_page);
        let mut edit_message_text =
            self.bot_provider
                .bot
                .edit_message_text(chat_id, message_id, "Доступные страницы: ");
        edit_message_text = edit_message_text.reply_markup(keyboard.clone());
        edit_message_text.await?;

        Ok(())
    }
}

pub struct GetCocktailPageByIdCommand {
    pub callback: CallbackQuery,
    pub prev_page: MenuCommands,
    pub cocktail_id: uuid::Uuid,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<GetCocktailPageByIdCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: GetCocktailPageByIdCommand) -> Result<()> {
        let chat_id = command.callback.chat_id().unwrap();
        let message_id = command.callback.message.unwrap().id();
        let user_id = command.callback.from.id;

        let cocktail = self.cocktail_repo.get_by_id(&command.cocktail_id).await?;
        match cocktail {
            Some(cock) => {
                let mut result_string = format!("🍸*Коктейль:* {}\n", escape(&cock.russian_name));
                result_string.push_str(&format!(
                    "*Английское название:* {}\n",
                    escape(&cock.name.unwrap())
                ));
                result_string.push_str("\n*Ингредиенты:*\n");
                for com_el in cock.composition_elements.unwrap() {
                    result_string.push_str(&format!(
                        "👉 {} {}{}\n",
                        escape(&com_el.name),
                        com_el.count,
                        escape(&com_el.unit)
                    ));
                }
                result_string.push_str("\n*Требуемые инструменты:*\n");
                for tool in cock.tools.unwrap() {
                    result_string.push_str(&format!(
                        "👉 {} {}{}\n",
                        escape(&tool.name),
                        tool.count,
                        escape(&tool.unit)
                    ));
                }
                result_string.push_str("\n*Способ приготовления:*\n");
                for (i, recipe_step) in cock.recipe.unwrap().steps.iter().enumerate() {
                    result_string.push_str(&format!("{}\\. {}\n", i + 1, escape(recipe_step)));
                }
                result_string.push_str("\n*История для этого коктейля:*\n");
                result_string.push_str(&escape(&cock.history.unwrap()));
                result_string.push_str("\n\n*Теги:*\n");
                for tag in cock.tags.unwrap() {
                    result_string.push_str(&format!("\\#{} ", tag.name.replace(" ", "\\_")));
                }

                let user = self.user_repo.get_by_telegram_id(&user_id.0).await?;

                let mut edit_message_text =
                    self.bot_provider
                        .bot
                        .edit_message_text(chat_id, message_id, &result_string);
                let keyboard = if let Some(user) = user {
                    if user.favorite_cocktails.contains(&command.cocktail_id) {
                        inline_keyboards::get_cocktail_card_navigate_keyboard(
                            &command.prev_page,
                            &command.cocktail_id,
                            &Some(true),
                        )
                    } else {
                        inline_keyboards::get_cocktail_card_navigate_keyboard(
                            &command.prev_page,
                            &command.cocktail_id,
                            &Some(false),
                        )
                    }
                } else {
                    inline_keyboards::get_cocktail_card_navigate_keyboard(
                        &command.prev_page,
                        &command.cocktail_id,
                        &None,
                    )
                };
                edit_message_text = edit_message_text.reply_markup(keyboard);

                edit_message_text.await?;
                Ok(())
            }
            None => panic!("Coctail not found"),
        }
    }
}

pub struct GetRegisterUserConfigrationCommand {
    pub chat_id: ChatId,
    pub message_id: MessageId,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<GetRegisterUserConfigrationCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: GetRegisterUserConfigrationCommand) -> Result<()> {
        let mut registration_confirmation_text = "Подтверждая регистрацию, вы соглашаетесь на то, что мы сохраняем ваш идентификатор пользователя Telegram. Другую информацию мы не собираем.\n\n".to_string();
        registration_confirmation_text.push_str("У вас появляется возможность сохранять любимые коктейли в свою личную подборку, чтобы проще было их искать.\n\n");
        registration_confirmation_text
            .push_str("В любой момент вы можете полностью удалить свой профиль.\n");
        registration_confirmation_text.push_str("Приятного использования ☺️");
        let mut edit_message_text = self.bot_provider.bot.edit_message_text(
            command.chat_id,
            command.message_id,
            escape(&registration_confirmation_text),
        );
        let registration_confirmation_keyboard =
            inline_keyboards::get_register_confirmation_keyboard();
        edit_message_text = edit_message_text.reply_markup(registration_confirmation_keyboard);
        edit_message_text.await?;

        Ok(())
    }
}

pub struct RegisterUserCommand {
    pub callback: CallbackQuery,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<RegisterUserCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: RegisterUserCommand) -> Result<()> {
        let user_id = command.callback.from.id;
        let chat_id = command.callback.chat_id().unwrap();
        let message_id = command.callback.message.unwrap().id();

        let user_to_add = User {
            id: Uuid::new_v4(),
            telegram_id: user_id.0,
            favorite_cocktails: vec![],
        };
        self.user_repo.create(&user_to_add).await?;

        let callback_query_answer = self
            .bot_provider
            .bot
            .answer_callback_query(&command.callback.id)
            .show_alert(true)
            .text("Вы успешно зарегистрированы".to_string())
            .await?;
        log::info!("Send callback register result {:?}", callback_query_answer);

        self.handle(GetMainMenuCommand {
            user_id,
            chat_id,
            message_id,
            edit_message: true,
        })
        .await?;

        Ok(())
    }
}

pub struct GetRemoveUserConfirmationCommand {
    pub chat_id: ChatId,
    pub message_id: MessageId,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<GetRemoveUserConfirmationCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: GetRemoveUserConfirmationCommand) -> Result<()> {
        let mut remove_user_confirmation_text =
            "Вы точно хотите удалить свой профиль?\n\n".to_string();

        remove_user_confirmation_text.push_str("Все избранные коктейли будут удалены. 😔\n");

        let mut edit_message_text = self.bot_provider.bot.edit_message_text(
            command.chat_id,
            command.message_id,
            escape(&remove_user_confirmation_text),
        );
        edit_message_text = edit_message_text
            .reply_markup(inline_keyboards::get_remove_user_confirmation_keyboard());
        edit_message_text.await?;
        Ok(())
    }
}

pub struct RemoveUserCommand {
    pub callback: CallbackQuery,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<RemoveUserCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: RemoveUserCommand) -> Result<()> {
        let user_id = command.callback.from.id;
        let chat_id = command.callback.chat_id().unwrap();
        let message_id = command.callback.message.unwrap().id();

        let user = self.user_repo.get_by_telegram_id(&user_id.0).await?;
        if let Some(user) = user {
            self.user_repo.delete(&user).await?;
            let callback_query_answer = self
                .bot_provider
                .bot
                .answer_callback_query(&command.callback.id)
                .show_alert(true)
                .text("Вы успешно удалили свою учетную запись".to_string())
                .await?;
            log::info!(
                "Send callback remove user result {:?}",
                callback_query_answer
            );

            self.handle(GetMainMenuCommand {
                user_id,
                chat_id,
                message_id,
                edit_message: true,
            })
            .await?;
        }
        Ok(())
    }
}

pub struct GetProfilePageCommand {
    pub chat_id: ChatId,
    pub message_id: MessageId,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<GetProfilePageCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: GetProfilePageCommand) -> Result<()> {
        let mut edit_message_text = self.bot_provider.bot.edit_message_text(
            command.chat_id,
            command.message_id,
            "Личный кабинет:",
        );
        edit_message_text =
            edit_message_text.reply_markup(inline_keyboards::get_profile_page_keyboard());
        edit_message_text.await?;
        Ok(())
    }
}

pub struct AddCocktailToFavoriteCommand {
    pub callback: CallbackQuery,
    pub prev_page: MenuCommands,
    pub cocktail_id: uuid::Uuid,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<AddCocktailToFavoriteCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: AddCocktailToFavoriteCommand) -> Result<()> {
        let user_id = command.callback.from.id;
        let user = self.user_repo.get_by_telegram_id(&user_id.0).await?;
        if let Some(mut user) = user {
            user.favorite_cocktails.push(command.cocktail_id);
            self.user_repo.update(&user).await?;
            self.handle(GetCocktailPageByIdCommand {
                callback: command.callback.clone(),
                prev_page: command.prev_page,
                cocktail_id: command.cocktail_id,
            })
            .await?;
            Ok(())
        } else {
            log::warn!("User with id {} not found in store", user_id.0);
            Ok(())
        }
    }
}

pub struct RemoveCocktailFromFavoriteCommand {
    pub callback: CallbackQuery,
    pub prev_page: MenuCommands,
    pub cocktail_id: uuid::Uuid,
}
#[async_trait]
impl<TUserRepo, TCocktailRepo> CommandHandler<RemoveCocktailFromFavoriteCommand>
    for MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo + Sync,
    TCocktailRepo: CocktailRepo + Sync,
{
    async fn handle(&self, command: RemoveCocktailFromFavoriteCommand) -> Result<()> {
        let user_id = command.callback.from.id;
        let user = self.user_repo.get_by_telegram_id(&user_id.0).await?;
        if let Some(mut user) = user {
            let index = user
                .favorite_cocktails
                .iter()
                .position(|x| *x == command.cocktail_id)
                .unwrap();
            user.favorite_cocktails.remove(index);
            self.user_repo.update(&user).await?;
            self.handle(GetCocktailPageByIdCommand {
                callback: command.callback.clone(),
                prev_page: command.prev_page,
                cocktail_id: command.cocktail_id,
            })
            .await?;
            Ok(())
        } else {
            log::warn!("User with id {} not found in store", user_id.0);
            Ok(())
        }
    }
}
