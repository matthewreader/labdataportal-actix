use crate::routes::samples::controller::*;
use actix_web::web;

pub fn study_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/samples")
        .service(post_new_sample);
    cfg.service(scope);
}