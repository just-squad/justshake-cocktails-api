use std::fmt::{Display, Formatter};
use envconfig::Envconfig;

#[derive(Envconfig, Debug, Clone)]
pub struct BotConfig {
    #[envconfig(from = "BOT_TOKEN")]
    pub bot_token: String,
}

impl Display for BotConfig{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bot_token)
    }
}
