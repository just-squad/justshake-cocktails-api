// private modules
mod handlers;
mod cocktails;
pub(crate) mod routes_common;

// public modules
pub mod configurations;
pub mod responses;

use configurations::ApiConfiguration;
use handlers::health_check;
use warp::{filters::cors::Builder, http::Method, Filter};

pub struct ApiProvider {
    api_configuration: ApiConfiguration,
}

impl ApiProvider {
    pub fn new(api_cfg: &ApiConfiguration) -> Self {
        ApiProvider {
            api_configuration: api_cfg.clone(),
        }
    }
}

impl ApiProvider {
    pub async fn start_server(&self) {
        let health_check_path = warp::path!("api" / "healthcheck")
            .and(warp::get())
            .and_then(health_check);

        let routes = health_check_path
            .with(self.add_cors())
            .with(warp::log("api"));

        log::info!("🚀 Server started successfully");
        warp::serve(routes)
            .run(([0, 0, 0, 0], self.api_configuration.http_port))
            .await;
    }
}

impl ApiProvider {
    fn add_cors(&self) -> Builder {
        warp::cors()
            .allow_methods(&[Method::GET, Method::POST])
            .allow_headers(vec!["content-type"])
            .allow_credentials(true)
    }
}
