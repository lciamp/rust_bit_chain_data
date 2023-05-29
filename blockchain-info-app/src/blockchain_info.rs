use dotenv;
use reqwest;
use tokio;
use serde_json::Result;
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_transaction::BlockchainTransaction;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {

    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", dotenv::var("API_KEY").expect("Could not find key: API_Key"))
        .send()
        .await
        .expect("Failed to get response.")
        .text() // string works for return value
        .await
        .expect("Failed to conver payload.")
}

pub fn blockchain_status_request() {
    let response = send_request(&HOST_ROOT);
    println!("{}", response);
}