mod lib;
mod models;
mod schema;
mod db_manager;
mod errors;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use models::*;
use diesel::prelude::*;

use warp::{Filter, reject};

use rustapi::{establish_connection, PgPool};
use crate::db_manager::DBAccessManager;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::errors::{handle_rejection, AppError};

#[derive(Debug)]
struct FetchError;
impl warp::reject::Reject for FetchError {}

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
    pool: PgPool
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone  {
    warp::path!("v1" / ..)   // Add path prefix /api/v1 to all our routes
        .and(
            add_order(pool.clone())
                .or(list_orders(pool))
        )
}

/// POST /books
fn add_order(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders")                    // Match /books path
        .and(warp::post())                  // Match POST method
        .and(with_db_access_manager(pool))  // Add DBAccessManager to params tuple
        .and(with_json_body::<Order>())   // Try to deserialize JSON body to AddBook
        .and_then(api_add_order)            // Pass the params touple to the handler function
}

/// GET /books
fn list_orders(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(api_list_orders)
}


fn with_db_access_manager(pool: PgPool) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {  match pool.get() {
            Ok(conn) => Ok(DBAccessManager::new(conn)),
            Err(err) => Err(reject::custom(FetchError)
            ),
        }})
}

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn api_add_order(
    db_manager: DBAccessManager,
    order: Order,
) -> Result<impl warp::Reply, warp::Rejection> {

    let id_response = db_manager.create_book(order).map(|order|
        { IdResponse::new(String::from("blabla")) }
    );

    respond(id_response, warp::http::StatusCode::CREATED)
}

pub async fn api_list_orders(
    db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {

    let result = db_manager.list_orders();

    respond(result, warp::http::StatusCode::OK)
}

fn respond<T: Serialize>(result: Result<T, AppError>, status: warp::http::StatusCode) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => {
            Ok(warp::reply::with_status(warp::reply::json(&response), status))
        }
        Err(err) => {
            Err(warp::reject::custom(err))
        }
    }
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