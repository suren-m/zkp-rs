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

use serde::Deserialize;
use serde::Serialize;
use zkp_common::dto::Request;
use zkp_common::dto::Response;
use zkp_common::dto::ResponseType;
use zkp_common::dto::User;
use zkp_server::error;
use zkp_server::handlers::*;

fn main() {
    let mut users: HashMap<i32, String> = HashMap::new();

    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 8080);
    let listener = TcpListener::bind(socket).unwrap();
    println!("listening on {}", socket);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut users);
    }
}

fn handle_connection(mut stream: TcpStream, users: &mut HashMap<i32, String>) {
    let mut de = serde_json::Deserializer::from_reader(&stream);

    let req = Request::deserialize(&mut de);

    if req.is_err() {
        println!("req error");
        handle_error(stream, users);
    } else {
        println!("req valid");
        handle_request(req.unwrap(), stream, users);
    }
}
