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
    MainMenu = 0,
    #[strum(to_string = "col")]
    CocktailsList(u64) = 1,
    #[strum(serialize = "sbn")]
    SearchByName = 2,
    #[strum(serialize = "reg")]
    Register = 3,
    #[strum(serialize = "prp")]
    ProfilePage = 4,
    #[strum(serialize = "sbi")]
    SearchById(String) = 5,
    #[strum(serialize = "sbi")]
    NextCocktailsPage(u64) = 6,
    #[strum(serialize = "sbi")]
    PreviousCocktailsPage(u64) = 7,
    #[strum(serialize = "sbi")]
    CocktailsPages = 8,

    Unknown = 99999,
}

impl MenuCommands {
    pub fn parse(s: &str) -> Self {
        let cmd = s.get(..3).unwrap_or_default();
        let param = s.get(3..).unwrap_or_default();

        if cmd == MenuCommands::MainMenu.as_ref() {
            MenuCommands::MainMenu
        } else if cmd == MenuCommands::CocktailsList(0).as_ref() {
            let int_param = param.parse().unwrap_or_default();
            MenuCommands::CocktailsList(int_param)
        } else if cmd == MenuCommands::SearchByName.as_ref() {
            MenuCommands::SearchByName
        } else if cmd == MenuCommands::Register.as_ref() {
            MenuCommands::Register
        } else if cmd == MenuCommands::ProfilePage.as_ref() {
            MenuCommands::ProfilePage
        } else if cmd == MenuCommands::SearchById(String::new()).as_ref() {
            MenuCommands::SearchById(param.to_string())
        } else if cmd == MenuCommands::NextCocktailsPage(0).as_ref() {
            let ulong_param = param.parse().unwrap_or_default();
            MenuCommands::NextCocktailsPage(ulong_param)
        } else if cmd == MenuCommands::PreviousCocktailsPage(0).as_ref() {
            let ulong_param = param.parse().unwrap_or_default();
            MenuCommands::PreviousCocktailsPage(ulong_param)
        } else if cmd == MenuCommands::CocktailsPages.as_ref() {
            MenuCommands::CocktailsPages
        }
        else {
            MenuCommands::Unknown
        }
    }
}
