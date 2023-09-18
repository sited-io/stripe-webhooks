use actix_web::{get, post, web, HttpResponse};

use crate::{EventService, HttpError};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/webhook")]
async fn webhook(
    payload: web::Bytes,
    event_service: web::Data<EventService>,
) -> Result<HttpResponse, HttpError> {
    let payload_string = match std::str::from_utf8(&payload) {
        Ok(s) => s.to_string(),
        Err(err) => {
            tracing::log::error!("{err}");
            return Ok(HttpResponse::Ok().finish());
        }
    };

    let event = match serde_json::from_str(&payload_string) {
        Ok(e) => e,
        Err(err) => {
            tracing::log::error!("{payload_string} {err}");
            return Ok(HttpResponse::Ok().finish());
        }
    };

    event_service.handle_event(event).await
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);

    cfg.service(webhook);
}
