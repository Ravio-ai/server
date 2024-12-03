use std::time::Duration;

use axum::body::Body;
use axum::http::{Request, Response};

use axum::routing::get;
use axum::Router;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Span;

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
        .nest_service("/static", server_dir)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
                .on_request(on_request)
                .on_response(on_response)
                .on_failure(on_failure),
        );
    app
}

fn on_request(request: &Request<Body>, _: &Span) {
    tracing::info!(
        "Request started: method {} path {}",
        request.method(),
        request.uri().path()
    );
}
fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!(
        "<- Response generated: status {} in {:?}",
        response.status(),
        latency
    )
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("-x- Request failed: {:?} after {:?}", error, latency)
}
