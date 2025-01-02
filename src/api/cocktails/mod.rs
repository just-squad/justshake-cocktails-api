use utoipa::OpenApi;
use warp::Filter;

// private modules
mod handlers;
mod models;
mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(handlers::get_by_id, handlers::list_by_filter)
)]
pub struct CocktailsApi;

pub fn use_cocktails_api(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    routes::get_by_id()
        .and_then(handlers::get_by_id)
        .or(routes::list_by_filter().and_then(handlers::list_by_filter))
}
