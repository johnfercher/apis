use crate::contracts::CreateOrderRequest;
use warp::Filter;

pub fn decode_create_order_request_from_json() -> impl Filter<Extract = (CreateOrderRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}