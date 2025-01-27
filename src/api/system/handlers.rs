use std::sync::Arc;

use utoipa_swagger_ui::Config;
use warp::{filters::path::{FullPath, Tail}, reply::Reply};

pub async fn healthcheck_handler() -> Result<impl Reply, warp::Rejection> {
    const MESSAGE: &str = "healthy";

    Ok(warp::reply::json(&MESSAGE))
}

pub async fn serve_swagger_handler(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, warp::Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(warp::http::Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    warp::hyper::Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(warp::hyper::StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            warp::hyper::Response::builder()
                .status(warp::hyper::StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
