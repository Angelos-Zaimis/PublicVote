use std::sync::{Arc, Mutex};
use crate::api::dtos::VoteRequestDTO;
use crate::blockchain::voting_state::Voting;

pub struct VotingService {
    state: Arc<Mutex<dyn Voting + Send + Sync>>, // Thread-safe trait object
}

impl VotingService {
    pub fn new(state: Arc<Mutex<dyn Voting + Send + Sync>>) -> Self {
        VotingService { state }
    }

    pub fn voter_has_voted(&self, voter_id: String) -> bool {
        let state = self.state.lock().unwrap();
        state.has_voted(&voter_id)
    }

    pub fn create_new_vote(&self, vote_request_dto: VoteRequestDTO){
        let mut state = self.state.lock().unwrap();
        state.record_new_vote(vote_request_dto)
    }
}
