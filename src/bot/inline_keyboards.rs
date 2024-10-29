use super::commands::MenuCommands;
use crate::domain::aggregates::cocktail::CocktailsPaged;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::markdown::escape;

#[derive(Copy, Clone)]
pub struct PageNumber(pub u64);
impl PageNumber {
    pub fn next(&self) -> Self {
        PageNumber(self.0 + 1)
    }

    pub fn previous(&self) -> Self {
        if self.0 == 0 {
            PageNumber(0)
        } else {
            PageNumber(self.0 - 1)
        }
    }

    pub fn human_readable_page_number(&self) -> PageNumber {
        PageNumber(self.0 + 1)
    }
}

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
    current_page: &PageNumber,
    page_size: &u64,
) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for cocktail_info in cocktails_paged.items.chunks(1) {
        let row = cocktail_info
            .iter()
            .map(|cocktail_info| {
                InlineKeyboardButton::callback(
                    cocktail_info.russian_name.to_owned(),
                    MenuCommands::get_cocktail_by_id_command_string(
                        &cocktail_info.id,
                        &MenuCommands::CocktailsList(0),
                    ),
                )
            })
            .collect();
        keyboard.push(row);
    }
    let available_pages: u64 = (cocktails_paged.total_count / page_size) + 1;
    let page_counter_text = escape(
        format!(
            "{}/{}",
            current_page.human_readable_page_number().0,
            available_pages
        )
        .as_str(),
    );

    let navigate_line: Vec<InlineKeyboardButton> = if current_page.0 == 0 {
        vec![
            InlineKeyboardButton::callback(
                page_counter_text.clone(),
                MenuCommands::get_cocktail_pages_command_string(&(available_pages)),
            ),
            InlineKeyboardButton::callback(
                "👉",
                MenuCommands::get_cocktails_list_command_string(&current_page.next()),
            ),
        ]
    } else if current_page.0 == cocktails_paged.total_count / page_size {
        vec![
            InlineKeyboardButton::callback(
                "👈",
                MenuCommands::get_cocktails_list_command_string(&current_page.previous()),
            ),
            InlineKeyboardButton::callback(
                page_counter_text.clone(),
                MenuCommands::get_cocktail_pages_command_string(&(available_pages)),
            ),
        ]
    } else {
        vec![
            InlineKeyboardButton::callback(
                "👈",
                MenuCommands::get_cocktails_list_command_string(&current_page.previous()),
            ),
            InlineKeyboardButton::callback(
                page_counter_text.clone(),
                MenuCommands::get_cocktail_pages_command_string(&(available_pages)),
            ),
            InlineKeyboardButton::callback(
                "👉",
                MenuCommands::get_cocktails_list_command_string(&current_page.next()),
            ),
        ]
    };
    keyboard.push(navigate_line);
    keyboard.push(vec![InlineKeyboardButton::callback(
        "👈 Назад",
        MenuCommands::get_main_menu_command_string(),
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn get_cocktail_pages_keyboard(total_pages: &u64) -> InlineKeyboardMarkup {
    let from_page: u64 = 1;
    let to_page = total_pages + 1;
    let pages = Vec::from_iter(from_page..to_page);

    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for page_line in pages.chunks(4) {
        let row = page_line
            .iter()
            .map(|page| {
                InlineKeyboardButton::callback(
                    page.to_string(),
                    MenuCommands::get_cocktails_list_command_string(&PageNumber(page - 1)),
                )
            })
            .collect();
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn get_cocktail_card_navigate_keyboard(
    prev_page: &MenuCommands,
    cocktail_id: &uuid::Uuid,
    favorite: &Option<bool>,
) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let mut navigate_row: Vec<InlineKeyboardButton> = vec![];
    let prev_page_command_string = match prev_page {
        MenuCommands::CocktailsList(_) => {
            MenuCommands::get_cocktails_list_command_string(&PageNumber(0))
        }
        MenuCommands::MainMenu => todo!(),
        MenuCommands::SearchByName => todo!(),
        MenuCommands::Register => todo!(),
        MenuCommands::ProfilePage => todo!(),
        MenuCommands::SearchById(_, _) => todo!(),
        MenuCommands::CocktailsPages(_) => todo!(),
        MenuCommands::Unknown => todo!(),
        MenuCommands::AddToFavorite(_, _) => todo!(),
        MenuCommands::RemoveFromFavorite(_, _) => todo!(),
    };
    navigate_row.push(InlineKeyboardButton::callback(
        "👈 Назад",
        prev_page_command_string,
    ));
    if favorite.is_some() {
        let fav_bool = favorite.unwrap();
        if fav_bool {
            navigate_row.push(InlineKeyboardButton::callback(
                "❤️",
                MenuCommands::get_remove_cocktail_from_favourite_command_string(cocktail_id, prev_page),
            ));
        } else {
            navigate_row.push(InlineKeyboardButton::callback(
                "🤍",
                MenuCommands::get_add_cocktail_to_favourite_command_string(cocktail_id, prev_page),
            ));
        }
    }
    keyboard.push(navigate_row);

    InlineKeyboardMarkup::new(keyboard)
}
