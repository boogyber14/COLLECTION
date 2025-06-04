use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/users/1";

    let response = reqwest::get(url)
        .await?
        .json::<User>()
        .await?;

    println!("{:#?}", response);
    Ok(())
}
