use actix_web::HttpResponse;
use serde_json::json;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use crate::routes::{CreateSample, Sample};
use chrono::Utc;

pub async fn post_new_sample_db(pool: &PgPool, new_sample: CreateSample) -> HttpResponse {
    let query_result = sqlx::query_as!(
        Sample,
        "INSERT INTO samples (id, sample_name, sample_source, collection_dtm, received_dtm, last_updated)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, sample_name, sample_source, collection_dtm, received_dtm, last_updated",
        Uuid::new_v4(),
        new_sample.sample_name,
        new_sample.sample_source,
        new_sample.collection_dtm,
        new_sample.received_dtm,
        Utc::now()
    )
        .fetch_one(pool)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    return match query_result {
        Ok(sample) => {
            let response = json!({"status": "success","data": serde_json::json!({
                "sample": &sample
            })});

            HttpResponse::Created().json(response)
        }

        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": format!("{:?}", e)})),
    };
}