use std::collections::HashMap;

use warp::{Filter, http};

use domain::entities::{Id, Order, Store};

use crate::contracts::CreateOrderRequest;
use crate::decodes::decode_create_order_request_from_json;

mod contracts;
mod decodes;
mod domain;

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("orders"))
        .and(warp::path::end())
        .and(decode_create_order_request_from_json())
        .and(store_filter.clone())
        .and_then(add_grocery_list_item);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("orders"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);

    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("orders"))
        .and(warp::path::end())
        .and(order_from_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("orders"))
        .and(warp::path::end())
        .and(id_json())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);

    let routes = add_items.or(get_items).or(delete_item).or(update_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn create_order_request_from_json() -> impl Filter<Extract = (CreateOrderRequest,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn order_from_json() -> impl Filter<Extract = (Order,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn id_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn add_grocery_list_item(
    create_order_request: CreateOrderRequest,
    store: Store) -> Result<impl warp::Reply, warp::Rejection> {

    let order = create_order_request.to_order();

    store.grocery_list.write().insert(order.id.clone(), order);

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

async fn get_grocery_list(
    store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = Vec::new();
    let r = store.grocery_list.read();


    for (key,value) in r.iter() {
        result.push(value);
    }

    Ok(warp::reply::json(
        &result
    ))
}

async fn delete_grocery_list_item(
    id: Id,
    store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().remove(&id.name);


    Ok(warp::reply::with_status(
        "Removed item from grocery list",
        http::StatusCode::OK,
    ))
}

async fn update_grocery_list(
    item: Order,
    store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(item.id.clone(), item);


    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}
