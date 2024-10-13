use async_trait::async_trait;
use std::error::Error;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub telegram_id: u64,
    pub favorite_cocktails: Vec<Uuid>,
}

#[async_trait]
pub trait UserRepo {
    /// .
    async fn create(&self);
    /// .
    async fn delete(&self);
    /// .
    async fn update(&self);
    /// .
    async fn get_by_telegram_id(
        &self,
        telegram_user_id: &u64,
    ) -> Result<User, Box<dyn Error + Sync + Send>>;
    /// .
    async fn is_exist_by_telegram_id(
        &self,
        telegram_user_id: &u64,
    ) -> Result<bool, Box<dyn Error + Sync + Send>>;
}
