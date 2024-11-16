use warp::{reply::json, Reply};

use crate::api::responses::ApiResponse;

use super::responses::ApiResult;

pub async fn health_check() -> ApiResult<impl Reply> {
    const MESSAGE: &str = "healthy";

    let response = &ApiResponse {
        status: crate::api::responses::ApiResponseStatus::Ok,
        message: MESSAGE.to_string(),
    };
    Ok(json(response))
}
