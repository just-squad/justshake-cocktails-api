use anyhow::Result;
use std::sync::Arc;

use app::Application;
use bot::TgBotProvider;
use dotenv::dotenv;
use infrastructure::RepositoryFactory;

mod api;
mod app;
mod bot;
mod domain;
mod infrastructure;
mod shared;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Load application settings...");
    let app = Arc::new(Application::new());

    log::info!("Starting bot...");
    let bot_provider = TgBotProvider::new(&app.config.bot_conf);
    bot::INSTANCE
        .set(bot_provider.clone())
        .expect("Can't set static bot provider");
    let repo_factory = RepositoryFactory::new(&app.config.db_configuration);
    infrastructure::REPOFACTORYINSTANCE
        .set(repo_factory.clone())
        .expect("Can't set static repository factory");
    tokio::spawn(start_bot());
    log::info!("Bot started...");
    log::info!("Start Api Server...");
    let api_provider = api::ApiProvider::new(&app.config.api_configuration);
    api_provider.start_server().await;
    Ok(())
}

async fn start_bot() {
    let bot_provider = bot::INSTANCE
        .get()
        .expect("Can't get instance of bot provider. Set instance before get");
    bot_provider.start_receive_messages().await;
}
