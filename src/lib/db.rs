use mongodb::{options::ClientOptions, Client};

pub async fn get_connection() -> Client {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    client
}
