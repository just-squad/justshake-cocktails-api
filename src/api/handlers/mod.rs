// public modules
pub mod cocktails;

use warp::{reply::json, Reply, http::StatusCode};

use crate::api::responses::ApiResponse;

use super::responses::ApiResult;

pub async fn health_check() -> ApiResult<impl Reply> {
    const MESSAGE: &str = "healthy";

    let response = &ApiResponse {
        status: StatusCode::OK,
        message: MESSAGE.to_string(),
    };
    Ok(json(response))
}
