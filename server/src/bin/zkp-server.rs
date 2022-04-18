use log::info;
use zkp_server::init_logger;
use zkp_server::init_session_store;

use std::env;
use std::io::Error;
use std::net::TcpListener;
use std::net::TcpStream;

use std::net::{Ipv4Addr, SocketAddrV4};
use zkp_common::request_dto::ClientRequest;
use zkp_common::response_dto::ServerResponse;
use zkp_common::write_and_flush_stream;

use zkp_server::session_store::SessionStore;

use serde::Deserialize;

use zkp_server::handlers::*;



fn main() -> Result<(), Error> {
    init_logger();
    let mut session_store = init_session_store();

    // 127.0.0.1 won't work in a containerized env
    let loopback = Ipv4Addr::new(0, 0, 0, 0);
    let port = env::var("APP_PORT").unwrap_or("9090".to_string());
    let port: u16 = port.parse::<u16>().unwrap();

    let socket = SocketAddrV4::new(loopback, port);
    let listener = TcpListener::bind(socket).unwrap();
    println!("listening on {}", socket);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut session_store)?
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, session_store: &mut SessionStore) -> Result<(), Error>{
    let mut de = serde_json::Deserializer::from_reader(&stream);

    let req = ClientRequest::deserialize(&mut de);

    if req.is_err() {
        info!("Invalid request");
        write_and_flush_stream(
            &mut stream,
            ServerResponse::Failure("Unable to deserialize request".to_string()),
        )       
    } else {
        let req = req?;
        handle_request(req, session_store, &mut stream)
    }
}
