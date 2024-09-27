use std::sync::Arc;

use app::Application;
use bot::TgBotProvider;
use dotenv::dotenv;

mod app;
mod bot;
mod domain;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Load application settings...");
    let app = Arc::new(Application::new());

    log::info!("Starting bot...");
    let bot_provider = TgBotProvider::new(&app.config.bot_conf);
    bot::INSTANCE
        .set(bot_provider.clone())
        .expect("Can't set static bot provider");
    tokio::spawn(start_bot());
    log::info!("Bot started...")
}

async fn start_bot() {
    let bot_provider = bot::INSTANCE
        .get()
        .expect("Can't get instance of bot provider. Set instance before get");
    bot_provider.start_receive_messages().await;
}
