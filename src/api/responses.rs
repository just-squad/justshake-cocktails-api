use serde::Serialize;
use warp::reject::Rejection;

pub type ApiResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
pub struct ApiResponse<TMessage> {
    pub status: warp::http::StatusCode,
    pub message: TMessage,
}

