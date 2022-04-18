//! zkp_client includes modules and functions used to communicate with the zkp_server
//! These include functionalities for:
//! * Generating the commits `r1, r2` using secret `x` (set via env variables)
//! * Generating random `k` value and computing `y1` and `y2` by `g^K` and `h^K`
//! * zkp flow
//!     * create_commits
//!     * register_user_with_server
//!     * prove authenticity using `s = k - c.x` where c is the challenge from server
//!     * continue to communicate with server once logged in  
//! ### Constraints
//! Following Constraints are set for convenience during exponential operations
//! Example:
//! ```rust
//! pub const MAX_USERNAME_LEN: usize = 50;
//! // min 'k' value set so that "s = k - c.x" is always positive.
//! pub const MIN_SEED_VAL: u32 = 100;
//! // max 'k' value. used during g^K and h^K operations
//! pub const MAX_SEED_VAL: u32 = 125;
//! // used when computing y1 = g^x and y2 = h^x
//! pub const MAX_SECRET_VAL: u32 = 25;
//! ```

use core::time;
use std::{
    io::{Error},
    net::TcpStream, process::exit, thread,
};

use auth::{create_register_commits, register_user_with_server};
use env_logger::Env;
use log::{error, info};
use seed::Seed;

use user::UserInfo;
use zkp_common::{
    request_dto::{Answer},
    response_dto::ServerResponse,
};

use crate::auth::{create_auth_request, prove_auth, check_status};

pub mod seed;
pub mod user;
pub mod auth;


pub const MAX_USERNAME_LEN: usize = 50;

// min `K` value set so that "s = k - c.x" is always positive.
pub const MIN_SEED_VAL: u32 = 100;

// smaller value chosen for convenience during g^K and h^K operations
pub const MAX_SEED_VAL: u32 = 125;

// for convenience when computing y1 = g^x and y2 = h^x
pub const MAX_SECRET_VAL: u32 = 25;

/// * create_commits
/// * register_user_with_server
/// * prove authenticity using `s = k - c.x` where c is the challenge from server
/// * continue to communicate with server once logged in 
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

/// Uses `TCPStream::connect` to establish connection to server and returns a `TcpStream` instance if successful.
/// ### Example:
/// ```rust
/// let port = env::var("SERVER_PORT").unwrap_or("9090".to_string());
/// let server_loc = env::var("SERVER_ADDRESS").unwrap_or("localhost".to_string());
/// let socket = &format!("{}:{}", server_loc, port);
/// let mut stream = connect(socket)?;
/// ```
fn connect(socket: &str) -> Result<TcpStream, Error> {
    match TcpStream::connect(socket) {
        Ok(stream) => Ok(stream),
        Err(e) => {
            println!("can't connect");
            return Err(e);
        }
    }
}

/// Env Logger level `info`. Set `$RUST_LOG` env variable accordingly to change severity levels
pub fn init_logger() {   
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

/// Prints any errors that occur during env variables validation
pub fn print_errors(errors: Vec<Error>) {
    info!("Following exceptions occurred when attempting to initialzie the client");
    for error in errors {
        error!("{}", error.to_string());
    }
}
