use axum::{
    routing::get, Router
};
use std::{error::Error, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

use backend::requests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/agent", get(requests::get_agent))
        .route("/location/:system/:symbol", get(requests::get_location))
        .layer(cors);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
