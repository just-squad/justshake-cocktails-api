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

pub enum ListCoctailsSource {
    MainMenu,
    Favorites,
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
    callback_data: &MenuCommands::RegisterConfirmation,
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

pub fn get_register_confirmation_keyboard() -> InlineKeyboardMarkup {
    let keyboard: Vec<Vec<InlineKeyboardButton>> = vec![
        vec![InlineKeyboardButton::callback(
            "Подтвердить регистрацию",
            MenuCommands::Register.as_ref(),
        )],
        vec![InlineKeyboardButton::callback(
            "👈 Назад",
            MenuCommands::get_main_menu_command_string(),
        )],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn get_remove_user_confirmation_keyboard() -> InlineKeyboardMarkup {
    let keyboard: Vec<Vec<InlineKeyboardButton>> = vec![
        vec![InlineKeyboardButton::callback(
            "Подтвердить удаление",
            MenuCommands::RemoveAccount.as_ref(),
        )],
        vec![InlineKeyboardButton::callback(
            "👈 Назад",
            MenuCommands::ProfilePage.as_ref(),
        )],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn get_cocktails_list_keyboard(
    cocktails_paged: &CocktailsPaged,
    current_page: &PageNumber,
    page_size: &u64,
    source: ListCoctailsSource,
) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for cocktail_info in cocktails_paged.items.chunks(1) {
        let row = cocktail_info
            .iter()
            .map(|cocktail_info| {
                let current_page_v = current_page.0;
                let command = match source {
                    ListCoctailsSource::MainMenu => &MenuCommands::CocktailsList(current_page_v),
                    ListCoctailsSource::Favorites => &MenuCommands::ShowFavorites(current_page_v),
                };
                InlineKeyboardButton::callback(
                    cocktail_info.russian_name.to_owned(),
                    MenuCommands::get_cocktail_by_id_command_string(&cocktail_info.id, command),
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
    let next_page_command = match source {
        ListCoctailsSource::MainMenu => {
            MenuCommands::get_cocktails_list_command_string(&current_page.next())
        }
        ListCoctailsSource::Favorites => {
            MenuCommands::get_favorite_cocktails_command_string(&current_page.next())
        }
    };
    let prev_page_command = match source {
        ListCoctailsSource::MainMenu => {
            MenuCommands::get_cocktails_list_command_string(&current_page.previous())
        }
        ListCoctailsSource::Favorites => {
            MenuCommands::get_favorite_cocktails_command_string(&current_page.previous())
        }
    };
    let get_pages_command = match source {
        ListCoctailsSource::MainMenu => MenuCommands::get_cocktail_pages_command_string(
            &available_pages,
            &MenuCommands::CocktailsList(0),
        ),
        ListCoctailsSource::Favorites => MenuCommands::get_cocktail_pages_command_string(
            &available_pages,
            &MenuCommands::ShowFavorites(0),
        ),
    };

    let navigate_line: Vec<InlineKeyboardButton> = if current_page.0 == 0 {
        if available_pages == 1 {
            vec![InlineKeyboardButton::callback(
                page_counter_text.clone(),
                get_pages_command,
            )]
        } else {
            vec![
                InlineKeyboardButton::callback(page_counter_text.clone(), get_pages_command),
                InlineKeyboardButton::callback("👉", next_page_command),
            ]
        }
    } else if current_page.0 == cocktails_paged.total_count / page_size {
        vec![
            InlineKeyboardButton::callback("👈", prev_page_command),
            InlineKeyboardButton::callback(page_counter_text.clone(), get_pages_command),
        ]
    } else {
        vec![
            InlineKeyboardButton::callback("👈", prev_page_command),
            InlineKeyboardButton::callback(page_counter_text.clone(), get_pages_command),
            InlineKeyboardButton::callback("👉", next_page_command),
        ]
    };
    keyboard.push(navigate_line);
    keyboard.push(vec![InlineKeyboardButton::callback(
        "👈 Назад",
        MenuCommands::get_main_menu_command_string(),
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn get_cocktail_pages_keyboard(
    total_pages: &u64,
    source: &MenuCommands,
) -> InlineKeyboardMarkup {
    let from_page: u64 = 1;
    let to_page = total_pages + 1;
    let pages = Vec::from_iter(from_page..to_page);

    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for page_line in pages.chunks(4) {
        let row = page_line
            .iter()
            .map(|page| {
                let list_command = match source {
                    MenuCommands::CocktailsList(_) => {
                        MenuCommands::get_cocktails_list_command_string(&PageNumber(page - 1))
                    }
                    MenuCommands::ShowFavorites(_) => {
                        MenuCommands::get_favorite_cocktails_command_string(&PageNumber(page - 1))
                    }
                    _ => MenuCommands::get_cocktails_list_command_string(&PageNumber(page - 1)),
                };
                InlineKeyboardButton::callback(page.to_string(), list_command)
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
        MenuCommands::CocktailsList(page) => {
            MenuCommands::get_cocktails_list_command_string(&PageNumber(*page))
        }
        MenuCommands::MainMenu => todo!(),
        MenuCommands::SearchByName => todo!(),
        MenuCommands::Register => todo!(),
        MenuCommands::ProfilePage => todo!(),
        MenuCommands::SearchById(_, _, _) => todo!(),
        MenuCommands::CocktailsPages(_, _) => todo!(),
        MenuCommands::Unknown => todo!(),
        MenuCommands::AddToFavorite(_, _) => todo!(),
        MenuCommands::RemoveFromFavorite(_, _) => todo!(),
        MenuCommands::RegisterConfirmation => todo!(),
        MenuCommands::RemoveAccount => todo!(),
        MenuCommands::RemoveAccountConfirmation => todo!(),
        MenuCommands::ShowFavorites(page) => {
            MenuCommands::get_favorite_cocktails_command_string(&PageNumber(*page))
        }
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
                MenuCommands::get_remove_cocktail_from_favourite_command_string(
                    cocktail_id,
                    prev_page,
                ),
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

pub fn get_profile_page_keyboard() -> InlineKeyboardMarkup {
    let keyboard: Vec<Vec<InlineKeyboardButton>> = vec![
        vec![InlineKeyboardButton::callback(
            "❤ Показать избранное",
            MenuCommands::get_favorite_cocktails_command_string(&PageNumber(0)),
        )],
        vec![InlineKeyboardButton::callback(
            "🗑 Удалить учетную запись",
            MenuCommands::RemoveAccountConfirmation.as_ref(),
        )],
        vec![InlineKeyboardButton::callback(
            "👈 Назад",
            MenuCommands::MainMenu.as_ref(),
        )],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
