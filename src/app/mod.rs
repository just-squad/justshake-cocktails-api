use envconfig::Envconfig;

use crate::bot::BotConfig;

#[derive(Envconfig, Debug, Clone)]
pub struct Config {
    #[envconfig(nested = true)]
    pub bot_conf: BotConfig,
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
