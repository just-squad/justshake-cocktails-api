use crate::bot::commands::MainCommands::Menu;
use crate::bot::inline_keyboards::PageNumber;
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
    #[strum(to_string = "mai")]
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
    #[strum(serialize = "ncp")]
    NextCocktailsPage(u64) = 6,
    #[strum(serialize = "pcp")]
    PreviousCocktailsPage(u64) = 7,
    #[strum(serialize = "cop")]
    CocktailsPages(u64) = 8,

    Unknown = 99999,
}

impl MenuCommands {
    pub fn parse(s: &str) -> Self {
        let cmd = s.get(..3).unwrap_or_default();
        let param = s.get(3..).unwrap_or_default();

        if cmd == MenuCommands::MainMenu.as_ref() {
            MenuCommands::MainMenu
        } else if cmd == MenuCommands::CocktailsList(0).as_ref() {
            let ulong_param = param.parse().unwrap_or_default();
            MenuCommands::CocktailsList(ulong_param)
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
        } else if cmd == MenuCommands::CocktailsPages(0).as_ref() {
            let ulong_param = param.parse().unwrap_or_default();
            MenuCommands::CocktailsPages(ulong_param)
        } else {
            MenuCommands::Unknown
        }
    }

    pub fn get_cocktails_list_command_string(page: &PageNumber) -> String {
        let cocktail_list_command = String::from(MenuCommands::CocktailsList(0).as_ref());
        format!("{}{}", cocktail_list_command, page.0.to_string())
    }

    pub fn get_main_menu_command_string() -> String {
        String::from(MenuCommands::MainMenu.as_ref())
    }

    pub fn get_cocktail_pages_command_string(total_pages: &u64) -> String {
        let cocktail_pages_command = String::from(MenuCommands::CocktailsPages(0).as_ref());
        format!("{}{}", cocktail_pages_command, total_pages.to_string())
    }
}
