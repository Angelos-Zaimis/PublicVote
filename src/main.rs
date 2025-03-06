use std::sync::{Arc, Mutex};
use crate::api::voting_service::VotingService;
use crate::blockchain::voting_state::VotingState;

mod blockchain;
mod api;


#[tokio::main]
async fn main() {

    api::web_server::start_webserver().await;
}
