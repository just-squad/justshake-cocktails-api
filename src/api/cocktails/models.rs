use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    api::common::PaginationRequest,
    domain::aggregates::cocktail::{Cocktail, CocktailItem, CocktailsPaged, Recipe, Tag},
};

// --------
// GetById
// --------

#[derive(Serialize, ToSchema, Clone)]
pub struct GetByIdResponse {
    pub id: uuid::Uuid,
    pub url: Option<String>,
    pub name: Option<String>,
    pub russian_name: String,
    pub country_of_origin: Option<String>,
    pub history: Option<String>,
    pub tags: Option<Vec<TagDto>>,
    pub tools: Option<Vec<CocktailItemDto>>,
    pub composition_elements: Option<Vec<CocktailItemDto>>,
    pub recipe: Option<RecipeDto>,
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
                .map(|tags| tags.iter().map(|x| TagDto::from(x)).collect()),
            tools: value
                .tools
                .map(|tools| tools.iter().map(|x| CocktailItemDto::from(x)).collect()),
            composition_elements: value.composition_elements.map(|composition_elements| {
                composition_elements
                    .iter()
                    .map(|x| CocktailItemDto::from(x))
                    .collect()
            }),
            recipe: value.recipe.map(|recipe| RecipeDto::from(recipe)),
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct TagDto {
    pub(crate) name: String,
}

impl From<&Tag> for TagDto {
    fn from(value: &Tag) -> Self {
        TagDto {
            name: value.name.clone(),
        }
    }
}

impl Into<Tag> for TagDto {
    fn into(self) -> Tag {
        Tag { name: self.name }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct CocktailItemDto {
    pub(crate) name: String,
    pub(crate) count: i32,
    pub(crate) unit: String,
}

impl From<&CocktailItem> for CocktailItemDto {
    fn from(value: &CocktailItem) -> Self {
        CocktailItemDto {
            name: value.name.clone(),
            count: value.count.clone(),
            unit: value.unit.clone(),
        }
    }
}

impl Into<CocktailItem> for CocktailItemDto {
    fn into(self) -> CocktailItem {
        CocktailItem {
            name: self.name,
            count: self.count,
            unit: self.unit,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct RecipeDto {
    pub(crate) steps: Vec<String>,
}

impl From<Recipe> for RecipeDto {
    fn from(value: Recipe) -> Self {
        RecipeDto {
            steps: value.steps.clone(),
        }
    }
}

impl Into<Recipe> for RecipeDto {
    fn into(self) -> Recipe {
        Recipe { steps: self.steps }
    }
}

// -------------
// ListByFilter
// -------------

#[derive(Deserialize, ToSchema, Clone)]
pub struct ListByFilterRequest {
    pub ids: Option<Vec<uuid::Uuid>>,
    pub pagination: PaginationRequest,
}

#[derive(Serialize, ToSchema, Clone)]
pub struct ListByFilterResponse {
    pub items: Vec<ListByFilterResponseItem>,
    pub total_count: u64,
}

impl From<&CocktailsPaged> for ListByFilterResponse {
    fn from(value: &CocktailsPaged) -> Self {
        ListByFilterResponse {
            items: value
                .items
                .iter()
                .map(|cocktail| ListByFilterResponseItem::from(cocktail))
                .collect(),
            total_count: value.total_count,
        }
    }
}

#[derive(Serialize, ToSchema, Clone)]
pub struct ListByFilterResponseItem {
    pub id: uuid::Uuid,
    pub url: Option<String>,
    pub name: Option<String>,
    pub russian_name: String,
    pub country_of_origin: Option<String>,
    pub tags: Option<Vec<TagDto>>,
}

impl From<&Cocktail> for ListByFilterResponseItem {
    fn from(value: &Cocktail) -> Self {
        let cloned = value.clone();
        ListByFilterResponseItem {
            id: cloned.id,
            url: cloned.url,
            name: cloned.name,
            russian_name: cloned.russian_name,
            country_of_origin: cloned.country_of_origin,
            tags: cloned
                .tags
                .map(|tags| tags.iter().map(|x| TagDto::from(x)).collect()),
        }
    }
}

// --------
// Create
// --------

#[derive(Deserialize, ToSchema)]
pub struct CreateRequest {
    pub name: String,
    pub russian_name: String,
    pub country_of_origin: String,
    pub url: Option<String>,
    pub history: Option<String>,
    pub tags: Option<Vec<TagDto>>,
    pub tools: Option<Vec<CocktailItemDto>>,
    pub composition_elements: Option<Vec<CocktailItemDto>>,
    pub recipe: Option<RecipeDto>,
}

impl Into<Cocktail> for CreateRequest {
    fn into(self) -> Cocktail {
        Cocktail::new(
            Some(self.name),
            self.russian_name,
            self.url,
            Some(self.country_of_origin),
            self.history,
            self.tags
                .map(|tags| tags.iter().map(|tag| tag.clone().into()).collect()),
            self.tools
                .map(|tools| tools.iter().map(|tool| tool.clone().into()).collect()),
            self.composition_elements.map(|compos_elements| {
                compos_elements
                    .iter()
                    .map(|compos_element| compos_element.clone().into())
                    .collect()
            }),
            self.recipe
                .map(|recipe| recipe.clone().into()),
        )
    }
}

// --------
// Update
// --------

#[derive(Deserialize, ToSchema, Clone)]
pub struct UpdateRequest {
    pub id: uuid::Uuid,
    pub name: String,
    pub russian_name: String,
    pub country_of_origin: String,
    pub url: Option<String>,
    pub history: Option<String>,
    pub tags: Option<Vec<TagDto>>,
    pub tools: Option<Vec<CocktailItemDto>>,
    pub composition_elements: Option<Vec<CocktailItemDto>>,
    pub recipe: Option<RecipeDto>,
}

impl Into<Cocktail> for UpdateRequest {
    fn into(self) -> Cocktail {
        Cocktail{
            id: self.id,
            name: Some(self.name),
            russian_name: self.russian_name,
            url: self.url,
            country_of_origin: Some(self.country_of_origin),
            history: self.history,
            tags: self.tags
                .map(|tags| tags.iter().map(|tag| tag.clone().into()).collect()),
            tools: self.tools
                .map(|tools| tools.iter().map(|tool| tool.clone().into()).collect()),
            composition_elements: self.composition_elements.map(|compos_elements| {
                compos_elements
                    .iter()
                    .map(|compos_element| compos_element.clone().into())
                    .collect()
            }),
            recipe: self.recipe
                .map(|recipe| recipe.clone().into()),
        }
    }
}

// --------
// Delete
// --------

#[derive(Deserialize, ToSchema, Clone)]
pub struct DeleteRequest {
    pub id: uuid::Uuid,
}
