use async_trait::async_trait;
use std::error::Error;

use mongodb::{bson::doc, Client, Collection};

use crate::{
    domain::aggregates::user::{User, UserRepo},
    infrastructure::{configurations::DbConfiguration, MongoDbClient},
};

#[derive(Debug, Clone)]
pub struct UserRepository {
    db_client: MongoDbClient,
}

impl UserRepository {
    pub async fn new(config: DbConfiguration) -> Self {
        let client = MongoDbClient::new(config).await;
        UserRepository { db_client: client }
    }
}

#[async_trait]
impl UserRepo for UserRepository {
    async fn create(&self) {
        todo!()
    }

    async fn delete(&self) {
        todo!()
    }

    async fn update(&self) {
        todo!()
    }

    async fn get_by_telegram_id(
        &self,
        telegram_user_id: &u64,
    ) -> Result<User, Box<dyn Error + Send + Sync>> {
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

        Ok(user_result)
    }

    async fn is_exist_by_telegram_id(
        &self,
        telegram_user_id: &u64,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let user = self
            .db_client
            .get_users_collection()
            .find_one(doc! {"telegram_id": telegram_user_id.to_string()})
            .await?;
        let user_found = user.is_some();

        Ok(user_found)
    }
}
