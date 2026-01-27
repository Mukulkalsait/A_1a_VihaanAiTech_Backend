use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

// use dotenvy::dotenv;
// use std::net::{SocketAddr, TcpListener};
// use tracing_subscriber;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello Axum 0.8.8 " }));

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.expect("Fail to bind address");

    println!("🚀 Service Running on http://{}", addr);

    axum::serve(listener, app).await.expect("Server Failed");
}
