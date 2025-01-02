use crate::{
    api::cocktails::models::GetByIdResponse, domain::{
        aggregates::cocktail::{CocktailFilter, CocktailRepo},
        Pagination,
    }, infrastructure

};

use super::models::{ListByFilterRequest, CocktailsPagedResponse};

#[utoipa::path(
        get,
        path = "v1/{id}",
        params(
            ("id" = uuid::Uuid, Path, description = "Identifier of cocktail.")
        ),
        responses(
            (status = 200, description = "Get by is ended successfully", body = [GetByIdResponse])
        )
    )]
pub async fn get_by_id(id: uuid::Uuid) -> Result<impl warp::Reply, warp::Rejection> {
    let repository_factory = infrastructure::RepositoryFactory::global().clone();
    let cocktail_repository = repository_factory.get_cocktails_repository().await.unwrap();
    let cocktail_from_db = cocktail_repository
        .get_by_id(&id)
        .await
        .expect("Error while get information about cocktail from db");
    let result = match cocktail_from_db {
        Some(cocktail) => cocktail,
        None => return Err(warp::reject::not_found()),
    };

    let response: GetByIdResponse = GetByIdResponse::from(result.clone());

    Ok(warp::reply::json(&response))
}

#[utoipa::path(
    post,
    path = "v1/by-filter",
    request_body = ListByFilterRequest,
    responses(
        (status = 200, description = "Get by is ended successfully", body = [CocktailsPagedResponse])
    )
)]
pub async fn list_by_filter(
    filter: ListByFilterRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = filter;
    let repository_factory = infrastructure::RepositoryFactory::global().clone();
    let cocktail_repository = repository_factory.get_cocktails_repository().await.unwrap();
    let cocktail_filter = CocktailFilter {
        ids: None,
        names: None,
        russian_names: None,
        pagination: Pagination {
            page: 1,
            items_per_page: 10,
        },
    };
    let cocktails_list = cocktail_repository
        .get_by_filter(&cocktail_filter)
        .await
        .expect("Error while get information about cocktail from db");

    Ok(warp::reply::json(&cocktails_list))
}

//pub async fn create(request: CreateRequest) -> Result<impl warp::Reply, warp::Rejection> {
//    let repository_factory = infrastructure::RepositoryFactory::global().clone();
//    let cocktail_repository = repository_factory.get_cocktails_repository().await.unwrap();
//    let cocktail_from_db = cocktail_repository
//        .get_by_id(&id)
//        .await
//        .expect("Error while get information about cocktail from db");
//    let result = match cocktail_from_db {
//        Some(cocktail) => cocktail,
//        None => return Err(warp::reject::not_found()),
//    };
//
//    Ok(warp::reply::json(&result))
//}
//
//pub async fn update(request: UpdateRequest) -> Result<impl warp::Reply, warp::Rejection> {
//    let repository_factory = infrastructure::RepositoryFactory::global().clone();
//    let cocktail_repository = repository_factory.get_cocktails_repository().await.unwrap();
//    let cocktail_from_db = cocktail_repository
//        .get_by_id(&id)
//        .await
//        .expect("Error while get information about cocktail from db");
//    let result = match cocktail_from_db {
//        Some(cocktail) => cocktail,
//        None => return Err(warp::reject::not_found()),
//    };
//
//    Ok(warp::reply::json(&result))
//}
//
//pub async fn delete(request: DeleteRequest) -> Result<impl warp::Reply, warp::Rejection> {
//    let repository_factory = infrastructure::RepositoryFactory::global().clone();
//    let cocktail_repository = repository_factory.get_cocktails_repository().await.unwrap();
//    let cocktail_from_db = cocktail_repository
//        .get_by_id(&id)
//        .await
//        .expect("Error while get information about cocktail from db");
//    let result = match cocktail_from_db {
//        Some(cocktail) => cocktail,
//        None => return Err(warp::reject::not_found()),
//    };
//
//    Ok(warp::reply::json(&result))
//}
