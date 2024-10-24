use anyhow::{Context, Result};
use std::error::Error;

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
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatId, UserId},
};

use super::inline_keyboards;

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
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let user_registered = self.user_repo.is_exist_by_telegram_id(&user_id.0).await?;
        let keyboard = inline_keyboards::get_main_menu_keyboad(&user_registered);

        self.bot_provider
            .bot
            .send_message(*chat_id, "Основное меню:")
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }

    pub async fn send_cocktails_paged(
        &self,
        _user_id: &UserId,
        _chat_id: &ChatId,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let cocktails_filter = CocktailNamesFilter {
            ids: vec![],
            pagination: Pagination {
                page: 0,
                items_per_page: 10,
            },
        };
        let _cocktails_names = self.cocktail_repo.get_names(&cocktails_filter).await?;
        Ok(())
    }

    pub async fn send_cocktails_paged_filter_by_name(
        &self,
        _user_id: &UserId,
        _chat_id: &ChatId,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}
