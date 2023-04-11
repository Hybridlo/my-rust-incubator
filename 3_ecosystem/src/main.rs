mod auth;
mod hashing;
mod schema;

use std::env::var;

use actix_web::{
    error::ErrorInternalServerError, guard, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use auth::get_user_from_request;
use dotenvy::dotenv;
use schema::{MutationRoot, QueryRoot, ServerSchema};
use sqlx::SqlitePool;

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/ws")
                .finish(),
        )
}

async fn index(
    schema: web::Data<ServerSchema>,
    db_pool: web::Data<SqlitePool>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> Result<GraphQLResponse, Error> {
    let mut request = gql_request.into_inner();

    let user = get_user_from_request(&req, &db_pool)
        .await
        .map_err(|_| ErrorInternalServerError(""))?;

    if let Some(user) = user {
        request = request.data(user);
    }
    request = request.data((**db_pool).clone());

    Ok(schema.execute(request).await.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("To find .env file");
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .limit_depth(3)
        .finish();

    let conn_string = var("DATABASE_URL").expect("To connect to the database");
    let db_pool = SqlitePool::connect(&conn_string)
        .await
        .expect("To connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::resource("/").guard(guard::Get()).to(graphiql))
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
