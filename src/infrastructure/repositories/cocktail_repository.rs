use anyhow::{Context, Result};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::Bson::Document;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
    domain::aggregates::cocktail::{
        Cocktail, CocktailFilter, CocktailNamesFilter, CocktailRepo, CocktailsPaged,
    },
    infrastructure::{
        configurations::DbConfiguration,
        mongo::{CocktailDbModel, MongoDbClient},
    },
};

#[derive(Debug, Clone)]
pub struct CocktailRepository {
    db_client: MongoDbClient,
}

impl CocktailRepository {
    pub async fn new(config: DbConfiguration) -> Result<Self> {
        let client = MongoDbClient::new(config)
            .await
            .context("failed to create mongodb client from cocktail repository")?;
        Ok(Self { db_client: client })
    }
}

#[async_trait]
impl CocktailRepo for CocktailRepository {
    async fn create(&self, entity: &Cocktail) {
        let cocktail_collection = self.db_client.get_cocktails_collection();
        let _insert_result = cocktail_collection
            .insert_one(CocktailDbModel::from(entity.clone()))
            .await
            .expect("Error while insert user to database");
    }

    async fn get_names(&self, filter: &CocktailNamesFilter) -> Result<CocktailsPaged> {
        let doc = mongodb::bson::Document::new();
        let filter_document = if filter.ids.len() > 0 {
            let uuids_doc: Vec<mongodb::bson::Document> = filter
                .ids
                .iter()
                .map(|id| doc! {"id": id.to_string()})
                .collect();
            doc! {"$or":uuids_doc}
        } else {
            doc! {}
        };

        let result = self
            .db_client
            .get_cocktails_collection()
            .find(filter_document.clone())
            .projection(doc! {"id":1, "russian_name":1})
            .limit(filter.pagination.items_per_page as i64)
            .skip(filter.pagination.page * filter.pagination.items_per_page)
            .await
            .context("failed to find")?
            .map(|x| x.map(|x| x.into()))
            .collect::<Result<_, _>>()
            .await
            .context("fail to collect cocktails in result")?;

        let count_by_filter = self
            .db_client
            .get_cocktails_collection()
            .count_documents(filter_document.clone())
            .await
            .context("failed to count cocktail documents")?;

        let result = CocktailsPaged {
            items: result,
            total_count: count_by_filter,
        };

        Ok(result)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Cocktail> {
        let cocktail_collection = self.db_client.get_cocktails_collection();

        let cocktail_result = cocktail_collection
            .find_one(doc! {"telegram_id": id.to_string()})
            .await?;
        let cocktail = match cocktail_result {
            Some(u) => u,
            None => {
                log::error!("Coctail with id {id} not found.");
                panic!("Coctail with id {id} not found");
            }
        };

        Ok(cocktail.into())
    }

    async fn get_by_filter(&self, _filter: &CocktailFilter) -> Result<CocktailsPaged> {
        let _cocktail_collection = self.db_client.get_cocktails_collection();
        let result = CocktailsPaged {
            items: vec![],
            total_count: 0,
        };

        Ok(result)
    }
}
