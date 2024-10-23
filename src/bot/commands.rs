use strum::{AsRefStr, EnumString};
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum MainCommands {
    #[command(parse_with = "split", description = "Основное меню бота\\.")]
    Menu,
}

#[derive(AsRefStr, EnumString, Debug)]
#[repr(i32)]
pub enum MenuCommands {
    #[strum(to_string = "col")]
    CocktailsList(i32) = 0,
    #[strum(serialize = "sbn")]
    SearchByName = 1,
    #[strum(serialize = "reg")]
    Register = 2,
    #[strum(serialize = "prp")]
    ProfilePage = 3,
    Unknown = 99999,
}

impl MenuCommands {
    pub fn parse(s: &str) -> Self {
        let cmd = s.get(..3).unwrap_or_default();
        let int_param = s.get(3..).unwrap_or_default().parse().unwrap_or_default();

        if cmd == MenuCommands::CocktailsList(0).as_ref() {
            MenuCommands::CocktailsList(int_param)
        } else if cmd == MenuCommands::SearchByName.as_ref() {
            MenuCommands::SearchByName
        } else if cmd == MenuCommands::Register.as_ref() {
            MenuCommands::Register
        } else if cmd == MenuCommands::ProfilePage.as_ref() {
            MenuCommands::ProfilePage
        } else {
            MenuCommands::Unknown
        }
    }
}
