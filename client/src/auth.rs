use std::{
    io::{BufReader, ErrorKind},
    net::TcpStream,
};

use log::warn;
use crate::seed::Seed;

use zkp_common::{
    request_dto::{Answer, ClientRequest, Commits, Username},
    response_dto::ServerResponse,
    write_and_flush_stream, G, H,
};

pub fn create_register_commits(k: Seed, secret: u32) -> Commits {
    Commits {
        r1: G.pow(k.val),
        r2: H.pow(k.val),
        y1: G.pow(secret),
        y2: H.pow(secret),
    }
}

pub fn register_user_with_server(
    stream: &mut TcpStream,
    username: Username,
    commits: Commits,
) -> Result<ServerResponse, std::io::Error> {    
    let req = ClientRequest::Register(username, commits);
    make_request(stream, req)
}

pub fn create_auth_request(
    stream: &mut TcpStream,
    username: Username,
) -> Result<ServerResponse, std::io::Error> {
    let req = ClientRequest::Authenticate(username);
    make_request(stream, req)
    
}

pub fn prove_auth(
    stream: &mut TcpStream,
    username: Username,
    answer: Answer,
) -> Result<ServerResponse, std::io::Error> {
    let req = ClientRequest::ProveAuthentication(username, answer);
    make_request(stream, req)
}

pub fn check_status(
    stream: &mut TcpStream,
    username: Username,
) -> Result<ServerResponse, std::io::Error> {
    let req = ClientRequest::CheckStatus(username);
    make_request(stream, req)
}

fn make_request(stream: &mut TcpStream, req: ClientRequest) -> Result<ServerResponse, std::io::Error> {
    write_and_flush_stream(stream, req)?;

    let br = BufReader::new(stream);
    let res: Result<ServerResponse, serde_json::Error> = serde_json::from_reader(br);

    if res.is_err() {
        warn!("Deserialization Error");
        return Err(std::io::Error::new(ErrorKind::Other, res.err().unwrap()));
    }

    Ok(res.unwrap())
}