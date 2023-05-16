use axum::{routing::get, Router};

use std::{error::Error, net::SocketAddr};

use backend::requests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/agent", get(requests::get_agent))
        .route("/location/:system/:symbol", get(requests::get_location));
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
