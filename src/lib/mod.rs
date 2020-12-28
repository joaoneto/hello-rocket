use mongodb::Client;

pub mod db;

#[derive(Debug)]
pub struct Storage {
    pub mongo: Client,
}