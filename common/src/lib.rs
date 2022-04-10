pub mod dto;

// public info
// Used when client is making the register commit
// r1 = g^k, r2 = h^k
// where k is a random seed generated by client
pub const g: u128 = 10;
pub const h: u128 = 20;
