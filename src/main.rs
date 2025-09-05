use axum::{routing::get, Router};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn hello() -> axum::Json<Message> {
    axum::Json(Message {
        text: "Hello from Json".into(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
