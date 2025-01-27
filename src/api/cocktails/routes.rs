use warp::{filters::BoxedFilter, Filter};

use crate::{api::common::api_prefix, json_body};

use super::models::{CreateRequest, UpdateRequest, DeleteRequest, ListByFilterRequest};

fn path_prefix() -> BoxedFilter<()> {
    warp::path!("cocktails" / "v1" / ..).boxed()
}

pub fn get_by_id() -> BoxedFilter<(uuid::Uuid,)> {
    warp::get()
        .and(api_prefix())
        .and(path_prefix())
        .and(warp::path::param::<uuid::Uuid>())
        .boxed()
}

pub fn list_by_filter() -> BoxedFilter<(ListByFilterRequest,)> {
    warp::post()
        .and(api_prefix())
        .and(path_prefix())
        .and(warp::path("by-filter").boxed())
        .and(warp::path::end())
        .and(json_body!())
        .boxed()
}

pub fn create() -> BoxedFilter<(CreateRequest,)> {
    warp::post()
        .and(api_prefix())
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body!())
        .boxed()
}

pub fn update() -> BoxedFilter<(UpdateRequest,)> {
    warp::put()
        .and(api_prefix())
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body!())
        .boxed()
}

pub fn delete() -> BoxedFilter<(DeleteRequest,)> {
    warp::delete()
        .and(api_prefix())
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body!())
        .boxed()
}
