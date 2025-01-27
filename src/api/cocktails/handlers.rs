use crate::{
    api::cocktails::models::GetByIdResponse,
    domain::{
        aggregates::cocktail::{CocktailFilter, CocktailRepo},
        Pagination,
    },
    infrastructure,
};

use super::models::{
    CreateRequest, DeleteRequest, ListByFilterRequest, ListByFilterResponse, UpdateRequest,
};

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
        (status = 200, description = "Get by is ended successfully", body = [ListByFilterResponse])
    )
)]
pub async fn list_by_filter(
    filter: ListByFilterRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let repository_factory = infrastructure::RepositoryFactory::global().clone();
    let cocktail_repository = repository_factory.get_cocktails_repository().await.unwrap();
    let cocktail_filter = CocktailFilter {
        ids: filter.ids,
        names: None,
        russian_names: None,
        pagination: Pagination {
            page: filter.pagination.page,
            items_per_page: filter.pagination.items_per_page,
        },
    };
    let cocktails_list = cocktail_repository
        .get_by_filter(&cocktail_filter)
        .await
        .expect("Error while get information about cocktail from db");

    let response = ListByFilterResponse::from(&cocktails_list);
    Ok(warp::reply::json(&response))
}

#[utoipa::path(
    post,
    path = "v1",
    request_body = CreateRequest,
    responses(
        (status = 200, description = "Create cocktail status")
    )
)]
pub async fn create(request: CreateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let repository_factory = infrastructure::RepositoryFactory::global().clone();
    let cocktail_repository = repository_factory.get_cocktails_repository().await.unwrap();
    cocktail_repository.create(&request.into()).await;

    Ok(warp::reply())
}

#[utoipa::path(
    put,
    path = "v1",
    request_body = UpdateRequest,
    responses(
        (status = 200, description = "Update cocktail status")
    )
)]
pub async fn update(request: UpdateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let repository_factory = infrastructure::RepositoryFactory::global().clone();
    let cocktail_repository = repository_factory.get_cocktails_repository().await.unwrap();
    let cocktail_from_db = cocktail_repository
        .get_by_id(&request.id)
        .await
        .expect("Error while get information about cocktail from db");
    let _ = match cocktail_from_db {
        Some(_) => {
            cocktail_repository.update(&request.into()).await;
        }
        None => return Err(warp::reject::not_found()),
    };

    Ok(warp::reply())
}

#[utoipa::path(
    delete,
    path = "v1",
    request_body = DeleteRequest,
    responses(
        (status = 200, description = "Delete cocktail status")
    )
)]
pub async fn delete(request: DeleteRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let repository_factory = infrastructure::RepositoryFactory::global().clone();
    let cocktail_repository = repository_factory.get_cocktails_repository().await.unwrap();
    let cocktail_from_db = cocktail_repository
        .get_by_id(&request.id)
        .await
        .expect("Error while get information about cocktail from db");
    let _ = match cocktail_from_db {
        Some(_) => {
            cocktail_repository.delete(&cocktail_from_db.unwrap()).await;
        }
        None => return Err(warp::reject::not_found()),
    };

    Ok(warp::reply())
}
