
use reqwest;
 
#[tokio::main]
async fn main() {
    // using library to make http request
    make_http_request_with_reqwest().await;
}

async fn make_http_request_with_reqwest()  {
    let url = "http://www.rustinaction.com/";
    let response: reqwest::Response = reqwest::get(url).await.unwrap();

    let content = response.text().await.unwrap();
    println!("{}", content);
}