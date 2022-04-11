use env_logger::Env;
use log::{error, info};
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::exit;
use zkp_client::user;
use zkp_common::dto::{Request, RequestType, User};

fn init_logger() {
    // set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.8.2/env_logger/
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

fn main() {
    init_logger();
    let user_info = user::get_user_info_from_env_vars();
    if user_info.is_err() {
        info!("Following exceptions occurred when attempting to parse environment variables");
        for error in user_info.err().unwrap() {
            error!("{}", error.to_string());
        }
        exit(-1);
    }

    let socket = "localhost:8080";
    match TcpStream::connect(socket) {
        Ok(mut stream) => {
            let msg = b"Get some data";
            let test = User {
                id: 1,
                name: "TestUser".to_string(),
            };
            let register_req = Request {
                request_type: RequestType::Authentication,
                user: test,
            };
            let j = serde_json::to_string(&register_req).unwrap();
            stream.write(j.as_bytes()).unwrap();

            // below will work but sending anything other than json string will result in connection reset
            //let test = json!("{ \"id\" : 1 }");
            //stream.write(test.to_string().as_bytes()).unwrap();

            let br = BufReader::new(stream);
            println!("printing buf reader contents");
            let res: Result<zkp_common::dto::Response, serde_json::Error> =
                serde_json::from_reader(br);
            dbg!(res);
            println!("done");
        }
        Err(_) => println!("can't connect"),
    }
}
