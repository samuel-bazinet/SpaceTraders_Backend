use crate::{
    constants::TOKEN,
    responses::{response_fields::AgentResponseFields, JsonResponse},
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
