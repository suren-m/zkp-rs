use core::time;
use env_logger::Env;
use log::{error, info};
use std::io::Error;
use std::net::TcpStream;
use std::process::exit;
use std::thread;
use zkp_client::seed::Seed;
use zkp_client::user::{self};
use zkp_client::{
    check_status, create_auth_request, create_register_commits, prove_auth,
    register_user_with_server,
};
use zkp_common::request_dto::Answer;
use zkp_common::response_dto::ServerResponse;

fn main() -> Result<(), Error> {
    init_logger();
    let socket = "localhost:8080";
    let user_info = user::get_user_info_from_env_vars();
    if user_info.is_err() {
        print_errors(user_info.err().unwrap());
        exit(-1);
    }
    let user_info = user_info.unwrap();

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
            info!("Challenge receieved. Continuing with Auth Request");
            dbg!(c);
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

fn init_logger() {
    // set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.8.2/env_logger/
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
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

fn print_errors(errors: Vec<Error>) {
    info!("Following exceptions occurred when attempting to initialzie the client");
    for error in errors {
        error!("{}", error.to_string());
    }
}
