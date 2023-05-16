use crate::responses::{
    response_fields::{AgentResponseFields, LocationResponseFields},
    JsonResponse,
};
use axum::{extract::Path, http::StatusCode, Json};

mod game_requests;

pub async fn get_agent() -> (StatusCode, Json<JsonResponse<AgentResponseFields>>) {
    let client = reqwest::Client::new();

    let game_response = game_requests::get_agent(&client).await;

    if let Ok(response) = game_response {
        (StatusCode::OK, Json(response))
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(JsonResponse::default()),
        )
    }
}

pub async fn get_location(
    Path((system, symbol)): Path<(String, String)>,
) -> (StatusCode, Json<JsonResponse<LocationResponseFields>>) {
    let client = reqwest::Client::new();

    let game_response = game_requests::get_location(&client, system, symbol).await;

    if let Ok(response) = game_response {
        (StatusCode::OK, Json(response))
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(JsonResponse::default()),
        )
    }
}
