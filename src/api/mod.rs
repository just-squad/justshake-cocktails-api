// private modules
mod cocktails;
pub(crate) mod common;
mod system;

// public modules
pub mod configurations;

use cocktails::use_cocktails_api;
use configurations::ApiConfiguration;
use system::use_system_api;
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
        
        let api = use_system_api().or(use_cocktails_api());

        let routes = api.with(self.add_cors()).with(warp::log("api"));

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
