use actix_web::{get, post, web, HttpRequest, HttpResponse};
use stripe::Webhook;

use crate::{AppSettings, EventService, HttpError};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/webhook")]
async fn webhook(
    request: HttpRequest,
    payload: web::Bytes,
    event_service: web::Data<EventService>,
    app_settings: web::Data<AppSettings>,
) -> Result<HttpResponse, HttpError> {
    let payload_string = match std::str::from_utf8(&payload) {
        Ok(s) => s.to_string(),
        Err(err) => {
            tracing::log::error!("{err}");
            return Ok(HttpResponse::Ok().finish());
        }
    };

    let signature = request
        .headers()
        .get("stripe-signature")
        .and_then(|s| s.to_str().ok())
        .ok_or_else(|| {
            HttpError::bad_request("no 'stripe-signature' header found")
        })?;

    let event = match Webhook::construct_event(
        &payload_string,
        signature,
        &app_settings.stripe_endpoint_secret,
    ) {
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
