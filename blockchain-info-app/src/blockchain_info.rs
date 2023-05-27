use dotrnv;
use reqwest;
use tokio;
use serde::Result;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

pub fn send_request(url: &str) -> String {

    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", "01cc171a-383b-4584-ab5a-14159b5dc100")

}