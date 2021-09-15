use actix_web::{web, App, HttpServer, HttpResponse};

async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("Should set 'DATABASE_URL'");
    println!("{}", database_url);  

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(get_health_status))
           // ^ Our new health route points to the get_health_status handler
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}