use anyhow::Result;
use async_trait::async_trait;

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
    async fn create(&self, user_entity: &User) -> Result<()>;
    /// .
    async fn delete(&self, user_entity: &User) -> Result<()>;
    /// .
    async fn update(&self, user_entity: &User) -> Result<()>;
    /// .
    async fn get_by_telegram_id(
        &self,
        telegram_user_id: &u64,
    ) -> Result<Option<User>>;
    /// .
    async fn is_exist_by_telegram_id(
        &self,
        telegram_user_id: &u64,
    ) -> Result<bool>;
}
