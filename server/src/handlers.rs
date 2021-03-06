use chrono::Utc;
use log::info;

use std::{
    io::{Error},
    net::TcpStream,
};
use zkp_common::{
    request_dto::ClientRequest, response_dto::ServerResponse, write_and_flush_stream, G, H,
};

use crate::{
    challenge::{Challenge},
    session_store::SessionStore,
};

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

            if let Some(user) = session_store.users.get_mut(&username) {
                let challenge = Challenge::new();
                user.challenge = Some(challenge);
                return write_and_flush_stream(stream, ServerResponse::Challenge(challenge.val));
            } else {
                return write_and_flush_stream(
                    stream,
                    ServerResponse::Failure(
                        "User does not exist. Registration required".to_string(),
                    ),
                );
            }
        }
        ClientRequest::ProveAuthentication(username, answer) => {
            info!("Verify Auth Request Received");
            if let Some(user) = session_store.users.get_mut(&username) {
                dbg!(&user);
                if user.challenge.is_none() {
                    return write_and_flush_stream(
                        stream,
                        ServerResponse::Failure("challenge expired. Retry login".to_owned()),
                    );
                }

                let r1 = G.pow(answer) * user.commits.y1.pow(user.challenge.unwrap().val);
                let r2 = H.pow(answer) * user.commits.y2.pow(user.challenge.unwrap().val);

                if r1 == user.commits.r1 && r2 == user.commits.r2 {
                    info!("verified user");
                    user.is_verified = true;
                    user.last_verified = Some(Utc::now());
                    return write_and_flush_stream(stream, ServerResponse::Success);
                } else {
                    return write_and_flush_stream(
                        stream,
                        ServerResponse::Failure("Login failed. Commits don't match".to_string()),
                    );
                }
            } else {
                return write_and_flush_stream(
                    stream,
                    ServerResponse::Failure(
                        "User does not exist. Registration required".to_string(),
                    ),
                );
            }
        }
        ClientRequest::CheckStatus(username) => {
            info!("Status Check Request Received");
            if let Some(user) = session_store.users.get_mut(&username) {
                if user.is_verified == true {
                    user.last_login = Some(Utc::now());
                    return write_and_flush_stream(stream, ServerResponse::Success);
                } else {
                    return write_and_flush_stream(
                        stream,
                        ServerResponse::Failure(
                            "User not verified. Try authentication again".to_string(),
                        ),
                    );
                }
            } else {
                return write_and_flush_stream(
                    stream,
                    ServerResponse::Failure("User not registered.".to_string()),
                );
            }
        }
    }
}
