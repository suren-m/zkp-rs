use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, io::Error, io::ErrorKind};
use zkp_common::request_dto::{Commits, Username};

use crate::server_error::ServerError;

pub struct User {
    pub username: String,
    pub commits: Commits,
    pub is_verified: bool,
    pub last_verified: Option<Utc>,
    pub last_login: Option<Utc>,
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
            last_verified: None,
            last_login: None,
        };

        self.users.insert(username, new_user_session);
    }
}
