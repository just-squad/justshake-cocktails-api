use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cocktail {
    id: Uuid,
    url: String,
    name: String,
    russian_name: String,
    country_of_origin: String,
    history: String,
    tags: Vec<Tag>,
    tools: Vec<CocktailItem>,
    composition_elements: Vec<CocktailItem>,
    recipe: Recipe,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocktailItem {
    name: String,
    count: i32,
    unit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Recipe {
    steps: Vec<String>,
}

pub trait CocktailRepo {
    async fn create(&self);
    async fn get_names(&self);
    async fn get_by_id(&self);
    async fn get_by_filter(&self);
}