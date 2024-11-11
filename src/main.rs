use anyhow::Result;
use std::{fs::File, io, path::Path, sync::Arc};

use app::Application;
use bot::TgBotProvider;
use infrastructure::RepositoryFactory;

mod api;
mod app;
mod bot;
mod domain;
mod infrastructure;
mod shared;

#[tokio::main]
async fn main() -> Result<()> {
    load_app_cfg()?;
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

fn load_app_cfg() -> Result<()> {
    let local_env_path = Path::new(".env.local");
    if local_env_path.exists() {
        dotenvy::dotenv()?;
        dotenvy::from_path(local_env_path)?;
        Ok(())
    } else {
        dotenvy::dotenv().ok();
        Ok(())
    }
}
