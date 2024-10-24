pub mod db_models;

use anyhow::{Context, Result};
use db_models::{CocktailDbModel, UserDbModel};
use mongodb::{options::UpdateModifications, Client, Collection};
use serde::{Deserialize, Serialize};

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
        log::info!("Create mongo db connection with connection string {mongo_connection_string}");

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
pub struct Set<T> {
    #[serde{rename="$set"}]
    pub value: T,
}

impl<T> From<Set<T>> for UpdateModifications
where
    T: Serialize,
{
    fn from(val: Set<T>) -> Self {
        UpdateModifications::Document(
            mongodb::bson::to_bson(&val)
                .expect("Can't convert value to bson")
                .as_document()
                .expect("Can't convert bson document to document")
                .to_owned(),
        )
    }
    // add code here
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Insert<T> {
    #[serde{rename="$insert"}]
    pub value: T,
}
