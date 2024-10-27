use anyhow::{Context, Result};
use async_trait::async_trait;

use mongodb::bson::doc;

use crate::{
    domain::aggregates::user::{User, UserRepo},
    infrastructure::{
        configurations::DbConfiguration, mongo::MongoDbClient, mongo::Set, mongo::UserDbModel,
    },
};

#[derive(Debug, Clone)]
pub struct UserRepository {
    db_client: MongoDbClient,
}

impl UserRepository {
    pub async fn new(config: DbConfiguration) -> Result<Self> {
        let client = MongoDbClient::new(config)
            .await
            .context("error while create db client for user_repository")?;
        Ok(Self { db_client: client })
    }
}

#[async_trait]
impl UserRepo for UserRepository {
    async fn create(&self, user_entity: &User) {
        let user_collection = self.db_client.get_users_collection();
        let _insert_result = user_collection
            .insert_one(UserDbModel::from(user_entity.clone()))
            .await
            .expect("Error while insert user to database");
    }

    async fn delete(&self, user_entity: &User) {
        let delete_filter = doc! {"id":  user_entity.id.to_string()};
        self.db_client
            .get_users_collection()
            .delete_one(delete_filter)
            .await
            .expect("Error while delete user from db");
    }

    async fn update(&self, user_entity: &User) {
        let update_filter = doc! {"id":  user_entity.id.to_string()};
        let to_set = Set { value: user_entity };
        self.db_client
            .get_users_collection()
            .update_one(update_filter, to_set)
            .await
            .expect("Error while update user in db");
    }

    async fn get_by_telegram_id(
        &self,
        telegram_user_id: &u64,
    ) -> Result<User> {
        let user = self
            .db_client
            .get_users_collection()
            .find_one(doc! {"telegram_id": telegram_user_id.to_string()})
            .await?;
        let user_result = match user {
            Some(u) => u,
            None => {
                log::error!("User with telegram id {telegram_user_id} not found.");
                panic!("User with telegram id {telegram_user_id} not found");
            }
        };

        Ok(user_result.into())
    }

    async fn is_exist_by_telegram_id(
        &self,
        telegram_user_id: &u64,
    ) -> Result<bool> {
        let user = self
            .db_client
            .get_users_collection()
            .find_one(doc! {"telegram_id": telegram_user_id.to_string()})
            .await?;
        let user_found = user.is_some();

        Ok(user_found)
    }
}
