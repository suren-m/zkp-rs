use log::{info};

use std::io::Error;
use std::process::exit;
use std::{env};

use zkp_client::user::{self};
use zkp_client::{
    print_errors, init_logger, init_zkp_flow,
};

fn main() -> Result<(), Error> {
    init_logger();
    let port = env::var("SERVER_PORT").unwrap_or("9090".to_string());
    let server_loc = env::var("SERVER_ADDRESS").unwrap_or("localhost".to_string());
    let socket = &format!("{}:{}", server_loc, port);
    info!("Server Address: {}", socket);

    let user_info = user::get_user_info_from_env_vars();
    if user_info.is_err() {
        print_errors(user_info.err().unwrap());
        exit(-1);
    }
    let user_info = user_info.unwrap();
    
    init_zkp_flow(user_info, socket)
}


