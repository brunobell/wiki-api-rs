use actix_web::HttpResponse;

use crate::models::heartbeat::HeartbeatResponse;

pub fn heartbeat_handle() -> HttpResponse {
    HttpResponse::Ok().json(HeartbeatResponse {
        status: 200,
        message: "ok".into(),
        version: std::env::var("API_VERSION").unwrap(),
    })
}
