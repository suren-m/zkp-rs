use env_logger::Env;
use log::info;

use std::env;
use std::net::TcpListener;
use std::net::TcpStream;

use std::net::{Ipv4Addr, SocketAddrV4};
use zkp_common::request_dto::ClientRequest;
use zkp_common::response_dto::ServerResponse;
use zkp_common::write_and_flush_stream;

use zkp_server::session_store::SessionStore;

use serde::Deserialize;

use zkp_server::handlers::*;

fn init_logger() {
    // set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.8.2/env_logger/
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

fn init_session_store() -> SessionStore {
    SessionStore::new()
}

fn main() {
    init_logger();
    let mut session_store = init_session_store();

    //127.0.0.1 won't work in containerized env
    let loopback = Ipv4Addr::new(0, 0, 0, 0);
    let port = env::var("APP_PORT").unwrap_or("9090".to_string());
    let port: u16 = port.parse::<u16>().unwrap();

    let socket = SocketAddrV4::new(loopback, port);
    let listener = TcpListener::bind(socket).unwrap();
    println!("listening on {}", socket);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut session_store);
    }
}

fn handle_connection(mut stream: TcpStream, session_store: &mut SessionStore) {
    let mut de = serde_json::Deserializer::from_reader(&stream);

    let req = ClientRequest::deserialize(&mut de);

    if req.is_err() {
        info!("Invalid request");
        write_and_flush_stream(
            &mut stream,
            ServerResponse::Failure("Unable to deserialize request".to_string()),
        )
        .unwrap();
    } else {
        let req = req.unwrap();
        handle_request(req, session_store, &mut stream).unwrap();
    }
}
