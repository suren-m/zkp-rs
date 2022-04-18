use core::time;
use std::{
    io::{BufReader, ErrorKind, Error},
    net::TcpStream, process::exit, thread,
};

use auth::{create_register_commits, register_user_with_server};
use env_logger::Env;
use log::{error, info};
use reqwest::Client;
use seed::Seed;

use user::UserInfo;
use zkp_common::{
    request_dto::{Answer, ClientRequest, Commits, Username},
    response_dto::ServerResponse,
    write_and_flush_stream, G, H,
};

use crate::auth::{create_auth_request, prove_auth, check_status};

pub mod seed;
pub mod user;
pub mod auth;

/// Constraints
pub const MAX_USERNAME_LEN: usize = 50;

// for convenience
// min seed value set so that "s = k - c.x" is always positive.
pub const MIN_SEED_VAL: u32 = 100;

// max random 'k' value.
// smaller value chosen for convenience during g^K and h^K operations
pub const MAX_SEED_VAL: u32 = 125;

// for convenience when computing y1 = g^x and y2 = h^x
pub const MAX_SECRET_VAL: u32 = 25;

pub fn init_zkp_flow(user_info: UserInfo, socket: &str) -> Result<(), Error>{
        // random value used to calculate r1 and r2
    let k = Seed::new();
    let commits = create_register_commits(k, user_info.secret);

    // attempt registration
    let mut stream = connect(socket)?;
    let reg_res = register_user_with_server(&mut stream, user_info.username.to_owned(), commits)?;

    if let ServerResponse::Failure(msg) = &reg_res {
        error!("Server Error when attempting registration");
        error!("{}", msg);
        exit(-1);
    }

    if let ServerResponse::Success = &reg_res {
        // continue with auth flow
        info!("Registration Successful. Continuing with Auth Request");

        // expect challenge
        let mut stream = connect(socket)?;
        let auth_resp = create_auth_request(&mut stream, user_info.username.to_owned())?;

        if let ServerResponse::Challenge(c) = auth_resp {
            // solve challenge
            info!("Challenge ({}) receieved. Continuing with Auth Request", c);
            let answer: Answer = k.val - c * user_info.secret;
            let mut stream = connect(socket)?;
            let verify_resp = prove_auth(&mut stream, user_info.username.to_owned(), answer)?;
            if let ServerResponse::Success = verify_resp {
                info!("Login sucessful.");
                loop {
                    info!("checking status");
                    thread::sleep(time::Duration::from_secs(5));
                    let mut stream = connect(socket)?;
                    let status_resp = check_status(&mut stream, user_info.username.to_owned())?;
                    if let ServerResponse::Success = status_resp {
                        info!("still logged in");
                    } else {
                        error!("Not Logged in. Try authenticating again");
                    }
                }
            } else if let ServerResponse::Failure(msg) = verify_resp {
                error!("login failed {}", msg);
            }
        } else {
            error!("Server Error when attempting authentication request");
        }
    }

    Ok(())
}

fn connect(socket: &str) -> Result<TcpStream, Error> {
    match TcpStream::connect(socket) {
        Ok(stream) => Ok(stream),
        Err(e) => {
            println!("can't connect");
            return Err(e);
        }
    }
}


pub fn init_logger() {
    // set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.8.2/env_logger/
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

pub fn print_errors(errors: Vec<Error>) {
    info!("Following exceptions occurred when attempting to initialzie the client");
    for error in errors {
        error!("{}", error.to_string());
    }
}
