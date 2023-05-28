#[macro_use]
extern crate serde_derive;

mod blockchain_info;
mod blockchain_address;
mod blockchain_status;
mod blockchain_transactions;

fn main() {
    blockchain_info::blockchain_status_request();
}
