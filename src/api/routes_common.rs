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
