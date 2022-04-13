use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, io::Error, io::ErrorKind};
use zkp_common::request_dto::{Commits, Username};

use crate::challenge::Challenge;
use crate::server_error::ServerError;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub commits: Commits,
    pub is_verified: bool,
    pub challenge: Option<Challenge>,
    pub last_verified: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
}

pub struct SessionStore {
    pub users: HashMap<Username, User>,
}

impl SessionStore {
    pub fn new() -> Self {
        return SessionStore {
            users: HashMap::new(),
        };
    }

    pub fn remove(&mut self, username: &Username) -> Option<User> {
        self.users.remove(username)
    }

    pub fn register(&mut self, username: Username, commits: Commits) {
        let new_user_session = User {
            username: username.clone(),
            commits: commits,
            is_verified: false,
            challenge: None,
            last_verified: None,
            last_login: None,
        };

        self.users.insert(username, new_user_session);
    }
}
