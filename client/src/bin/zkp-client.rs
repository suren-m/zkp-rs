use env_logger::Env;
use log::{error, info};
use std::net::TcpStream;
use std::process::exit;
use zkp_client::seed::Seed;
use zkp_client::user::{self};
use zkp_client::{create_auth_request, create_register_commits, register_user_with_server};
use zkp_common::response_dto::ServerResponse;

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
    let user_info = user_info.unwrap();

    let socket = "localhost:8080";
    let stream = TcpStream::connect(socket);
    if stream.is_err() {
        error!("Can't connect - {}", stream.err().unwrap().to_string());
        exit(-1);
    }

    // random value used to calculate r1 and r2
    let k = Seed::new();
    let commits = create_register_commits(k);

    // attempt registration
    let mut stream = stream.unwrap();
    let reg_res = register_user_with_server(&mut stream, user_info.username.to_owned(), commits);

    if reg_res.is_err() {
        error!(
            "Error when attempting to register: {}",
            &reg_res.err().unwrap().to_string()
        );
        exit(-1);
    }

    let resp = reg_res.unwrap();
    if let ServerResponse::Failure(msg) = &resp {
        error!("Server Error when attempting registration");
        error!("{}", msg);
        exit(-1);
    }

    if let ServerResponse::Success = &resp {
        // continue with auth flow
        info!("Registration Successful. Continuing with Auth Request");

        // expect challenge
        let mut new_stream = TcpStream::connect(socket).unwrap();
        let auth_resp = create_auth_request(&mut new_stream, user_info.username.to_owned());

        if auth_resp.is_err() {
            error!("{}", &auth_resp.err().unwrap().to_string());
            exit(-1);
        }

        if let ServerResponse::Challenge(c) = auth_resp.unwrap() {
            // solve challenge
            info!("Challenge receieved. Continuing with Auth Request");
            dbg!(c);
        } else {
            error!("Server dummy Error when attempting authentication request");
        }
    }
}

fn init_logger() {
    // set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.8.2/env_logger/
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}
