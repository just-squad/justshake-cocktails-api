use warp::{filters::BoxedFilter, Filter};

use crate::json_body;

use super::models::ListByFilterRequest;

fn path_prefix() -> BoxedFilter<()> {
    warp::path("cocktails").boxed()
}

pub fn get_by_id() -> BoxedFilter<(uuid::Uuid,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<uuid::Uuid>())
        .boxed()
}

pub fn list_by_filter() -> BoxedFilter<(ListByFilterRequest, )>{
    warp::post()
    .and(path_prefix())
        .and(warp::path("list").boxed())
        .and(warp::path::end())
        .and(json_body!())
}
