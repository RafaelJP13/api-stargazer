use actix_web::{web, App, HttpServer, Responder};

const ADDRESS_SERVER: &str = "127.0.0.1:8080";

async fn hello_world() -> impl Responder{
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(hello_world))
    })
    .bind(ADDRESS_SERVER)?
    .run()
    .await

}