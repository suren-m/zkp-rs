use serde::{Deserialize, Serialize};

pub type Username = String;
pub type Answer = u32;

/// r1, r2 from client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commits {
    pub r1: u128,
    pub r2: u128,
    pub y1: u128,
    pub y2: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientRequest {
    Register(Username, Commits), // server will respond with a success or failure
    Authenticate(Username),      // server will respond with a challenge to this
    ProveAuthentication(Username, Answer), // server will respond with a sucess or failure
    CheckStatus(Username), // server will respond whether logged in or not. (called after Login flow)
}
