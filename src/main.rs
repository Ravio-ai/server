use server::{self, routers::routes};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listen: TcpListener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let app = routes::router();

    println!("Server is runnign on port :8080");
    axum::serve(listen, app).await.unwrap();
}
