use mongodb::Client;

pub mod db;
pub mod security;

#[derive(Debug)]
pub struct Storage {
    pub mongo: Client,
}