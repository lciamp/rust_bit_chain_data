#[macro_use]
extern crate serde_derive;

mod blockchain_info;
mod blockchain_status;
mod blockchain_address;
mod blockchain_transaction;

use {
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_address::BlockchainAddress,
    crate::blockchain_transaction::BlockchainTransaction,
    dotenv,
    std::io,
    std::{thread, time},
    time::Duration,
};

fn blockchain_info_app(address: &str){
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    println!("\n\nQuerying {} - chain: {}\n\n", &blockchain_status.blockbook.coin, &blockchain_status.backend.chain);
    
    let blockchain_address: BlockchainAddress = blockchain_info::blockchain_address_request(address);
    println!("\nAnalyzing transactions for Bitcoin address: {}", &blockchain_address.address);

    let sleep_time = Duration::from_millis(2500);
    thread::sleep(sleep_time);

    println!("\nYou have a total of {} transactions.", &blockchain_address.txids.len());

    println!("\nDo you want to query these transactions? (y/n)\n");

    let mut command = String::new();
    io::stdin().read_line(&mut command);

    if command.trim().eq("y") {
        println!("\nWe will look up the following transactions:\n");
        println!("{:#?}", &blockchain_address.txids);
        thread::sleep(sleep_time);

        let mut balance: i32 = 0;
        for tx_id in &blockchain_address.txids {
            let mut subtotal_vin: i32 = 0;
            let mut subtotal_vout: i32 = 0;

            //

            balance += &subtotal_vout - &subtotal_vin
        }
    }
}

fn main() {
    let entered_address = dotenv::var("WALLET").expect("Error reading env var.");
    blockchain_info_app(&entered_address);

}
