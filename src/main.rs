use server::{self, logs::init, routers::routes};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let add: &str = "127.0.0.1:8082";

    let listen: TcpListener = TcpListener::bind(add).await.expect("Failed to bind server");

    init::logging();

    let app = routes::router();

    tracing::info!("Server is running on {}...", add);
    axum::serve(listen, app)
        .await
        .expect("Failed to start server")
}
