//! zkp_server includes modules and functions that `zkp_client` communicates with.
//! 
//! Functionalities such as:
//! * Serving requests on top of TCP protocol using JSON for data exchange and simplicity.
//! * Registering the user's commits using an in-memory session store
//! * Presenting Authentication challenge as part of the login flow
//! * Validating the answer from client before providing access

//! ### Constraints
//! Following Constraints are set for convenience 
//! ```rust
//! // set so that `s = k - c.x` calculation doesn't result in a negative value.
//! // see constraints for zkp_client and zkp_common for more details
//! pub const MAX_CHALLENGE_VAL: u32 = 4;
//! ```
//! 
//! ### Session Store
//! Below structure represents the in-memory session store used to store user sessions
//! ```rust
//! #[derive(Debug)]
//! pub struct User {
//!     pub username: String,
//!     pub commits: Commits,
//!     pub is_verified: bool,
//!     pub challenge: Option<Challenge>,
//!     pub last_verified: Option<DateTime<Utc>>,
//!     pub last_login: Option<DateTime<Utc>>,
//! }
//! 
//! pub struct SessionStore {
//!     pub users: HashMap<Username, User>,
//! }
//! ```
//! 
//! ### Performance Improvements
//! * Server performance can be further improved by adopting a `multi-threaded` model or by implementing `async/await` pattern using runtimes such as `Tokio`
//! * The Session storage would also need to be enhanced when using a `multi-threaded` model. 
//!     * Either using `Arc Mutex` for shared-state concurrency or using a dedicated, external key/value store for session management.
//! * `JSON` is used for simplicity, but using a binary based representation such as `protocol buffers` for data exchange can further increase serialization and deserialization performance.



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