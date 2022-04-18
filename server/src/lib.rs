use env_logger::Env;
use session_store::SessionStore;

pub mod challenge;
pub mod handlers;
pub mod session_store;

// smaller value chosen for convenience
// to avoid negative values during "s = k - c.x" calculation
pub const MAX_CHALLENGE_VAL: u32 = 4;


pub fn init_logger() {
    // set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.8.2/env_logger/
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

pub fn init_session_store() -> SessionStore {
    SessionStore::new()
}