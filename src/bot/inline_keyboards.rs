use teloxide::{types::{InlineKeyboardButton, InlineKeyboardMarkup}, utils::command::BotCommands};

use super::commands::MenuCommands;

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

pub fn get_main_menu_keyboad(user_registered: &bool) -> teloxide::types::InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for button_row in MAIN_MENU_BUTTONS_MAP.chunks(1) {
        let row = button_row
            .iter()
            .map(|btn_info| InlineKeyboardButton::callback(btn_info.name, btn_info.callback_data.as_ref()))
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
