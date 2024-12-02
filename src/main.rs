use askama::Template;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listen: TcpListener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let app = Router::new()
        .route("/", get(home))
        .route("/todos", get(todos_handler))
        .route("/create_todo", get(create_toto_handler))
        .route("/sign_in", get(sign_in_handler))
        .route("/sign_up", get(sign_up_handler))
        .route("/server_error", get(server_error_handler));

    axum::serve(listen, app).await.unwrap();
}

async fn home() -> impl IntoResponse {
    let html: String = HomeTemplate {}.render().unwrap();
    Html(html)
}

async fn todos_handler() -> impl IntoResponse {
    let html: String = TodoTemplate {}.render().unwrap();
    Html(html)
}

async fn create_toto_handler() -> impl IntoResponse {
    let html: String = CreateTemplate {}.render().unwrap();
    Html(html)
}

async fn sign_in_handler() -> impl IntoResponse {
    let html: String = SignInTemplate {}.render().unwrap();
    Html(html)
}

async fn sign_up_handler() -> impl IntoResponse {
    let html: String = SignUpTemplate {}.render().unwrap();
    Html(html)
}

async fn server_error_handler() -> impl IntoResponse {
    let html: String = ServerErrorTemplate {}.render().unwrap();
    Html(html)
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate {}

#[derive(Template)]
#[template(path = "pages/todo.html")]
struct TodoTemplate {}

#[derive(Template)]
#[template(path = "pages/create.html")]
struct CreateTemplate {}

#[derive(Template)]
#[template(path = "pages/sign_in.html")]
struct SignInTemplate {}

#[derive(Template)]
#[template(path = "pages/sign_up.html")]
struct SignUpTemplate {}

#[derive(Template)]
#[template(path = "pages/server_error.html")]
struct ServerErrorTemplate {}
