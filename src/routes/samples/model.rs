use actix_web::web;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Sample {
    pub id: Uuid,
    pub sample_name: String,
    pub sample_source: String,
    pub collection_dtm: DateTime<Utc>,
    pub received_dtm: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateSample {
    pub sample_name: String,
    pub sample_source: String,
    pub collection_dtm: DateTime<Utc>,
    pub received_dtm: DateTime<Utc>,
}

impl From<web::Json<CreateSample>> for CreateSample {
    fn from(new_study: web::Json<CreateSample>) -> Self {
        CreateSample {
            sample_name: new_study.sample_name.clone(),
            sample_source: new_study.sample_source.clone(),
            collection_dtm: new_study.collection_dtm.clone(),
            received_dtm: new_study.received_dtm.clone(),
        }
    }
}