use serde::Deserialize;
use utoipa::ToSchema;
use warp::{filters::BoxedFilter, Filter};

pub fn api_prefix() -> BoxedFilter<()> {
    warp::path("api").boxed()
}

#[macro_export]
macro_rules! json_body {
    () => {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    };
}

#[derive(Deserialize, Clone, ToSchema)]
pub struct PaginationRequest {
    pub page: u64,
    pub items_per_page: u64,
}
