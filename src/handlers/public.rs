use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::models::templates::HomeTemplate;

pub async fn home() -> impl IntoResponse {
    let html: String = HomeTemplate {}.render().unwrap();
    Html(html)
}
