use serde::{Deserialize, Serialize};

pub type Username = String;

/// r1, r2 from client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commits {
    pub r1: u128,
    pub r2: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Answer(u128);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientRequest {
    Register(Username, Commits), // server will respond with a success or failure
    Authenticate,                // server will respond with a challenge to this
    ProveAuthentication(Answer), // server will respond with a sucess or failure
}
