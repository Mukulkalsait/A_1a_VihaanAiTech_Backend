use axum::{Router, serve::Listener};
use dotenvy::dotenv;
use std::net::{SocketAddr, TcpListener};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello Axum 0.8.8 " }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Service Running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
