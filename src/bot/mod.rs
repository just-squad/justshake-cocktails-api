use envconfig::Envconfig;
use std::sync::OnceLock;
use teloxide::{
    adaptors::DefaultParseMode,
    dptree,
    prelude::{Dispatcher, LoggingErrorHandler, RequesterExt},
    types::ParseMode,
    Bot as TBot,
};

pub type Bot = DefaultParseMode<teloxide::Bot>;
pub static INSTANCE: OnceLock<TgBotProvider> = OnceLock::new();

#[derive(Envconfig, Debug, Clone)]
pub struct BotConfig {
    #[envconfig(from = "BOT_TOKEN")]
    pub bot_token: String,
}

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
}

impl TgBotProvider {
    pub async fn start_receive_messages(&self) {
        let bot_instance = self.bot.clone();
        let handler = dptree::entry();

        Dispatcher::builder(bot_instance, handler)
            .default_handler(|upd| async move { log::warn!("Unhandable update {:?}", upd) })
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has been occured in the dispancher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}
