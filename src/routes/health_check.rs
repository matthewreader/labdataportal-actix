use actix_web::HttpResponse;
use serde_json::json;
use chrono;

pub async fn health_check() -> HttpResponse {
    let dtm = chrono::offset::Utc::now();
    HttpResponse::Ok()
        .json(json!({"status": "success",
        "time": &dtm}))
}
