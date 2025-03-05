use serde::Deserialize;

#[derive(Deserialize)]
pub struct VoteRequestDTO {
    pub candidate: String
}