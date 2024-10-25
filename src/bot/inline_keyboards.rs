use super::commands::MenuCommands;
use crate::domain::aggregates::cocktail::CocktailsPaged;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::markdown::escape;

#[derive(Debug)]
struct MenuButtonMeta<'a> {
    name: &'a str,
    callback_data: &'a MenuCommands,
}

const MAIN_MENU_BUTTONS_MAP: &[MenuButtonMeta] = &[
    MenuButtonMeta {
        name: "📋 Список коктейлей",
        callback_data: &MenuCommands::CocktailsList(0),
    },
    MenuButtonMeta {
        name: "🔎 Поиск по названию",
        callback_data: &MenuCommands::SearchByName,
    },
];

const PROFILE_PAGE_MENU_BTN: &MenuButtonMeta = &MenuButtonMeta {
    name: "🗄 Личная страница",
    callback_data: &MenuCommands::ProfilePage,
};
const REGISTER_PAGE_MENU_BTN: &MenuButtonMeta = &MenuButtonMeta {
    name: "🔑 Регистрация",
    callback_data: &MenuCommands::Register,
};

pub fn get_main_menu_keyboard(user_registered: &bool) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for button_row in MAIN_MENU_BUTTONS_MAP.chunks(1) {
        let row = button_row
            .iter()
            .map(|btn_info| {
                InlineKeyboardButton::callback(btn_info.name, btn_info.callback_data.as_ref())
            })
            .collect();
        keyboard.push(row);
    }
    match user_registered {
        true => {
            keyboard.push(vec![InlineKeyboardButton::callback(
                PROFILE_PAGE_MENU_BTN.name.to_string().to_owned(),
                PROFILE_PAGE_MENU_BTN.callback_data.as_ref(),
            )]);
        }
        false => {
            keyboard.push(vec![InlineKeyboardButton::callback(
                REGISTER_PAGE_MENU_BTN.name.to_string().to_owned(),
                REGISTER_PAGE_MENU_BTN.callback_data.as_ref(),
            )]);
        }
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn get_cocktails_list_keyboard(
    cocktails_paged: &CocktailsPaged,
    current_page: &u64,
    page_size: &u64,
) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for cocktail_info in cocktails_paged.items.chunks(1) {
        let row = cocktail_info
            .iter()
            .map(|cocktail_info| {
                let callback = MenuCommands::SearchById(cocktail_info.id.to_string());
                InlineKeyboardButton::callback(
                    cocktail_info.russian_name.to_owned(),
                    callback.as_ref(),
                )
            })
            .collect();
        keyboard.push(row);
    }
    let available_pages: u64 = cocktails_paged.total_count / page_size;
    let human_current_page = current_page + 1;
    let page_counter_text = escape(format!("{}/{}", human_current_page, available_pages).as_str());

    let next_page = current_page + 1;
    let prev_page = if current_page.clone() == 0 {
        0
    } else {
        current_page - 1
    };
    let cocktail_list_command = String::from(MenuCommands::CocktailsList(0).as_ref());
    let main_menu_command = String::from(MenuCommands::MainMenu.as_ref());

    let navigate_line: Vec<InlineKeyboardButton> = if current_page.clone() == 0 {
        vec![
            InlineKeyboardButton::callback(page_counter_text.clone(), "a"),
            InlineKeyboardButton::callback(
                "👉",
                format!("{}{}", cocktail_list_command, next_page.to_string()),
            ),
        ]
    } else if current_page.clone() == cocktails_paged.total_count / page_size {
        vec![
            InlineKeyboardButton::callback(
                "👈",
                format!("{}{}", cocktail_list_command, prev_page.to_string()),
            ),
            InlineKeyboardButton::callback(page_counter_text.clone(), "a"),
        ]
    } else {
        vec![
            InlineKeyboardButton::callback(
                "👈",
                format!("{}{}", cocktail_list_command, prev_page.to_string()),
            ),
            InlineKeyboardButton::callback(page_counter_text.clone(), "a"),
            InlineKeyboardButton::callback(
                "👉",
                format!("{}{}", cocktail_list_command, next_page.to_string()),
            ),
        ]
    };
    keyboard.push(navigate_line);
    keyboard.push(vec![InlineKeyboardButton::callback("👈 Назад", main_menu_command)]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn get_cocktail_pages_keyboard(){

}
