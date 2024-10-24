// public modules
pub mod configurations;

// private modules
mod mongo;
mod repositories;

use anyhow::Result;
use configurations::DbConfiguration;
use repositories::{cocktail_repository::CocktailRepository, user_repository::UserRepository};
use std::sync::OnceLock;

use crate::domain::aggregates::{
    cocktail::CocktailRepo,
    user::UserRepo,
};

pub static REPOFACTORYINSTANCE: OnceLock<RepositoryFactory> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct RepositoryFactory {
    db_configuration: DbConfiguration,
}

impl RepositoryFactory {
    pub fn new(cfg: &DbConfiguration) -> Self {
        RepositoryFactory {
            db_configuration: cfg.clone(),
        }
    }

    pub fn global() -> &'static RepositoryFactory {
        REPOFACTORYINSTANCE
            .get()
            .expect("Can't get instance of RepositoryFactory")
    }
}

impl RepositoryFactory {
    pub async fn get_user_repository(&self) -> Result<impl UserRepo> {
        UserRepository::new(self.db_configuration.clone()).await
    }

    pub async fn get_cocktails_repository(&self) -> Result<impl CocktailRepo> {
        CocktailRepository::new(self.db_configuration.clone()).await
    }
}


