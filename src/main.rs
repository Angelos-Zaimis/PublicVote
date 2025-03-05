use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use crate::blockchain::transaction::Transaction;

mod blockchain;
mod api;

#[tokio::main]
async fn main() {

    api::web_server::start_webserver().await;
}
