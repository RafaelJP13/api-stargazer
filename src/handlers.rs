use actix_web::{web, HttpResponse};

use crate::models::Champion;

pub async fn create_champion(champion: web::Json<Champion>) -> HttpResponse {
    HttpResponse::Ok().json(champion.into_inner())
}