use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use validator::Validate;

use crate::models::{
    auth_form_model::AuthFormModel,
    templates::{SignInTemplate, SignUpTemplate},
};

use super::helpers;

pub async fn sign_in_handler() -> impl IntoResponse {
    let html: String = SignInTemplate {}.render().unwrap();
    Html(html)
}

pub async fn sign_up_handler() -> impl IntoResponse {
    let html: String = SignUpTemplate {
        email: "",
        email_error: "",
        password_error: "",
    }
    .render()
    .unwrap();
    Html(html)
}
pub async fn post_sign_up_handler(Form(user_form): Form<AuthFormModel>) -> impl IntoResponse {
    match user_form.validate() {
        Ok(_) => Redirect::to("/").into_response(),
        Err(errs) => {
            let err = errs.to_string();

            let mut email_error: String = String::new();
            let mut password_error: String = String::new();

            helpers::extract_error(&err, |field, message| {
                if field == "email" {
                    email_error = message;
                } else if field == "password" {
                    password_error = message;
                }
            });
            let html_string = SignUpTemplate {
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
            }
            .render()
            .unwrap();

            let response = Html(html_string).into_response();

            (StatusCode::BAD_REQUEST, response).into_response()
        }
    }
}
