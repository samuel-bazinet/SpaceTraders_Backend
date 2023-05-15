use std::error::Error;

use backend::{constants, requests};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let agent_response = requests::get_agent(&client).await?;
    println!("{}", agent_response);
    let agent_response = requests::get_location(&client, String::from(constants::HOME_SYSTEM), String::from(constants::HOME_SYMBOL)).await?;
    println!("{}", agent_response);
    Ok(())
}
