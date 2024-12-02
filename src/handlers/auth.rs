use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::models::templates::{SignInTemplate, SignUpTemplate};

pub async fn sign_in_handler() -> impl IntoResponse {
    let html: String = SignInTemplate {}.render().unwrap();
    Html(html)
}

pub async fn sign_up_handler() -> impl IntoResponse {
    let html: String = SignUpTemplate {}.render().unwrap();
    Html(html)
}
