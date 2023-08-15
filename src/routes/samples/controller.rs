use actix_web::{post, web, Responder, get};
use sqlx::PgPool;
use crate::routes::{CreateSample, get_all_samples_db, post_new_sample_db};


#[post("/")]
pub async fn post_new_sample(
    new_sample: web::Json<CreateSample>,
    pool: web::Data<PgPool>
) -> impl Responder {
    post_new_sample_db(&pool,new_sample.into()).await
}

#[get("/")]
pub async fn get_all_samples(
    pool: web::Data<PgPool>
) -> impl Responder {
    get_all_samples_db(&pool).await
}
