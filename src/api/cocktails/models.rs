use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::aggregates::cocktail::{Cocktail, CocktailItem, Recipe, Tag};

#[derive(Serialize, ToSchema, Clone)]
pub struct GetByIdResponse {
    pub id: uuid::Uuid,
    pub url: Option<String>,
    pub name: Option<String>,
    pub russian_name: String,
    pub country_of_origin: Option<String>,
    pub history: Option<String>,
    pub tags: Option<Vec<TagResponse>>,
    pub tools: Option<Vec<CocktailItemResponse>>,
    pub composition_elements: Option<Vec<CocktailItemResponse>>,
    pub recipe: Option<RecipeResponse>,
}

impl From<Cocktail> for GetByIdResponse {
    fn from(value: Cocktail) -> Self {
        GetByIdResponse {
            id: value.id,
            url: value.url,
            name: value.name,
            russian_name: value.russian_name,
            country_of_origin: value.country_of_origin,
            history: value.history,
            tags: value
                .tags
                .map(|tags| tags.iter().map(|x| TagResponse::from(x)).collect()),
            tools: value.tools.map(|tools| {
                tools
                    .iter()
                    .map(|x| CocktailItemResponse::from(x))
                    .collect()
            }),
            composition_elements: value.composition_elements.map(|composition_elements| {
                composition_elements
                    .iter()
                    .map(|x| CocktailItemResponse::from(x))
                    .collect()
            }),
            recipe: value.recipe.map(|recipe| RecipeResponse::from(recipe)),
        }
    }
}

#[derive(Serialize, ToSchema, Clone)]
pub struct TagResponse {
    pub(crate) name: String,
}

impl From<&Tag> for TagResponse {
    fn from(value: &Tag) -> Self {
        TagResponse {
            name: value.name.clone(),
        }
    }
}

#[derive(Serialize, ToSchema, Clone)]
pub struct CocktailItemResponse {
    pub(crate) name: String,
    pub(crate) count: i32,
    pub(crate) unit: String,
}

impl From<&CocktailItem> for CocktailItemResponse {
    fn from(value: &CocktailItem) -> Self {
        CocktailItemResponse {
            name: value.name.clone(),
            count: value.count.clone(),
            unit: value.unit.clone(),
        }
    }
}

#[derive(Serialize, ToSchema, Clone)]
pub struct RecipeResponse {
    pub(crate) steps: Vec<String>,
}

impl From<Recipe> for RecipeResponse {
    fn from(value: Recipe) -> Self {
        RecipeResponse {
            steps: value.steps.clone(),
        }
    }
}

#[derive(Serialize, ToSchema, Clone)]
pub struct CocktailsPagedResponse {}

#[derive(Deserialize, ToSchema, Clone)]
pub struct ListByFilterRequest {
    ids: Option<Vec<uuid::Uuid>>,
}

#[derive(Deserialize)]
pub struct CreateRequest {}

#[derive(Deserialize)]
pub struct UpdateRequest {}

#[derive(Deserialize)]
pub struct DeleteRequest {}
