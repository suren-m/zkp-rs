use env_logger::Env;
use log::{error, info};
use std::collections::HashMap;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;
use std::{
    env,
    io::{BufRead, BufReader, Write},
};
use std::{
    fs::File,
    net::{Ipv4Addr, SocketAddrV4},
};
use zkp_common::request_dto::ClientRequest;
use zkp_server::session_store;
use zkp_server::session_store::SessionStore;

use serde::Deserialize;
use serde::Serialize;

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

    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 8080);
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
        println!("req error");
        //handle_error(stream, users);
    } else {
        println!("req valid");
        let req = req.unwrap();

        dbg!(&req);
        handle_request(req, session_store, &mut stream).unwrap();
    }
}
