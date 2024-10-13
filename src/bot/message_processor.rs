use std::error::Error;

use crate::{bot::TgBotProvider, domain::aggregates::user::UserRepo, infrastructure};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatId, UserId},
};

use super::inline_keyboards;

#[derive(Debug, Clone)]
pub struct MessageProcessor<TUserRepo>
{
    bot_provider: TgBotProvider,
    user_repo: TUserRepo,
}

impl MessageProcessor<()> {
    /// .
    pub async fn new() -> MessageProcessor<impl UserRepo> {
        let bt_prvdr = TgBotProvider::global().clone();
        let repository_factory = infrastructure::RepositoryFactory::global().clone();
        let user_repository = repository_factory.get_user_repository().await;

        MessageProcessor {
            bot_provider: bt_prvdr,
            user_repo: user_repository,
        }
    }
}

impl<TUserRepo> MessageProcessor<TUserRepo>
where
    TUserRepo: UserRepo,
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
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        TUserRepo: UserRepo,
    {
        let user_registered = self.user_repo.is_exist_by_telegram_id(&user_id.0).await?;
        let keyboard = inline_keyboards::get_main_menu_keyboad(&user_registered);

        self.bot_provider
            .bot
            .send_message(*chat_id, "Меню")
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }
}
