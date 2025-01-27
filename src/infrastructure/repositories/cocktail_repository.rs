use anyhow::{Context, Result};
use async_trait::async_trait;
use mongodb::bson::doc;
use tokio_stream::StreamExt;

use crate::{
    domain::aggregates::cocktail::{Cocktail, CocktailFilter, CocktailRepo, CocktailsPaged},
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
        let _insert_result = self
            .db_client
            .get_cocktails_collection()
            .insert_one(CocktailDbModel::from(entity.clone()))
            .await
            .expect("Error while insert user to database");
    }

    async fn delete(&self, entity: &Cocktail) {
        let uuid_mongo = mongodb::bson::Uuid::parse_str(entity.id.to_string()).unwrap();
        let delete_filter = doc! {"id":  &uuid_mongo};
        let _insert_result = self
            .db_client
            .get_cocktails_collection()
            .find_one_and_delete(delete_filter)
            .await
            .expect("Error while insert user to database");
    }

    async fn update(&self, entity: &Cocktail) {
        let uuid_mongo = mongodb::bson::Uuid::parse_str(entity.id.to_string()).unwrap();
        let _insert_result = self
            .db_client
            .get_cocktails_collection()
            .find_one_and_update(
                doc! {"id": &uuid_mongo},
                CocktailDbModel::from(entity.clone()),
            )
            .await
            .expect("Error while insert user to database");
    }

    async fn get_names(&self, filter: &CocktailFilter) -> Result<CocktailsPaged> {
        let filter_by_ids = if !filter.ids.is_some() {
            let uuids_doc: Vec<mongodb::bson::Document> = filter
                .ids
                .as_ref()
                .map(|ids| {
                    ids
                    .iter()
                    .map(|id| doc! {"id": mongodb::bson::Uuid::parse_str(id.to_string()).unwrap()})
                    .collect()
                })
                .unwrap();
            doc! {"$or":uuids_doc}
        } else {
            doc! {}
        };

        let filter_by_en_names = if let Some(names) = &filter.names {
            let names_doc: Vec<mongodb::bson::Document> =
                names.iter().map(|name| doc! {"name": doc!{"$regex": mongodb::bson::Regex{ pattern: format!("[a-zA-Zа-яА-Я0-9 ]*({})[a-zA-Zа-яА-Я0-9 ]*", name), options: "mi".to_string() }}}).collect();
            doc! {"$or": names_doc}
        } else {
            doc! {}
        };

        let filter_by_russian_names = if let Some(rus_names) = &filter.russian_names {
            let names_doc: Vec<mongodb::bson::Document> =
                rus_names.iter().map(|name| doc! {"russian_name": doc!{"$regex": mongodb::bson::Regex{ pattern: format!("[a-zA-Zа-яА-Я0-9 ]*({})[a-zA-Zа-яА-Я0-9 ]*", name), options: "mi".to_string() }}}).collect();
            doc! {"$or": names_doc}
        } else {
            doc! {}
        };

        let filter_by_names = doc! {"$or": vec![filter_by_en_names, filter_by_russian_names]};
        let filter_document = doc! {"$and": vec![filter_by_names, filter_by_ids]};

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

    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Option<Cocktail>> {
        let cocktail_collection = self.db_client.get_cocktails_collection();
        let uuid_mongo = mongodb::bson::Uuid::parse_str(id.to_string()).unwrap();

        cocktail_collection
            .find_one(doc! {"id": &uuid_mongo})
            .await
            .map(|x| x.map(|x| x.into()))
            .context(format!("Coctail with id {} not found", uuid_mongo))
    }

    async fn get_by_filter(&self, filter: &CocktailFilter) -> Result<CocktailsPaged> {
        let filter_by_ids = if let Some(ids) = &filter.ids {
            let uuids_doc: Vec<mongodb::bson::Document> = ids
                .iter()
                .map(|id| doc! {"id": mongodb::bson::Uuid::parse_str(id.to_string()).unwrap()})
                .collect();
            doc! {"$or":uuids_doc}
        } else {
            doc! {}
        };

        let filter_by_en_names = if let Some(names) = &filter.names {
            let names_doc: Vec<mongodb::bson::Document> =
                names.iter().map(|name| doc! {"name": doc!{"$regex": mongodb::bson::Regex{ pattern: format!("[a-zA-Zа-яА-Я0-9 ]*({})[a-zA-Zа-яА-Я0-9 ]*", name), options: "mi".to_string() }}}).collect();
            doc! {"$or": names_doc}
        } else {
            doc! {}
        };

        let filter_by_russian_names = if let Some(rus_names) = &filter.russian_names {
            let names_doc: Vec<mongodb::bson::Document> =
                rus_names.iter().map(|name| doc! {"russian_name": doc!{"$regex": mongodb::bson::Regex{ pattern: format!("[a-zA-Zа-яА-Я0-9 ]*({})[a-zA-Zа-яА-Я0-9 ]*", name), options: "mi".to_string() }}}).collect();
            doc! {"$or": names_doc}
        } else {
            doc! {}
        };

        let filter_by_names = doc! {"$or": vec![filter_by_en_names, filter_by_russian_names]};
        let filter_document = doc! {"$and": vec![filter_by_names, filter_by_ids]};

        let result = self
            .db_client
            .get_cocktails_collection()
            .find(filter_document.clone())
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
}
