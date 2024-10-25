use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Cocktail {
    pub(crate) id: Uuid,
    pub(crate) url: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) russian_name: String,
    pub(crate) country_of_origin: Option<String>,
    pub(crate) history: Option<String>,
    pub(crate) tags: Option<Vec<Tag>>,
    pub(crate) tools: Option<Vec<CocktailItem>>,
    pub(crate) composition_elements: Option<Vec<CocktailItem>>,
    pub(crate) recipe: Option<Recipe>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Tag {
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CocktailItem {
    pub(crate) name: String,
    pub(crate) count: i32,
    pub(crate) unit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Recipe {
    pub(crate) steps: Vec<String>,
}

#[async_trait]
pub trait CocktailRepo {
    /// .
    async fn create(&self, entity: &Cocktail);
    /// .
    async fn get_names(&self, filter: &CocktailNamesFilter) -> Result<CocktailsPaged>;
    /// .
    async fn get_by_id(&self, id: &Uuid) -> Result<Cocktail>;
    /// .
    async fn get_by_filter(&self, filter: &CocktailFilter) -> Result<CocktailsPaged>;
}

#[derive(Clone, Debug)]
pub struct CocktailsPaged {
    pub items: Vec<Cocktail>,
    pub total_count: u64,
}

#[derive(Clone, Debug)]
pub struct CocktailFilter {
    pub ids: Vec<Uuid>,
    pub names: Vec<String>,
    pub russian_names: Vec<String>,
    pub pagination: crate::domain::Pagination,
}

#[derive(Clone, Debug)]
pub struct CocktailNamesFilter {
    pub ids: Vec<Uuid>,
    pub pagination: crate::domain::Pagination,
}
