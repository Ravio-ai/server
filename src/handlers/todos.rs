use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::models::templates::{CreateTemplate, TodoTemplate};

pub async fn todos_handler() -> impl IntoResponse {
    let html: String = TodoTemplate {}.render().unwrap();
    Html(html)
}

pub async fn create_toto_handler() -> impl IntoResponse {
    let html: String = CreateTemplate {}.render().unwrap();
    Html(html)
}
