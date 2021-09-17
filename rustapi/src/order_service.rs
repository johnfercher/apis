use crate::order_repository::OrderRepository;
use serde::Serialize;
use crate::errors::AppError;
use crate::models::Order;
use crate::IdResponse;

pub async fn add_order(
    db_manager: OrderRepository,
    order: Order,
) -> Result<impl warp::Reply, warp::Rejection> {

    let id_response = db_manager.create_order(order).map(|_|
        { IdResponse::new(String::from("blabla")) }
    );

    respond(id_response, warp::http::StatusCode::CREATED)
}

pub async fn list_orders(
    db_manager: OrderRepository,
) -> Result<impl warp::Reply, warp::Rejection> {

    let result = db_manager.list_orders();

    respond(result, warp::http::StatusCode::OK)
}

pub async fn update(
    order_id: String,
    db_manager: OrderRepository,
    order: Order,
) -> Result<impl warp::Reply, warp::Rejection> {

    let id_response = db_manager.update_orders(order).map(|_|
        { IdResponse::new(order_id) }
    );

    respond(id_response, warp::http::StatusCode::OK)
}

pub async fn delete_order(
    order_id: String,
    db_manager: OrderRepository,
) -> Result<impl warp::Reply, warp::Rejection> {

    let result = db_manager.delete_order(order_id.clone()).map(|_|
        { IdResponse::new(order_id) }
    );

    respond(result, warp::http::StatusCode::OK)
}

pub async fn get_order_by_id(
    order_id: String,
    db_manager: OrderRepository,
) -> Result<impl warp::Reply, warp::Rejection> {

    let result = db_manager.get_order_by_id(order_id);

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