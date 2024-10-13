use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum MainCommands {
    #[command(parse_with = "split", description = "Основное меню бота\\.")]
    Menu,
}
