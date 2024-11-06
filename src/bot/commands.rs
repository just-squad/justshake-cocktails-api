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
    #[strum(to_string = "mam")]
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
    SearchById(String, String, Option<u64>) = 5,
    #[strum(serialize = "cop")]
    CocktailsPages(u64, String) = 6,
    #[strum(serialize = "atf")]
    AddToFavorite(String, String) = 7,
    #[strum(serialize = "rff")]
    RemoveFromFavorite(String, String) = 8,
    #[strum(serialize = "rec")]
    RegisterConfirmation = 9,
    #[strum(serialize = "rea")]
    RemoveAccount = 10,
    #[strum(serialize = "rac")]
    RemoveAccountConfirmation = 11,
    #[strum(serialize = "shf")]
    ShowFavorites(u64) = 12,

    Unknown = 99999,
}

impl MenuCommands {
    pub fn parse(s: &str) -> Self {
        let cmd = s.get(..3).unwrap_or_default();
        let param = s.get(3..).unwrap_or_default().trim();

        // Main Menu
        if cmd == MenuCommands::MainMenu.as_ref() {
            MenuCommands::MainMenu
        }
        /* CoctailsList */
        else if cmd == MenuCommands::CocktailsList(0).as_ref() {
            let ulong_param = param.parse().unwrap_or_default();
            MenuCommands::CocktailsList(ulong_param)
        }
        /* Search by name */
        else if cmd == MenuCommands::SearchByName.as_ref() {
            MenuCommands::SearchByName
        }
        /* Register */
        else if cmd == MenuCommands::Register.as_ref() {
            MenuCommands::Register
        }
        /* Profile page */
        else if cmd == MenuCommands::ProfilePage.as_ref() {
            MenuCommands::ProfilePage
        }
        /* Search by id */
        else if cmd == MenuCommands::SearchById(String::new(), String::new(), Some(0)).as_ref() {
            let params: Vec<&str> = param.split(" ").collect();
            MenuCommands::SearchById(
                params[0].to_string(),
                params[1].to_string(),
                match params[2].parse::<u64>() {
                    Ok(val) => Some(val),
                    _err => None,
                },
            )
        }
        /* Cocktails page */
        else if cmd == MenuCommands::CocktailsPages(0, String::new()).as_ref() {
            let params: Vec<&str> = param.trim().split(" ").collect();
            MenuCommands::CocktailsPages(params[0].parse().unwrap_or_default(), params[1].to_string())
        }
        /* Add to favorite */
        else if cmd == MenuCommands::AddToFavorite(String::new(), String::new()).as_ref() {
            let params: Vec<&str> = param.trim().split(" ").collect();
            MenuCommands::AddToFavorite(params[0].to_string(), params[1].to_string())
        }
        /* Remove from favorite */
        else if cmd == MenuCommands::RemoveFromFavorite(String::new(), String::new()).as_ref() {
            let params: Vec<&str> = param.trim().split(" ").collect();
            MenuCommands::RemoveFromFavorite(params[0].to_string(), params[1].to_string())
        }
        /* Register confirmation */
        else if cmd == MenuCommands::RegisterConfirmation.as_ref() {
            MenuCommands::RegisterConfirmation
        }
        /* Remove account */
        else if cmd == MenuCommands::RemoveAccount.as_ref() {
            MenuCommands::RemoveAccount
        }
        /* Remove account confirmation*/
        else if cmd == MenuCommands::RemoveAccountConfirmation.as_ref() {
            MenuCommands::RemoveAccountConfirmation
        }
        /* Show favorites */
        else if cmd == MenuCommands::ShowFavorites(0).as_ref() {
            let ulong_param = param.parse().unwrap_or_default();
            MenuCommands::ShowFavorites(ulong_param)
        } else {
            MenuCommands::Unknown
        }
    }

    pub fn get_cocktails_list_command_string(page: &PageNumber) -> String {
        let cmd = String::from(MenuCommands::CocktailsList(0).as_ref());
        format!("{} {}", cmd, page.0)
    }

    pub fn get_favorite_cocktails_command_string(page: &PageNumber) -> String {
        let cmd = String::from(MenuCommands::ShowFavorites(0).as_ref());
        format!("{} {}", cmd, page.0)
    }

    pub fn get_main_menu_command_string() -> String {
        String::from(MenuCommands::MainMenu.as_ref())
    }

    pub fn get_cocktail_pages_command_string(total_pages: &u64, source_page: &MenuCommands) -> String {
        let cmd = String::from(MenuCommands::CocktailsPages(0, String::new()).as_ref());
        format!("{} {} {}", cmd, total_pages, source_page.as_ref())
    }

    pub fn get_cocktail_by_id_command_string(
        cocktail_id: &uuid::Uuid,
        source_page: &MenuCommands,
    ) -> String {
        let cmd =
            String::from(MenuCommands::SearchById(String::new(), String::new(), Some(0)).as_ref());
        let prev_page_command = String::from(source_page.as_ref());
        let prev_list_page = match source_page {
            MenuCommands::CocktailsList(page) => Some(*page),
            MenuCommands::ShowFavorites(page) => Some(*page),
            _ => None,
        };
        format!(
            "{} {} {} {}",
            cmd,
            cocktail_id,
            prev_page_command,
            if let Some(prev_list_page) = prev_list_page {
                prev_list_page.to_string()
            } else {
                "".to_string()
            }
        )
    }

    pub fn get_add_cocktail_to_favourite_command_string(
        cocktail_id: &uuid::Uuid,
        source_page: &MenuCommands,
    ) -> String {
        let cmd = String::from(MenuCommands::AddToFavorite(String::new(), String::new()).as_ref());
        let prev_page_command = String::from(source_page.as_ref());
        format!("{} {} {}", cmd, cocktail_id, prev_page_command)
    }

    pub fn get_remove_cocktail_from_favourite_command_string(
        cocktail_id: &uuid::Uuid,
        source_page: &MenuCommands,
    ) -> String {
        let cmd =
            String::from(MenuCommands::RemoveFromFavorite(String::new(), String::new()).as_ref());
        let prev_page_command = String::from(source_page.as_ref());
        format!("{} {} {}", cmd, cocktail_id, prev_page_command)
    }
}
