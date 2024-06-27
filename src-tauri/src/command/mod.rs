use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
}

impl ApiError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<T: std::error::Error> From<T> for ApiError {
    fn from(error: T) -> ApiError {
        ApiError {
            message: format!("{}", error),
        }
    }
}

#[get("/misc/reqs_markdown")]
async fn misc_get_reqs_markdown() -> Result<impl Responder, ApiError> {
    let response = reqwest::get("gitlab page here").await?;
    let text = response.text().await?;
    Ok(web::Json(text))
}

#[get("/misc/wiki_page/{page}")]
async fn misc_get_wiki_page(path: web::Path<String>) -> Result<impl Responder, ApiError> {
    let page = path.into_inner();
    let response = reqwest::get(format!(
        "Gitlab page here/{}",
        page
    ))
    .await?;
    let text = response.text().await?;
    Ok(web::Json(text))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Need to figure out how to manage background processess
