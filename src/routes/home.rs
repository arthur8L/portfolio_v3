use actix_web::{http::header::ContentType, HttpResponse};
use askama::Template;

use crate::error::AppError;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    body: &'a str,
}

pub async fn home() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        HomeTemplate {
            body: "This is body from hom struct",
        }
        .render()
        .map_err(AppError::new)?,
    ))
}
