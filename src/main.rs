use std::{env, sync::Mutex};

use bitcoincore_rpc::{Auth, Client, RpcApi};
use chrono::Duration;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String = env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

// TODO: Task 1
fn time_to_mine(block_height: u64) -> Duration {
    let rpc_client: &Client = &*RPC_CLIENT;
    rpc_client.get_block_hash(234);
    todo!()
}

// TODO: Task 2
fn number_of_transactions(block_height: u64) -> u16 {
    todo!()
}

fn main() {
    // you can use rpc_client here as if it was a global variable
    // println!("{:?}", res);
    let rpc_client = &*RPC_CLIENT;
    rpc_client.get_block_hash(234);
}
