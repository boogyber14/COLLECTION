use reqwest::Client;
use tokio;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    // Create a reqwest Client
    let client = Client::new();

    // Fetch the HTML content of the webpage asynchronously
    let response = client.get(url).send().await?;

    // Extract the body of the response as a string
    let body = response.text().await?;

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the URL of the webpage to fetch
    let url = "https://www.facebook.com";

    // Fetch the HTML content of the webpage asynchronously
    let html_content = fetch_url(url).await?;

    println!("HTML content of {}: \n{}", url, html_content);

    Ok(())
}
