use bson::Document;
use mongodb::{Client, Collection, options::ClientOptions};

pub async fn get_connection() -> Client {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.max_pool_size = Some(10);
    let client = Client::with_options(client_options).unwrap();

    client
}

pub fn get_users_collection(mongo: &Client) -> Collection<Document> {
    mongo
        .database("hello_rocket")
        .collection::<Document>("users")
}