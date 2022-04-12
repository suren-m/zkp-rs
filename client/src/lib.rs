use reqwest::{Error, StatusCode};
pub mod user;
use zkp_common::request_dto::*;

// pub async fn register(register_req: RegisterRequest) -> Result<StatusCode, Error> {
//     let request_url = "http://localhost:8000/register";

//     println!("{}", request_url);
//     dbg!(&register_req);

//     let client = reqwest::Client::new();
//     let response = client.post(request_url).json(&register_req).send().await?;
//     let status = response.status();
//     Ok(status)
// }
