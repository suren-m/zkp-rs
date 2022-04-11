use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use zkp_common::models::{Commits, User};

pub struct UserSession {
    pub username: String,
    pub commits: Commits,
    pub is_verified: bool,
    pub last_verified: Utc,
    pub last_login: Utc,
}

pub struct SessionStore {
    users: HashMap<User, UserSession>,
}

impl SessionStore {
    pub fn new() -> Self {
        return SessionStore {
            users: HashMap::new(),
        };
    }
}
