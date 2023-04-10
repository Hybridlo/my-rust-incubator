mod db;
mod articles;

use actix_web::{App, web, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() {
    dotenv().expect(".env to be found");

    HttpServer::new(|| {
        App::new()
            .route("/articles", web::get().to(articles::get_all_articles))
            .service(
                web::scope("/article")
                    .route("/{id}", web::get().to(articles::get_article))
                    .route("", web::post().to(articles::new_article))
                    .route("/{id}", web::delete().to(articles::delete_article))
            )
    })
    .bind(("127.0.0.1", 8080))
    .expect("Server to launch")
    .run()
    .await
    .expect("Server to launch");
}
