mod models;
mod handlers;

use actix_web::{web, App, HttpServer};

const ADDRESS_SERVER: &str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
            App::new()
                .service(
                    web::scope("/api")
                        .route("/champions",
                            web::post().to(handlers::create_champion)),
                )
    })
    .bind(ADDRESS_SERVER)?
    .run()
    .await

}