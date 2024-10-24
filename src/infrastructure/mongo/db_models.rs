use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::aggregates::{
    cocktail::{Cocktail, CocktailItem, Recipe, Tag},
    user::User,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDbModel {
    pub id: String,
    pub telegram_id: u64,
    pub favorite_cocktails: Vec<String>,
}

impl From<User> for UserDbModel {
    fn from(value: User) -> Self {
        UserDbModel {
            id: value.id.to_string(),
            telegram_id: value.telegram_id,
            favorite_cocktails: value
                .favorite_cocktails
                .iter()
                .map(|i| i.to_string())
                .collect(),
        }
    }
}

impl Into<User> for UserDbModel {
    fn into(self) -> User {
        User {
            id: Uuid::parse_str(&self.id).unwrap(),
            telegram_id: self.telegram_id,
            favorite_cocktails: self
                .favorite_cocktails
                .iter()
                .map(|fc| Uuid::parse_str(fc).unwrap())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocktailDbModel {
    pub id: String,
    pub url: String,
    pub name: String,
    pub russian_name: String,
    pub country_of_origin: String,
    pub history: String,
    pub tags: Vec<TagDbModel>,
    pub tools: Vec<CocktailItemDbModel>,
    pub composition_elements: Vec<CocktailItemDbModel>,
    pub recipe: RecipeDbModel,
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
            tags: value
                .tags
                .iter()
                .map(|x| TagDbModel::from(x.clone()))
                .collect(),
            tools: value
                .tools
                .iter()
                .map(|x| CocktailItemDbModel::from(x.clone()))
                .collect(),
            composition_elements: value
                .composition_elements
                .iter()
                .map(|x| CocktailItemDbModel::from(x.clone()))
                .collect(),
            recipe: RecipeDbModel::from(value.recipe),
        }
    }
}

//impl Into<Cocktail> for CocktailDbModel {
//    fn into(self) -> Cocktail {
//        Cocktail {
//            id: Uuid::parse_str(&self.id).unwrap(),
//            url: self.url,
//            name: self.name,
//            russian_name: self.russian_name,
//            country_of_origin: self.country_of_origin,
//            history: self.history,
//            tags: self.tags.iter().map(|x| Into::into(x.clone())).collect(),
//            tools: self.tools.iter().map(|x| Into::into(x.clone())).collect(),
//            composition_elements: self
//                .composition_elements
//                .iter()
//                .map(|x| Into::into(x.clone()))
//                .collect(),
//            recipe: self.recipe.into(),
//        }
//    }
//}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagDbModel {
    pub name: String,
}

impl From<Tag> for TagDbModel {
    fn from(value: Tag) -> Self {
        TagDbModel { name: value.name }
    }
}

impl Into<Tag> for TagDbModel {
    fn into(self) -> Tag {
        Tag { name: self.name }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocktailItemDbModel {
    pub name: String,
    pub count: i32,
    pub unit: String,
}

impl From<CocktailItem> for CocktailItemDbModel {
    fn from(value: CocktailItem) -> Self {
        CocktailItemDbModel {
            name: value.name,
            count: value.count,
            unit: value.unit,
        }
    }
}

impl Into<CocktailItem> for CocktailItemDbModel {
    fn into(self) -> CocktailItem {
        CocktailItem {
            name: self.name,
            count: self.count,
            unit: self.unit,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeDbModel {
    pub steps: Vec<String>,
}

impl From<Recipe> for RecipeDbModel {
    fn from(value: Recipe) -> Self {
        RecipeDbModel {
            steps: value.steps.iter().map(|x| x.to_string()).collect(),
        }
    }
}

impl Into<Recipe> for RecipeDbModel {
    fn into(self) -> Recipe {
        Recipe { steps: self.steps }
    }
}
