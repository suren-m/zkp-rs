pub mod challenge;
pub mod handlers;
pub mod server_error;
pub mod session_store;

// smaller value chosen for convenience
// to avoid negative values during "s = k - c.x" calculation
pub const MAX_CHALLENGE_VAL: u32 = 5;
