use envconfig::Envconfig;

#[derive(Envconfig, Debug, Clone)]
pub struct BotConfig {
    #[envconfig(from = "BOT_TOKEN")]
    pub bot_token: String,
}
