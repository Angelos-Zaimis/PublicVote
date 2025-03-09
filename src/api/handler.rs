use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use crate::api::dtos::VoteRequestDTO;
use validator::{Validate};
use crate::api::voting_service::VotingService;

pub async fn handle_create_vote(
    State(service): State<Arc<VotingService>>,
    Json(payload): Json<VoteRequestDTO>,
) -> Json<String> {
    if service.voter_has_voted(payload.voter_id.clone()) {
        return Json("You have already voted.".to_string());
    }

    service.create_new_vote(payload);
    Json("You voted successfully!".to_string())
}

fn validate_payload(payload: &VoteRequestDTO) {
    payload.validate().expect("VoteRequestDTO failed");
}
