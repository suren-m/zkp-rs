use warp::Rejection;

pub mod challenge;
pub mod error;
pub mod handlers;

pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Result<T> = std::result::Result<T, error::Error>;
