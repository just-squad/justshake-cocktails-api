use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

#[derive(Debug)]
struct MenuButtonMeta<'a> {
    name: &'a str,
    command: &'a str,
}

const MAIN_MENU_BUTTONS_MAP: &[MenuButtonMeta] = &[
    MenuButtonMeta {
        name: "📋 Список коктейлей",
        command: "cocktails",
    },
    MenuButtonMeta {
        name: "🔎 Поиск по названию",
        command: "searchbyname",
    },
];

const PROFILE_PAGE_MENU_BTN: (&str, &str) = ("🗄 Личная страница", "profile");
const REGISTER_PAGE_MENU_BTN: (&str, &str) = ("🔑 Регистрация", "registerrequest");

pub fn get_main_menu_keyboad(user_registered: &bool) -> teloxide::types::InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for button_row in MAIN_MENU_BUTTONS_MAP.chunks(1) {
        let row = button_row
            .iter()
            .map(|btn_info| InlineKeyboardButton::callback(btn_info.name, btn_info.command))
            .collect();
        keyboard.push(row);
    }
    match user_registered {
        true => {
            keyboard.push(vec![InlineKeyboardButton::callback(
                PROFILE_PAGE_MENU_BTN.0.to_string(),
                PROFILE_PAGE_MENU_BTN.1.to_string(),
            )]);
        }
        false => {
            keyboard.push(vec![InlineKeyboardButton::callback(
                REGISTER_PAGE_MENU_BTN.0.to_string(),
                REGISTER_PAGE_MENU_BTN.1.to_string(),
            )]);
        }
    }

    InlineKeyboardMarkup::new(keyboard)
}
