use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commits {
    pub r1: u128,
    pub r2: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub user: User,
    pub commits: Commits,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    pub use_registered_commits: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAuthRequest {
    user: User,
    auth_req: AuthenticationRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Answer {
    pub data: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifyAuthRequest {
    user: User,
    answer: Answer,
}
