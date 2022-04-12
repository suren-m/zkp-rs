use log::info;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Write},
    net::TcpStream,
};
use zkp_common::{
    request_dto::ClientRequest,
    response_dto::{Message, ResponseType, ServerResponse},
};

use crate::session_store::SessionStore;

pub fn handle_request(
    req: ClientRequest,
    session_store: &mut SessionStore,
    stream: &mut TcpStream,
) -> Result<(), Error> {
    match req {
        zkp_common::request_dto::ClientRequest::Register(username, commits) => {
            info!("Register Request Received");

            if username.len() > 50 {
                info!("username too long");
                let fail_resp = ServerResponse::<String> {
                    response_type: ResponseType::Failure,
                    data: String::from("invalid user"),
                };
                return write_and_flush_stream(stream, fail_resp);
            }

            if let Some(_) = session_store.users.get(&username) {
                info!("User already exists. Returning failure");
                let fail_resp = ServerResponse::<String> {
                    response_type: ResponseType::Failure,
                    data: String::from(
                        "username already exists. Login again or pick a different username",
                    ),
                };
                return write_and_flush_stream(stream, fail_resp);
            }

            session_store.register(username, commits);

            let sucess_resp = ServerResponse::<String> {
                response_type: ResponseType::Success,
                data: String::from("Registration Successful. Proceed with auth"),
            };
            return write_and_flush_stream(stream, sucess_resp);
        }
        zkp_common::request_dto::ClientRequest::Authenticate => Ok(()),
        zkp_common::request_dto::ClientRequest::ProveAuthentication(answer) => Ok(()),
    }
}

fn write_and_flush_stream<T: Serialize>(stream: &mut TcpStream, data: T) -> Result<(), Error> {
    let j = serde_json::to_string(&data);
    if j.is_ok() {
        let res = j.unwrap();
        info!("writing to response stream");
        dbg!(&res);
        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
        Ok(())
    } else {
        return Err(Error::new(ErrorKind::Other, j.err().unwrap()));
    }
}

pub fn handle_error(mut stream: TcpStream, users: &mut HashMap<i32, String>) {
    // create(users);

    // for i in 0..10 {
    //     update(users, i);
    // }
    // let mut users_vec: Vec<User> = Vec::new();
    // for (k, v) in users.iter().enumerate() {
    //     let user = User {
    //         id: k as i32,
    //         name: v.1.to_string(),
    //     };
    //     users_vec.push(user);
    // }
    // let resp = Response {
    //     response_type: ResponseType::Success,
    //     users: users_vec,
    // };
    // let j = serde_json::to_string(&resp).unwrap();
    // dbg!(&j);
    // stream.write(j.as_bytes()).unwrap();
    // stream.flush().unwrap();
}
