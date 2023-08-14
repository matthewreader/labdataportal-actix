use actix_web::{post, web, Responder};
use sqlx::PgPool;
use crate::routes::{CreateSample, post_new_sample_db};


#[post("/")]
pub async fn post_new_sample(
    new_sample: web::Json<CreateSample>,
    pool: web::Data<PgPool>
) -> impl Responder {
    post_new_sample_db(&pool,new_sample.into()).await
}
