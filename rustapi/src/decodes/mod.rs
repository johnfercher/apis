use crate::contracts::OrderRequest;
use warp::Filter;
use crate::domain::entities::{Order, Id};

pub fn decode_order_request_from_json() -> impl Filter<Extract = (OrderRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}