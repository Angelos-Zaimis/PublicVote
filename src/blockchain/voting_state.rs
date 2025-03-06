use std::collections::HashSet;
use std::sync::Arc;

pub trait Voting: Send + Sync {
    fn record_vote(&mut self, voter_id: String) -> bool;
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
    fn record_vote(&mut self, voter_id: String) -> bool {
        if self.has_voted(&voter_id) {
            return false;
        }
        self.voted_users.insert(voter_id);
        true
    }

    fn has_voted(&self, voter_id: &String) -> bool {
        self.voted_users.contains(voter_id)
    }
}
