use serde::Serialize;
use warp::reject::Rejection;

pub type ApiResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: ApiResponseStatus,
    pub message: String,
}

#[derive(Serialize)]
pub enum ApiResponseStatus {
    Ok = 1,
    BadRequest = 2,
    InternalServerError = 3,
}
