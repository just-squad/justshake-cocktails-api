use envconfig::Envconfig;

#[derive(Envconfig, Debug, Clone)]
pub struct DbConfiguration {
    #[envconfig{ from = "MONGO_DATABASE_NAME"}]
    pub mongo_database_name: String,
    #[envconfig{ from = "MONGO_USERNAME" }]
    pub mongo_username: String,
    #[envconfig{ from = "MONGO_PASSWORD" }]
    pub mongo_password: String,
    #[envconfig{ from = "MONGO_HOST" }]
    pub mongo_host: String,
    #[envconfig{ from = "MONGO_PORT" }]
    pub mongo_port: String,
}
