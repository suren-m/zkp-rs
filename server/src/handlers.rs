use log::info;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Write},
    net::TcpStream,
};
use zkp_common::{
    request_dto::ClientRequest, response_dto::ServerResponse, write_and_flush_stream,
};

use crate::{challenge::Challenge, session_store::SessionStore};

pub fn handle_request(
    req: ClientRequest,
    session_store: &mut SessionStore,
    stream: &mut TcpStream,
) -> Result<(), Error> {
    match req {
        ClientRequest::Register(username, commits) => {
            info!("Register Request Received");

            if username.len() > 50 {
                info!("username too long");
                return write_and_flush_stream(
                    stream,
                    ServerResponse::Failure("username must be less than 50 characters".to_string()),
                );
            }

            if let Some(_) = session_store.users.get(&username) {
                info!("User already exists. Returning failure");
                return write_and_flush_stream(
                    stream,
                    ServerResponse::Failure(
                        "user already exists. Login again or pick a different username".to_string(),
                    ),
                );
            }

            session_store.register(username, commits);
            return write_and_flush_stream(stream, ServerResponse::Success);
        }
        ClientRequest::Authenticate(username) => {
            info!("Autentication Request Received. Responding with Challenge");
            let challenge = Challenge::new().val;
            return write_and_flush_stream(stream, ServerResponse::Challenge(challenge));
        }
        ClientRequest::ProveAuthentication(answer) => Ok(()),
        ClientRequest::CheckStatus => todo!(),
    }
}
