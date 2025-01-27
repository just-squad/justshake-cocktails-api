// private modules
mod handlers;
mod models;
mod routes;

use std::sync::Arc;

use utoipa::OpenApi;
use utoipa_swagger_ui::Config;
use warp::Filter;

pub fn use_system_api(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let config = Arc::new(Config::from("/api-doc.json"));

    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&models::ApiDoc::openapi()));

    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(handlers::serve_swagger_handler);

    routes::healthcheck_route()
        .and_then(handlers::healthcheck_handler)
        .or(api_doc)
        .or(swagger_ui)
}
