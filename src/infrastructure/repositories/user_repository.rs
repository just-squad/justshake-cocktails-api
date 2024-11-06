use anyhow::{Context, Result};
use async_trait::async_trait;

use mongodb::bson::doc;

use crate::{
    domain::aggregates::user::{User, UserRepo},
    infrastructure::{configurations::DbConfiguration, mongo::MongoDbClient, mongo::UserDbModel},
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
    async fn create(&self, user_entity: &User) -> Result<()> {
        let user_db: UserDbModel = UserDbModel::from(user_entity.clone());
        let user_collection = self.db_client.get_users_collection();
        let _insert_result = user_collection
            .insert_one(user_db)
            .await
            .context("Error while insert user to database")?;
        log::info!(
            "Insert new user in database. Insert id {}",
            _insert_result.inserted_id
        );

        Ok(())
    }

    async fn delete(&self, user_entity: &User) -> Result<()> {
        let uuid_mongo = mongodb::bson::Uuid::parse_str(user_entity.id.to_string()).unwrap();
        let delete_filter = doc! {"id":  &uuid_mongo};
        let _delete_result = self.db_client
            .get_users_collection()
            .find_one_and_delete(delete_filter)
            .await
            .context("Error while delete user from db")?;

        Ok(())
    }

    async fn update(&self, user_entity: &User) -> Result<()> {
        let uuid_mongo = mongodb::bson::Uuid::parse_str(user_entity.id.to_string()).unwrap();
        let user_db: UserDbModel = UserDbModel::from(user_entity.clone());
        log::info!(
            "update user with id {}. favorite cocktails count {}",
            user_db.id,
            user_db.favorite_cocktails.len()
        );
        let _update_result = self
            .db_client
            .get_users_collection()
            .find_one_and_update(doc! {"id": &uuid_mongo}, user_db)
            .await
            .context("Error while update user in db")?;

        Ok(())
    }

    async fn get_by_telegram_id(&self, telegram_user_id: &u64) -> Result<Option<User>> {
        let user = self
            .db_client
            .get_users_collection()
            .find_one(doc! {"telegram_id": telegram_user_id.to_string()})
            .await?;
        match user {
            Some(u) => Ok(Some(u.into())),
            None => Ok(None),
        }
    }

    async fn is_exist_by_telegram_id(&self, telegram_user_id: &u64) -> Result<bool> {
        let user = self
            .db_client
            .get_users_collection()
            .find_one(doc! {"telegram_id": telegram_user_id.to_string()})
            .await?;
        let user_found = user.is_some();

        Ok(user_found)
    }
}
