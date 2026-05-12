use crate::{
    redis::redis_manager::RedisManager,
    types::redis::outgoing_message_to_engine::{self, OutgoingMessageToEngine},
};
use actix_web::{
    HttpResponse, Responder, cookie::time::format_description::modifier::WeekNumber, web,
};
use diesel::sql_types::Decimal;
use serde::Serialize;
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/orders")
            .route("/open", web::get().to(get_open_orders))
            .route("/{order_id}", web::delete().to(cancel_order))
            .route("/create", web::post().to(create_order)),
    );
}

pub async fn get_open_orders(
    user_id: web::ReqData<String>,
    market: web::ReqData<String>,
) -> impl Responder {
    let redis_manager = RedisManager::get_instance().lock().unwrap();
    let message = OutgoingMessageToEngine::GetOpenOrders {
        user_id: user_id.to_string(),
        market: market.to_string(),
    };

    let repsonse = redis_manager
        .send_and_await(user_id.to_string(), message)
        .await;

    match repsonse {
        Ok(message) => HttpResponse::Ok().json(repsonse),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn create_order(
    user_id: web::ReqData<String>,
    body: web::Json<outgoing_message_to_engine::CreateOrderData>,
) -> impl Responder {
    let redis_manager = RedisManager::get_instance().lock().unwrap();
    let req_body = body.into_inner();

    let message = OutgoingMessageToEngine::CreateOrder {
        market: req_body.market,
        price: req_body.price,
        quantity: req_body.quantity,
        side: req_body.side,
    };

    let repsonse = redis_manager
        .send_and_await(user_id.to_string(), message)
        .await;

    match repsonse {
        Ok(message) => HttpResponse::Ok().json(repsonse),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn cancel_order(
    user_id: web::ReqData<String>,
    order_id: web::ReqData<String>,
) -> impl Responder {
    let redis_manager = RedisManager::get_instance().lock().unwrap();
    let message = OutgoingMessageToEngine::CancelOrder {
        order_id: order_id.to_string(),
        user_id: user_id.to_string(),
    };
    let repsonse = redis_manager
        .send_and_await(user_id.to_string(), message)
        .await;

    match repsonse {
        Ok(message) => HttpResponse::Ok().json(repsonse),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}
