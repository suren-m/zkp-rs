use crate::challenge::*;
use serde_json::json;
use zkp_common::dto::*;

use warp::{reply, Reply};

use crate::WebResult;

pub async fn register_handler(register_req: RegisterRequest) -> WebResult<impl Reply> {
    println!("register request");

    Ok(reply())
}

pub async fn create_auth_handler(create_auth_req: CreateAuthRequest) -> WebResult<impl Reply> {
    println!("create auth request");

    let challenge = Challenge::new();
    Ok(reply::json(&"test"))
}

pub async fn verify_auth_handler(verify_auth_req: VerifyAuthRequest) -> WebResult<impl Reply> {
    println!("verify auth request");
    let result = true;

    Ok(reply::json(&result))
}
