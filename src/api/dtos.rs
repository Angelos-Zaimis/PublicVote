use serde::Deserialize;
use validator::{Validate};

#[derive(Deserialize, Debug, Validate)]
pub struct VoteRequestDTO {
    #[validate(length(min = 1, message = "Election ID cannot be empty"))]
    pub election_id: String,

    #[validate(length(min = 1, message = "Voter ID cannot be empty"))]
    pub voter_id: String,

    #[validate(length(min = 1, message = "Candidate ID cannot be empty"))]
    pub candidate_id: String,
}
