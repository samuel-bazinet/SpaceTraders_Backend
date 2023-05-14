use std::error::Error;

use backend::requests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let post_response = requests::get_agent(&client).await?;

    println!("{}", post_response);
    Ok(())
}
