pub mod configurations;
mod mongo;
mod repositories;

use anyhow::{Context, Result};
use configurations::DbConfiguration;
use repositories::{cocktail_repository::CocktailRepository, user_repository::UserRepository};
use std::sync::OnceLock;
use thiserror::Error;

use crate::domain::aggregates::{cocktail::CocktailRepo, user::UserRepo};

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
        Ok(UserRepository::new(self.db_configuration.clone())
            .await
            .context("failed to create user_repository")?)
    }

    pub async fn get_cocktails_repository(&self) -> Result<impl CocktailRepo> {
        Ok(CocktailRepository::new(self.db_configuration.clone())
            .await
            .context("failed to create cocktail_repository")?)
    }
}

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
}
