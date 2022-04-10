use env_logger::Env;
use log::{error, info};
use reqwest::Error;
use std::process::exit;
use zkp_client::user;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_logger();
    let user_info = user::get_user_info_from_env_vars();
    if user_info.is_err() {
        info!("Following exceptions occurred when attempting to parse environment variables");
        for error in user_info.err().unwrap() {
            error!("{}", error.to_string());
        }
        exit(-1);
    }

    Ok(())
}

fn init_logger() {
    // set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.8.2/env_logger/
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}
