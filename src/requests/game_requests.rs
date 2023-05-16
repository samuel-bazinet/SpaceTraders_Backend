use crate::{
    constants::TOKEN,
    responses::{
        response_fields::{AgentResponseFields, LocationResponseFields},
        JsonResponse,
    },
};
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};

pub async fn get_agent(
    client: &Client,
) -> Result<JsonResponse<AgentResponseFields>, reqwest::Error> {
    client
        .get("https://api.spacetraders.io/v2/my/agent")
        .header(AUTHORIZATION, format!("Bearer {TOKEN}"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<JsonResponse<AgentResponseFields>>()
        .await
}

pub async fn get_location(
    client: &Client,
    system: String,
    symbol: String,
) -> Result<JsonResponse<LocationResponseFields>, reqwest::Error> {
    client
        .get(format!(
            "https://api.spacetraders.io/v2/systems/{}/waypoints/{}",
            system, symbol
        ))
        .header(AUTHORIZATION, format!("Bearer {TOKEN}"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<JsonResponse<LocationResponseFields>>()
        .await
}
