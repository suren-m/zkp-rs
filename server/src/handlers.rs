use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::challenge::*;
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
pub fn handle_request(req: Request, mut stream: TcpStream, users: &mut HashMap<i32, String>) {
    match req.request_type {
        zkp_common::dto::RequestType::Register => {
            println!("Human says Register: {:?}", req.user);
            create(users);

            for i in 0..10 {
                update(users, i);
            }
            let mut users_vec: Vec<User> = Vec::new();
            for (k, v) in users.iter().enumerate() {
                let user = User {
                    id: k as i32,
                    name: v.1.to_string(),
                };
                users_vec.push(user);
            }
            let resp = Response {
                response_type: ResponseType::Success,
                users: users_vec,
            };
            let j = serde_json::to_string(&resp).unwrap();
            stream.write(j.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        zkp_common::dto::RequestType::Authentication => {
            println!("Human says Auth: {:?}", req.user);
            create(users);

            for i in 0..10 {
                update(users, i);
            }
            let mut users_vec: Vec<User> = Vec::new();
            for (k, v) in users.iter().enumerate() {
                let user = User {
                    id: k as i32,
                    name: v.1.to_string(),
                };
                users_vec.push(user);
            }
            let resp = Response {
                response_type: ResponseType::Success,
                users: users_vec,
            };
            let j = serde_json::to_string(&resp).unwrap();
            dbg!(&j);
            stream.write(j.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        zkp_common::dto::RequestType::ProveAuthentication => {
            println!("Human says Prove: {:?}", req.user);
            create(users);

            for i in 0..2 {
                update(users, i);
            }
            let mut users_vec: Vec<User> = Vec::new();
            for (k, v) in users.iter().enumerate() {
                let user = User {
                    id: k as i32,
                    name: v.1.to_string(),
                };
                users_vec.push(user);
            }
            let resp = Response {
                response_type: ResponseType::Success,
                users: users_vec,
            };
            let j = serde_json::to_string(&resp).unwrap();
            dbg!(&j);
            stream.write(j.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        zkp_common::dto::RequestType::VerifyAuthRequest => {
            println!("Human says Verify: {:?}", req.user);
            create(users);

            for i in 0..10 {
                update(users, i);
            }
            let mut users_vec: Vec<User> = Vec::new();
            for (k, v) in users.iter().enumerate() {
                let user = User {
                    id: k as i32,
                    name: v.1.to_string(),
                };
                users_vec.push(user);
            }
            let resp = Response {
                response_type: ResponseType::Success,
                users: users_vec,
            };
            let j = serde_json::to_string(&resp).unwrap();
            stream.write(j.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

pub fn create(users: &mut HashMap<i32, String>) {
    users.insert(1, "hello".to_string());
}
pub fn update(users: &mut HashMap<i32, String>, id: i32) {
    users.insert(id, "hello again".to_string());
}

pub fn handle_error(mut stream: TcpStream, users: &mut HashMap<i32, String>) {
    create(users);

    for i in 0..10 {
        update(users, i);
    }
    let mut users_vec: Vec<User> = Vec::new();
    for (k, v) in users.iter().enumerate() {
        let user = User {
            id: k as i32,
            name: v.1.to_string(),
        };
        users_vec.push(user);
    }
    let resp = Response {
        response_type: ResponseType::Success,
        users: users_vec,
    };
    let j = serde_json::to_string(&resp).unwrap();
    dbg!(&j);
    stream.write(j.as_bytes()).unwrap();
    stream.flush().unwrap();
}
