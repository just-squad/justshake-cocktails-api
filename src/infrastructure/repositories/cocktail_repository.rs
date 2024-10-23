use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
};
use uuid::Uuid;

use crate::{
    domain::aggregates::cocktail::{
        Cocktail, CocktailFilter, CocktailNamesFilter, CocktailRepo, CocktailsPaged,
    },
    infrastructure::{configurations::DbConfiguration, mongo::MongoDbClient, InfrastructureError},
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
            .insert_one(entity)
            .await
            .expect("Error while insert user to database");
    }

    async fn get_names(
        &self,
        filter: &CocktailNamesFilter,
    ) -> Result<CocktailsPaged, Box<dyn std::error::Error + Sync + Send>> {
        self.db_client
            .get_cocktails_collection()
            .find(doc! {})
            //.projection(doc! {"id":1, "russian_name":1})
            //.limit(filter.pagination.items_per_page as i64)
            //.skip(filter.pagination.page * filter.pagination.items_per_page)
            .await
            .context("failed to find")?
            .map(|x| x.map(|x| x.into()))
            .collect::<Result<_, _>>()
            .await
            .context("failed to deserialize cocktail");
        //{
        //    Ok(cursor) => {
        //        let mut cocktails_res: Vec<Cocktail> = Vec::new();
        //        while let Some(cocktail) = cursor.try_next().await? {
        //            cocktails_res.push(cocktail);
        //        }
        //        cocktails_res
        //    },
        //    Err(e) => Err(e.into()),
        //};
        //let mut coctails_result: Vec<Cocktail> = Vec::new();
        //while let Some(doc) = cursor.next().await {
        //    coctails_result.push(doc.unwrap());
        //}
        let result = CocktailsPaged {
            items: cocktails.unwrap(),
            total_count: 0,
        };

        Ok(result)
    }

    async fn get_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Cocktail, Box<dyn std::error::Error + Sync + Send>> {
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

        Ok(cocktail)
    }

    async fn get_by_filter(
        &self,
        filter: &CocktailFilter,
    ) -> Result<CocktailsPaged, Box<dyn std::error::Error + Sync + Send>> {
        let cocktail_collection = self.db_client.get_cocktails_collection();
        let result = CocktailsPaged {
            items: vec![],
            total_count: 0,
        };

        Ok(result)
    }
}
