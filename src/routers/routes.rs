use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

use crate::handlers::auth::{sign_in_handler, sign_up_handler};
use crate::handlers::public::home;
use crate::handlers::todos::{create_toto_handler, todos_handler};

pub fn router() -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/todos", get(todos_handler))
        .route("/create_todo", get(create_toto_handler))
        .route("/sign_in", get(sign_in_handler))
        .route("/sign_up", get(sign_up_handler))
        .nest_service("/static", server_dir);
    app
}
