mod lib;
mod models;
mod schema;
mod order_repository;
mod errors;
mod order_service;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use models::*;
use diesel::prelude::*;

use warp::{Filter, reject};

use rustapi::{establish_connection, DbPool};
use crate::order_repository::OrderRepository;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::errors::{handle_rejection, AppError, ErrorType};

#[tokio::main]
async fn main() {
    let sqlite_connection = establish_connection();

    let routes = api_filters(sqlite_connection)
        .recover(handle_rejection);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn api_filters(
    pool: DbPool
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone  {
    warp::path!("v1" / ..)   // Add path prefix /api/v1 to all our routes
        .and(
            add_order(pool.clone())
                .or(get_order_by_id(pool.clone()))
                .or(list_orders(pool.clone()))
                .or(update_order(pool.clone()))
                .or(delete_order(pool))
        )
}

fn add_order(
    pool: DbPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders")
        .and(warp::post())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<Order>())
        .and_then(order_service::add_order)
}

fn list_orders(
    pool: DbPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(order_service::list_orders)
}

fn update_order(
    pool: DbPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders" / String )
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<Order>())
        .and_then(order_service::update)
}

fn delete_order(
    pool: DbPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders" / String )
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(order_service::delete_order)
}

fn get_order_by_id(
    pool: DbPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders" / String )
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(order_service::get_order_by_id)
}

fn with_db_access_manager(pool: DbPool) -> impl Filter<Extract = (OrderRepository,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: DbPool| async move {  match pool.get() {
            Ok(conn) => Ok(OrderRepository::new(conn)),
            Err(err) => Err(reject::custom(
                AppError::new(format!("Error getting connection from pool: {}", err.to_string()).as_str(), ErrorType::Internal))
            ),
        }})
}

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Debug, Serialize, Clone)]
pub struct IdResponse {
    pub id: String,
}

impl IdResponse {
    pub fn new(id: String) -> IdResponse {
        IdResponse { id }
    }
}