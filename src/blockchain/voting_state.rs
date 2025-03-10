use std::collections::HashSet;
use std::sync::Arc;
use crate::api::dtos::VoteRequestDTO;
use crate::blockchain::transactions::transaction::VoteTransaction;

pub trait Voting: Send + Sync {
    fn record_new_vote(&mut self, vote_request_dto: VoteRequestDTO);
    fn has_voted(&self, voter_id: &String) -> bool;
}

pub struct VotingState {
    voted_users: HashSet<String>,
}

impl VotingState {
    pub fn new() -> Self {
        VotingState {
            voted_users: HashSet::new(),
        }
    }
}

impl Voting for VotingState {
    fn record_new_vote(&mut self, vote_request_dto: VoteRequestDTO) {
        let new_transaction = VoteTransaction::new(vote_request_dto.voter_public_key, );


    }

    fn has_voted(&self, voter_id: &String) -> bool {
        self.voted_users.contains(voter_id)
    }
}
