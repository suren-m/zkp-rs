use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Message {
//     pub data: String,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub enum ResponseType {
//     Success,
//     Challenge,
//     Failure,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct ServerResponse<T> {
//     pub response_type: ResponseType,
//     pub data: T,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerResponse {
    Success,
    Challenge(u128),
    Failure(String),
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct ServerResponse<T> {
//     pub response_type: ResponseType,
//     pub data: T,
// }
