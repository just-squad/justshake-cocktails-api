use serde::{Deserialize, Serialize};

use crate::domain::aggregates::cocktail::{Cocktail, CocktailItem, Recipe, Tag};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct CocktailDbModel {
    pub(super) id: String,
    pub(super) url: String,
    pub(super) name: String,
    pub(super) russian_name: String,
    pub(super) country_of_origin: String,
    pub(super) history: String,
    pub(super) tags: Vec<TagDbModel>,
    pub(super) tools: Vec<CocktailItemDbModel>,
    pub(super) composition_elements: Vec<CocktailItemDbModel>,
    pub(super) recipe: RecipeDbModel,
}

impl From<Cocktail> for CocktailDbModel {
    fn from(value: Cocktail) -> Self {
        CocktailDbModel {
            id: value.id.to_string(),
            url: value.url,
            name: value.name,
            russian_name: value.russian_name,
            country_of_origin: value.country_of_origin,
            history: value.history,
            tags: todo!(),
            tools: todo!(),
            composition_elements: todo!(),
            recipe: todo!(),
        }
    }
}

impl Into<Cocktail> for CocktailDbModel {
    fn into(self) -> Cocktail {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct TagDbModel {
    pub(super) name: String,
}

impl From<Tag> for TagDbModel {
    fn from(value: Tag) -> Self {
        todo!()
    }
}

impl Into<Tag> for TagDbModel {
    fn into(self) -> Tag {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct CocktailItemDbModel {
    pub(super) name: String,
    pub(super) count: i32,
    pub(super) unit: String,
}

impl From<CocktailItem> for CocktailItemDbModel {
    fn from(value: CocktailItem) -> Self {
        todo!()
    }
}

impl Into<CocktailItem> for CocktailItemDbModel {
    fn into(self) -> CocktailItem {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct RecipeDbModel {
    pub(super) steps: Vec<String>,
}

impl From<Recipe> for RecipeDbModel {
    fn from(value: Recipe) -> Self {
        todo!()
    }
}

impl Into<Recipe> for RecipeDbModel {
    fn into(self) -> Recipe {
        todo!()
    }
}
