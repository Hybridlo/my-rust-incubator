use actix_web::{error, web::{Json, self}, HttpResponse};
use serde::{Serialize, Deserialize};

use crate::db::connect_to_db;

#[derive(Deserialize)]
pub struct NewArticle {
    title: String,
    body: String,
    labels: Vec<String>
}

#[derive(Serialize)]
pub struct Article {
    id: i64,
    title: String,
    body: String,
    labels: Vec<String>
}

pub async fn get_all_articles() -> Result<Json<Vec<Article>>, error::Error> {
    let mut conn = connect_to_db().await.map_err(|_| error::ErrorInternalServerError(""))?;

    let res = sqlx::query!("SELECT * FROM articles").fetch_all(&mut conn).await.map_err(|_| error::ErrorInternalServerError(""))?;

    Ok(Json(res.into_iter().map(|value| {
        Article {
            id: value.id,
            title: value.title,
            body: value.body,
            labels: value.labels.split(",").map(|res| res.to_string()).collect()
        }
    }).collect()))
}

pub async fn get_article(article_id: web::Path<i64>) -> Result<Json<Article>, error::Error> {
    let mut conn = connect_to_db().await.map_err(|_| error::ErrorInternalServerError(""))?;
    let article_id = article_id.into_inner();

    let res = sqlx::query!("SELECT * FROM articles WHERE id = ?", article_id).fetch_one(&mut conn).await.map_err(|_| error::ErrorInternalServerError(""))?;
    
    Ok(Json(Article {
        id: res.id,
        title: res.title,
        body: res.body,
        labels: res.labels.split(",").map(|res| res.to_string()).collect()
    }))
}

pub async fn new_article(article: Json<NewArticle>) -> Result<HttpResponse, error::Error> {
    let mut conn = connect_to_db().await.map_err(|_| error::ErrorInternalServerError(""))?;
    let lables = article.labels.join(",");

    let article_id = sqlx::query!(
        "INSERT INTO articles (title, body, labels) VALUES (?, ?, ?) RETURNING id",
        article.title,
        article.body,
        lables
    ).fetch_one(&mut conn).await.map_err(|_| error::ErrorInternalServerError(""))?;

    Ok(HttpResponse::Ok().body(article_id.id.to_string()))
}

pub async fn delete_article(article_id: web::Path<i64>) -> Result<HttpResponse, error::Error> {
    let mut conn = connect_to_db().await.map_err(|_| error::ErrorInternalServerError(""))?;
    let article_id = article_id.into_inner();

    sqlx::query!("DELETE FROM articles WHERE id = ?", article_id).execute(&mut conn).await.map_err(|_| error::ErrorInternalServerError(""))?;

    Ok(HttpResponse::Ok().body(""))
}