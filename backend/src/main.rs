use axum::{
    routing::get,
    Json, Router, http::header::CONTENT_TYPE,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(
            tower_http::cors::CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
            .allow_headers([CONTENT_TYPE])
            .allow_methods([axum::http::Method::GET]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listen on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message { message: String::from("Hello World") })
}
