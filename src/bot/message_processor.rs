use anyhow::{Context, Result};
use std::error::Error;

use super::inline_keyboards;
use crate::bot::inline_keyboards::PageNumber;
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
use teloxide::payloads::EditMessageTextSetters;
use teloxide::types::MessageId;
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
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
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
        _user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
        next_page: &u64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
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
        );
        let mut edit_message_text =
            self.bot_provider
                .bot
                .edit_message_text(*chat_id, *message_id, "Коктейли: ");
        edit_message_text = edit_message_text.reply_markup(keyboard.clone());
        edit_message_text.await?;

        Ok(())
    }

    pub async fn send_cocktails_pages(
        &self,
        _user_id: &UserId,
        chat_id: &ChatId,
        message_id: &MessageId,
        total_pages: &u64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let keyboard = inline_keyboards::get_cocktail_pages_keyboard(total_pages);
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
        chat_id: &ChatId,
        message_id: &MessageId,
        cocktail_id: &uuid::Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let cocktail = self.cocktail_repo.get_by_id(cocktail_id).await?;
        match cocktail {
            Some(cock) => {
                let result_string = format!("🍸<b>Коктейль:</b> {}\n", cock.russian_name);
                //result_string.push_str(format!("<b>Английское название:</b> {}\n", cocktail.name));
                //result_string.push_str("\n<b>Ингредиенты:</b>\n");
                //for _, element := range res.CompositionElements {
                //	resultString = resultString + fmt.Sprintf("👉 %+v %+v%+v\n", element.Name, element.Count, element.Unit)
                //}
                //resultString = resultString + fmt.Sprintf("\n<b>Требуемые инструменты:</b>\n")
                //for _, element := range res.Tools {
                //	resultString = resultString + fmt.Sprintf("👉 %+v %+v%+v\n", element.Name, element.Count, element.Unit)
                //}
                //resultString = resultString + fmt.Sprintf("\n<b>Способ приготовления:</b>\n")
                //for i, element := range res.Recipe.Steps {
                //	resultString = resultString + fmt.Sprintf("%+v. %+v\n", i+1, element)
                //}
                //resultString = resultString + fmt.Sprintf("\n<b>История под этого коктейль:</b>\n")
                //resultString = resultString + res.History
                //resultString = resultString + fmt.Sprintf("\n\n<b>Теги:</b>\n")
                //for _, element := range res.Tags {
                //	resultString = resultString + fmt.Sprintf("#%+v ", element.Name)
                //}

                let edit_message_text =
                    self.bot_provider
                        .bot
                        .edit_message_text(*chat_id, *message_id, result_string);

                edit_message_text.await?;
                Ok(())
            }
            None => Err("Cocktail not found"),
        }
    }

    pub async fn send_cocktails_paged_filter_by_name(
        &self,
        _user_id: &UserId,
        _chat_id: &ChatId,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}
