use warp::{Filter, http, Reply};

use domain::entities::{Id, Order};

use crate::contracts::OrderRequest;
use crate::order_mem_repository::OrderMemRepository;
use crate::domain::repositories::OrderRepository;
use warp::filters::BoxedFilter;
use uuid::Uuid;

mod contracts;
mod decodes;
mod domain;
mod order_mem_repository;

#[tokio::main]
async fn main() {
    let order_mem_repository = OrderMemRepository::new();
    let order_mem_repository_filter = warp::any().map(move || order_mem_repository.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("orders"))
        .and(warp::path::end())
        .and(decodes::decode_order_request_from_json())
        .and(order_mem_repository_filter.clone())
        .and_then(add_order);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("orders"))
        .and(warp::path::end())
        .and(order_mem_repository_filter.clone())
        .and_then(get_orders);

    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path!("orders" / String))
        .and(warp::path::end())
        .and(decodes::decode_order_request_from_json())
        .and(order_mem_repository_filter.clone())
        .and_then(update_order);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path!("orders" / String))
        .and(warp::path::end())
        .and(order_mem_repository_filter.clone())
        .and_then(delete_order);

    let routes = add_items.or(get_items).or(delete_item).or(update_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn get_orders(
    repository: OrderMemRepository) -> Result<impl warp::Reply, warp::Rejection> {

    let result = repository.search().await;

    Ok(warp::reply::json(
        &result.unwrap()
    ))
}

async fn delete_order(
    id: String,
    repository: OrderMemRepository) -> Result<impl warp::Reply, warp::Rejection> {

    repository.delete(id).await;

    Ok(warp::reply::with_status(
        "Removed item from grocery list",
        http::StatusCode::OK,
    ))
}

async fn update_order(
    id: String,
    mut order_request: OrderRequest,
    repository: OrderMemRepository) -> Result<impl warp::Reply, warp::Rejection> {

    let order = order_request.to_order(id);
    repository.update(order).await;

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

async fn add_order(
    order_request: OrderRequest,
    repository: OrderMemRepository) -> Result<impl warp::Reply, warp::Rejection> {

    let order = order_request.to_order(Uuid::new_v4().to_string());

    repository.create(order).await;

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}