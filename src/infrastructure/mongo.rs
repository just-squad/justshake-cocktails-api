#![allow(clippy::from_over_into)]

use anyhow::{Context, Result};
use mongodb::{bson::doc, options::UpdateModifications, Client, Collection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::aggregates::{
    cocktail::{Cocktail, CocktailItem, Recipe, Tag},
    user::User,
};

use super::configurations::DbConfiguration;

#[derive(Clone, Debug)]
pub struct MongoDbClient {
    config: DbConfiguration,
    client: Client,
}

impl MongoDbClient {
    pub async fn new(cfg: DbConfiguration) -> Result<Self> {
        let mongo_connection_string: String =
            if cfg.mongo_username.is_empty() || cfg.mongo_password.is_empty() {
                format!("mongodb://{}:{}", cfg.mongo_host, cfg.mongo_port)
            } else {
                format!(
                    "mongodb://{}:{}@{}:{}",
                    cfg.mongo_username, cfg.mongo_password, cfg.mongo_host, cfg.mongo_port
                )
            };
        let mongodb_client = Client::with_uri_str(&mongo_connection_string)
            .await
            .context("failed to create mongodb client")?;
        Ok(Self {
            config: cfg.clone(),
            client: mongodb_client,
        })
    }

    pub fn get_users_collection(&self) -> Collection<UserDbModel> {
        self.client
            .database(&self.config.mongo_database_name)
            .collection::<UserDbModel>("users")
    }

    pub fn get_cocktails_collection(&self) -> Collection<CocktailDbModel> {
        self.client
            .database(&self.config.mongo_database_name)
            .collection::<CocktailDbModel>("cocktails")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Insert<T> {
    #[serde{rename="$insert"}]
    pub value: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDbModel {
    pub id: mongodb::bson::uuid::Uuid,
    pub telegram_id: String,
    pub favorite_cocktails: Vec<mongodb::bson::uuid::Uuid>,
}

impl From<User> for UserDbModel {
    fn from(value: User) -> Self {
        UserDbModel {
            id: mongodb::bson::Uuid::parse_str(value.id.to_string()).unwrap(),
            telegram_id: value.telegram_id.to_string(),
            favorite_cocktails: value
                .favorite_cocktails
                .iter()
                .map(|i| mongodb::bson::Uuid::parse_str(i.to_string()).unwrap())
                .collect(),
        }
    }
}

impl Into<User> for UserDbModel {
    fn into(self) -> User {
        User {
            id: Uuid::parse_str(&self.id.to_string()).unwrap(),
            telegram_id: self.telegram_id.parse().unwrap(),
            favorite_cocktails: self
                .favorite_cocktails
                .iter()
                .map(|fc| Uuid::parse_str(&fc.to_string()).unwrap())
                .collect(),
        }
    }
}

impl Into<UpdateModifications> for UserDbModel {
    fn into(self) -> UpdateModifications {
        UpdateModifications::Document(doc! {"$set":{"favorite_cocktails": self.favorite_cocktails}})
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocktailDbModel {
    pub id: mongodb::bson::uuid::Uuid,
    pub url: Option<String>,
    pub name: Option<String>,
    pub russian_name: String,
    pub country_of_origin: Option<String>,
    pub history: Option<String>,
    pub tags: Option<Vec<TagDbModel>>,
    pub tools: Option<Vec<CocktailItemDbModel>>,
    pub composition_elements: Option<Vec<CocktailItemDbModel>>,
    pub recipe: Option<RecipeDbModel>,
}

impl From<Cocktail> for CocktailDbModel {
    fn from(value: Cocktail) -> Self {
        CocktailDbModel {
            id: mongodb::bson::uuid::Uuid::parse_str(value.id.to_string()).unwrap(),
            url: value.url,
            name: value.name,
            russian_name: value.russian_name,
            country_of_origin: value.country_of_origin,
            history: value.history,
            tags: value
                .tags
                .map(|tags| tags.iter().map(|x| TagDbModel::from(x.clone())).collect()),
            tools: value.tools.map(|tools| {
                tools
                    .iter()
                    .map(|x| CocktailItemDbModel::from(x.clone()))
                    .collect()
            }),
            composition_elements: value.composition_elements.map(|composition_elements| {
                composition_elements
                    .iter()
                    .map(|x| CocktailItemDbModel::from(x.clone()))
                    .collect()
            }),
            recipe: value.recipe.map(RecipeDbModel::from),
        }
    }
}

impl Into<Cocktail> for CocktailDbModel {
    fn into(self) -> Cocktail {
        Cocktail {
            id: Uuid::parse_str(&self.id.to_string()).unwrap(),
            url: self.url,
            name: self.name,
            russian_name: self.russian_name,
            country_of_origin: self.country_of_origin,
            history: self.history,
            tags: self
                .tags
                .map(|tags| tags.iter().map(|x| Into::into(x.clone())).collect()),
            tools: self
                .tools
                .map(|tags| tags.iter().map(|x| Into::into(x.clone())).collect()),
            composition_elements: self.composition_elements.map(|composition_elements| {
                composition_elements
                    .iter()
                    .map(|x| Into::into(x.clone()))
                    .collect()
            }),
            recipe: self.recipe.map(|recipe| recipe.into()),
        }
    }
}

impl Into<UpdateModifications> for CocktailDbModel {
    fn into(self) -> UpdateModifications {
        let bson_tags = mongodb::bson::to_bson(&self.tags).unwrap();
        let bson_composition_elements = mongodb::bson::to_bson(&self.composition_elements).unwrap();
        let bson_tools = mongodb::bson::to_bson(&self.tools).unwrap();
        let bson_recipe = mongodb::bson::to_bson(&self.recipe).unwrap();

        UpdateModifications::Document(doc! {"$set":{"name": self.name},
            "$set":{"russian_name": self.russian_name},
            "$set":{"country_of_origin": self.country_of_origin},
            "$set":{"history": self.history},
            "$set":{"url": self.url},
            "$set":{"tags": bson_tags},
            "$set":{"composition_elements": bson_composition_elements},
            "$set":{"tools": bson_tools},
            "$set":{"recipe": bson_recipe}
        })
    }
}

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

impl Into<UpdateModifications> for TagDbModel {
    fn into(self) -> UpdateModifications {
        UpdateModifications::Document(doc! {"$set":{"name": self.name}
        })
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
