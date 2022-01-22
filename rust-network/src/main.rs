use std::{error::Error, };
use reqwest;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.op/";
    let response: reqwest::Response = reqwest::get(url).await?;

    let content = response.text().await?;
    println!("{}", content);

    Ok(())
}
