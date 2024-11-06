use anyhow::{Context, Result};
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::utils::markdown::escape;
use uuid::Uuid;

use super::commands::MenuCommands;
use super::inline_keyboards::{self, ListCoctailsSource};
use crate::bot::inline_keyboards::PageNumber;
use crate::domain::aggregates::user::User;
use crate::{
    bot::TgBotProvider,
    domain::{
        aggregates::{
            cocktail::{CocktailNamesFilter, CocktailRepo},
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

impl<TUserRepo, TCocktailRepo> MessageProcessor<TUserRepo, TCocktailRepo>
where
    TUserRepo: UserRepo,
    TCocktailRepo: CocktailRepo,
{
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn send_menu_to_user(
        &self,
        user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
        edit_message: bool,
    ) -> Result<()> {
        let user_registered = self.user_repo.is_exist_by_telegram_id(&user_id.0).await?;
        let keyboard = inline_keyboards::get_main_menu_keyboard(&user_registered);

        if edit_message {
            let mut edit_message_text =
                self.bot_provider
                    .bot
                    .edit_message_text(*chat_id, *message_id, "Основное меню: ");
            edit_message_text = edit_message_text.reply_markup(keyboard.clone());
            edit_message_text.await?;
        } else {
            self.bot_provider
                .bot
                .send_message(*chat_id, "Основное меню:")
                .reply_markup(keyboard)
                .await?;
        }

        Ok(())
    }

    pub async fn send_cocktails_paged(
        &self,
        callback: &CallbackQuery,
        next_page: &u64,
    ) -> Result<()> {
        let page_size: u64 = 10;
        let cocktails_filter = CocktailNamesFilter {
            ids: vec![],
            pagination: Pagination {
                page: *next_page,
                items_per_page: page_size,
            },
        };
        let _cocktails_names = self.cocktail_repo.get_names(&cocktails_filter).await?;
        let keyboard = inline_keyboards::get_cocktails_list_keyboard(
            &_cocktails_names,
            &PageNumber(*next_page),
            &page_size,
            ListCoctailsSource::MainMenu,
        );
        let callback_cloned = callback.clone();
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

    pub async fn send_cocktails_pages(
        &self,
        prev_page: &MenuCommands,
        _user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
        total_pages: &u64,
    ) -> Result<()> {
        let keyboard = inline_keyboards::get_cocktail_pages_keyboard(total_pages, prev_page);
        let mut edit_message_text =
            self.bot_provider
                .bot
                .edit_message_text(*chat_id, *message_id, "Доступные страницы: ");
        edit_message_text = edit_message_text.reply_markup(keyboard.clone());
        edit_message_text.await?;

        Ok(())
    }

    pub async fn send_cocktail_page(
        &self,
        prev_page: &MenuCommands,
        user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
        cocktail_id: &uuid::Uuid,
    ) -> Result<()> {
        let cocktail = self.cocktail_repo.get_by_id(cocktail_id).await?;
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
                        .edit_message_text(*chat_id, *message_id, &result_string);
                let keyboard = if let Some(user) = user {
                    if user.favorite_cocktails.contains(cocktail_id) {
                        inline_keyboards::get_cocktail_card_navigate_keyboard(
                            prev_page,
                            cocktail_id,
                            &Some(true),
                        )
                    } else {
                        inline_keyboards::get_cocktail_card_navigate_keyboard(
                            prev_page,
                            cocktail_id,
                            &Some(false),
                        )
                    }
                } else {
                    inline_keyboards::get_cocktail_card_navigate_keyboard(
                        prev_page,
                        cocktail_id,
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

    pub async fn send_cocktails_paged_filter_by_name(
        &self,
        _user_id: &UserId,
        _chat_id: &ChatId,
    ) -> Result<()> {
        Ok(())
    }

    pub async fn send_register_user_confirmation(
        &self,
        _user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
    ) -> Result<()> {
        let mut registration_confirmation_text = "Подтверждая регистрацию, вы соглашаетесь на то, что мы сохраняем ваш идентификатор пользователя Telegram. Другую информацию мы не собираем.\n\n".to_string();
        registration_confirmation_text.push_str("У вас появляется возможность сохранять любимые коктейли в свою личную подборку, чтобы проще было их искать.\n\n");
        registration_confirmation_text
            .push_str("В любой момент вы можете полностью удалить свой профиль.\n");
        registration_confirmation_text.push_str("Приятного использования ☺️");
        let mut edit_message_text = self.bot_provider.bot.edit_message_text(
            *chat_id,
            *message_id,
            escape(&registration_confirmation_text),
        );
        let registration_confirmation_keyboard =
            inline_keyboards::get_register_confirmation_keyboard();
        edit_message_text = edit_message_text.reply_markup(registration_confirmation_keyboard);
        edit_message_text.await?;

        Ok(())
    }

    pub async fn send_profile_page(
        &self,
        _user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
    ) -> Result<()> {
        let mut edit_message_text =
            self.bot_provider
                .bot
                .edit_message_text(*chat_id, *message_id, "Личный кабинет:");
        edit_message_text =
            edit_message_text.reply_markup(inline_keyboards::get_profile_page_keyboard());
        edit_message_text.await?;
        Ok(())
    }

    pub async fn send_remove_user_confirmation(
        &self,
        _user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
    ) -> Result<()> {
        let mut remove_user_confirmation_text =
            "Вы точно хотите удалить свой профиль?\n\n".to_string();

        remove_user_confirmation_text.push_str("Все избранные коктейли будут удалены. 😔\n");

        let mut edit_message_text = self.bot_provider.bot.edit_message_text(
            *chat_id,
            *message_id,
            escape(&remove_user_confirmation_text),
        );
        edit_message_text = edit_message_text
            .reply_markup(inline_keyboards::get_remove_user_confirmation_keyboard());
        edit_message_text.await?;
        Ok(())
    }

    pub async fn register_user(&self, callback_query: &CallbackQuery) -> Result<()> {
        let callback = callback_query.clone();
        let user_id = callback.from.id;
        let chat_id = callback.chat_id().unwrap();
        let message_id = callback.message.unwrap().id();

        let user_to_add = User {
            id: Uuid::new_v4(),
            telegram_id: user_id.0,
            favorite_cocktails: vec![],
        };
        self.user_repo.create(&user_to_add).await?;

        let callback_query_answer = self
            .bot_provider
            .bot
            .answer_callback_query(&callback.id)
            .show_alert(true)
            .text("Вы успешно зарегистрированы".to_string())
            .await?;
        log::info!("Send callback register result {:?}", callback_query_answer);

        self.send_profile_page(&user_id, &chat_id, &message_id)
            .await?;

        Ok(())
    }

    pub async fn remove_user(&self, callback_query: &CallbackQuery) -> Result<()> {
        let callback = callback_query.clone();
        let user_id = callback.from.id;
        let chat_id = callback.chat_id().unwrap();
        let message_id = callback.message.unwrap().id();

        let user = self.user_repo.get_by_telegram_id(&user_id.0).await?;
        if let Some(user) = user {
            self.user_repo.delete(&user).await?;
            let callback_query_answer = self
                .bot_provider
                .bot
                .answer_callback_query(&callback.id)
                .show_alert(true)
                .text("Вы успешно удалили свою учетную запись".to_string())
                .await?;
            log::info!(
                "Send callback remove user result {:?}",
                callback_query_answer
            );

            self.send_menu_to_user(&user_id, &chat_id, &message_id, true)
                .await?;
        }
        Ok(())
    }

    pub async fn add_coctail_to_favorite(
        &self,
        prev_page: &MenuCommands,
        user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
        cocktail_id: &uuid::Uuid,
    ) -> Result<()> {
        let user = self.user_repo.get_by_telegram_id(&user_id.0).await?;
        if let Some(mut user) = user {
            user.favorite_cocktails.push(*cocktail_id);
            self.user_repo.update(&user).await?;
            self.send_cocktail_page(prev_page, user_id, chat_id, message_id, cocktail_id)
                .await?;
            Ok(())
        } else {
            log::warn!("User with id {} not found in store", user_id.0);
            Ok(())
        }
    }

    pub async fn remove_coctail_from_favorite(
        &self,
        prev_page: &MenuCommands,
        user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
        cocktail_id: &uuid::Uuid,
    ) -> Result<()> {
        let user = self.user_repo.get_by_telegram_id(&user_id.0).await?;
        if let Some(mut user) = user {
            let index = user
                .favorite_cocktails
                .iter()
                .position(|x| *x == *cocktail_id)
                .unwrap();
            user.favorite_cocktails.remove(index);
            self.user_repo.update(&user).await?;
            self.send_cocktail_page(prev_page, user_id, chat_id, message_id, cocktail_id)
                .await?;
            Ok(())
        } else {
            log::warn!("User with id {} not found in store", user_id.0);
            Ok(())
        }
    }

    pub async fn send_favorite_cocktails(
        &self,
        callback: &CallbackQuery,
        next_page: &u64,
    ) -> Result<()> {
        let callback_cloned = callback.clone();
        let user_id = callback_cloned.from.id;
        let chat_id = callback_cloned.chat_id().unwrap();
        let message_id = callback_cloned.message.unwrap().id();

        let user = self.user_repo.get_by_telegram_id(&user_id.0).await?;
        if let Some(user) = user {
            let page_size: u64 = 10;
            let cocktails_filter = CocktailNamesFilter {
                ids: user.favorite_cocktails,
                pagination: Pagination {
                    page: *next_page,
                    items_per_page: page_size,
                },
            };
            let _cocktails_names = self.cocktail_repo.get_names(&cocktails_filter).await?;
            let keyboard = inline_keyboards::get_cocktails_list_keyboard(
                &_cocktails_names,
                &PageNumber(*next_page),
                &page_size,
                ListCoctailsSource::Favorites,
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
