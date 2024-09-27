use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    telegram_id: i64,
    favorite_cocktails: Vec<Uuid>,
}

pub trait UserRepo {
    async fn create(&self);
    async fn delete(&self);
    async fn update(&self);
    async fn get_by_telegram_id(&self);
    async fn is_exist(&self);
}
