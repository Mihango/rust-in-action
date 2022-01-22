<<<<<<< HEAD
use std::{error::Error, };
use reqwest;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.op/";
    let response: reqwest::Response = reqwest::get(url).await?;

    let content = response.text().await?;
    println!("{}", content);

    Ok(())
=======
fn main() {
    println!("Hello, world!");
>>>>>>> 0e60756304c9a06731c322df002baf47667e5b24
}
