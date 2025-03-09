use axum::routing::post;
use axum::Router;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

use crate::api::handler::handle_create_vote;
use crate::api::voting_service::VotingService;
use crate::blockchain::voting_state::VotingState;

pub fn routers() -> Router {
    let voting_state = Arc::new(Mutex::new(VotingState::new()));
    let service = Arc::new(VotingService::new(voting_state));

    let app = Router::new()
        .route("/vote", post(handle_create_vote))
        .layer(CorsLayer::permissive())
        .with_state(service);

    app
}
