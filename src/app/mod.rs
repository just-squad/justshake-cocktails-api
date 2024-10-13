use envconfig::Envconfig;

use crate::{
    api::configurations::ApiConfiguration, bot::configurations::BotConfig,
    infrastructure::configurations::DbConfiguration,
};

#[derive(Envconfig, Debug, Clone)]
pub struct Config {
    #[envconfig(nested)]
    pub bot_conf: BotConfig,
    #[envconfig(nested)]
    pub db_configuration: DbConfiguration,
    #[envconfig(nested)]
    pub api_configuration: ApiConfiguration,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub config: Config,
}

impl Application {
    pub fn new() -> Self {
        let config = Config::init_from_env().expect("Can't load config from environment");

        Self { config }
    }
}
