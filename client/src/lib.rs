use std::{
    io::{BufReader, ErrorKind},
    net::TcpStream,
};

use log::warn;
use seed::Seed;
use user::UserInfo;
use zkp_common::{
    request_dto::{ClientRequest, Commits, Username},
    response_dto::ServerResponse,
    write_and_flush_stream, G, H,
};

pub mod seed;
pub mod user;

pub fn create_register_commits(k: Seed) -> Commits {
    Commits {
        r1: G.pow(k.val),
        r2: H.pow(k.val),
    }
}

pub fn register_user_with_server(
    stream: &mut TcpStream,
    username: Username,
    commits: Commits,
) -> Result<ServerResponse, std::io::Error> {
    let req = ClientRequest::Register(username, commits);

    write_and_flush_stream(stream, req)?;

    let br = BufReader::new(stream);
    let res: Result<ServerResponse, serde_json::Error> = serde_json::from_reader(br);

    if res.is_err() {
        warn!("Deserialization Error");
        return Err(std::io::Error::new(ErrorKind::Other, res.err().unwrap()));
    }

    Ok(res.unwrap())
}

pub fn create_auth_request(
    stream: &mut TcpStream,
    username: Username,
) -> Result<ServerResponse, std::io::Error> {
    let req = ClientRequest::Authenticate(username);

    write_and_flush_stream(stream, req)?;

    let br = BufReader::new(stream);
    let res: Result<ServerResponse, serde_json::Error> = serde_json::from_reader(br);

    if res.is_err() {
        warn!("Deserialization Error");
        return Err(std::io::Error::new(ErrorKind::Other, res.err().unwrap()));
    }

    Ok(res.unwrap())
}
