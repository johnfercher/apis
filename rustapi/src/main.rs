use warp::{Filter, http, Reply};

use domain::entities::{Id, Order};

use crate::contracts::OrderRequest;
use crate::order_mem_repository::OrderMemRepository;
use crate::domain::repositories::OrderRepository;
use warp::filters::BoxedFilter;
use uuid::Uuid;
use std::io::Error;

mod contracts;
mod decodes;
mod domain;
mod order_mem_repository;

#[derive(Debug)]
struct FetchError;
impl warp::reject::Reject for FetchError {}

#[tokio::main]
async fn main() {
    let order_mem_repository = OrderMemRepository::new();
    let order_mem_repository_filter = warp::any().map(move || order_mem_repository.clone());

    let add_order = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("orders"))
        .and(warp::path::end())
        .and(decodes::decode_order_request_from_json())
        .and(order_mem_repository_filter.clone())
        .and_then(add_order);

    let get_orders = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("orders"))
        .and(warp::path::end())
        .and(order_mem_repository_filter.clone())
        .and_then(get_orders);

    let get_order_by_id = warp::get()
        .and(warp::path("v1"))
        .and(warp::path!("orders" / String))
        .and(warp::path::end())
        .and(order_mem_repository_filter.clone())
        .and_then(get_order_by_id);

    let update_order = warp::put()
        .and(warp::path("v1"))
        .and(warp::path!("orders" / String))
        .and(warp::path::end())
        .and(decodes::decode_order_request_from_json())
        .and(order_mem_repository_filter.clone())
        .and_then(update_order);

    let delete_order = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path!("orders" / String))
        .and(warp::path::end())
        .and(order_mem_repository_filter.clone())
        .and_then(delete_order);

    let routes = add_order.or(get_orders).or(get_order_by_id).or(delete_order).or(update_order);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn get_orders(
    repository: OrderMemRepository) -> Result<impl warp::Reply, warp::Rejection> {

    match repository.search().await {
        Ok(v) => Ok(warp::reply::json(&v)),
        Err(_) => Err(warp::reject::custom(FetchError))
    }
}

async fn get_order_by_id(
    id: String,
    repository: OrderMemRepository) -> Result<impl warp::Reply, warp::Rejection> {

    match repository.get_by_id(id).await {
        Ok(v) => Ok(warp::reply::json(&v)),
        Err(_) => Err(warp::reject::custom(FetchError))
    }
}

async fn delete_order(
    id: String,
    repository: OrderMemRepository) -> Result<impl warp::Reply, warp::Rejection> {

    match repository.delete(id).await {
        Some(_) => Err(warp::reject::custom(FetchError)),
        None => Ok(warp::reply::with_status(
            "Removed order",
            http::StatusCode::OK,
        ))
    }
}

async fn update_order(
    id: String,
    mut order_request: OrderRequest,
    repository: OrderMemRepository) -> Result<impl warp::Reply, warp::Rejection> {

    let order = order_request.to_order(id);

    match repository.update(order).await {
        Some(_) => Err(warp::reject::custom(FetchError)),
        None => Ok(warp::reply::with_status(
            "Updated order",
            http::StatusCode::OK,
        ))
    }
}

async fn add_order(
    order_request: OrderRequest,
    repository: OrderMemRepository) -> Result<impl warp::Reply, warp::Rejection> {

    let order = order_request.to_order(Uuid::new_v4().to_string());

    match repository.create(order).await {
        Some(_) => Err(warp::reject::custom(FetchError)),
        None => Ok(warp::reply::with_status(
            "Added order",
            http::StatusCode::CREATED,
        ))
    }
}