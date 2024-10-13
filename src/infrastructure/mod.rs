pub mod configurations;
mod repositories;

use configurations::DbConfiguration;
use mongodb::{Client, Collection};
use repositories::user_repository::UserRepository;
use std::sync::OnceLock;

use crate::domain::aggregates::user::{User, UserRepo};

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
    pub async fn get_user_repository(&self) -> impl UserRepo {
        UserRepository::new(self.db_configuration.clone()).await
    }
}

#[derive(Clone, Debug)]
struct MongoDbClient {
    config: DbConfiguration,
    client: Client,
}

impl MongoDbClient {
    pub async fn new(cfg: DbConfiguration) -> Self {
        let mongo_connection_string = format!(
            "mongodb://{}:{}@{}:{}",
            cfg.mongo_username, cfg.mongo_password, cfg.mongo_host, cfg.mongo_port
        );
        let mongodb_client = Client::with_uri_str(&mongo_connection_string)
            .await
            .expect("Can't create MongoDb client");
        MongoDbClient {
            config: cfg.clone(),
            client: mongodb_client,
        }
    }

    pub fn get_users_collection(&self) -> Collection<User> {
        self.client
            .database(&self.config.mongo_database_name)
            .collection::<User>("users")
    }
}
