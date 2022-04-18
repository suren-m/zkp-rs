use core::time;
use std::{thread, io::Error};

use log::{info, error};
use zkp_client::{user::{UserInfo}, init_zkp_flow, auth::check_status, connect};
use zkp_common::response_dto::ServerResponse;

// callback upon login success - infinite loop
pub fn test_callback(socket: &str, user_info: &UserInfo) -> Result<(), Error>{
    for i in 1..3 {
        info!("checking status. iteration - {}", i);
        thread::sleep(time::Duration::from_secs(2));      
        let mut stream = connect(socket)?;
        let status_resp = check_status(&mut stream, user_info.username.to_owned())?;
        if let ServerResponse::Success = status_resp {
            info!("still logged in");
        } else {
            error!("Not Logged in. Try authenticating again");
        }
    }
    Ok(())
}

#[test]
fn test_zkp_flow() {
    let user_info = UserInfo {
        username: "flow_test_user".to_string(),
        secret: 10        
    };

    let socket = "localhost:9090";    
    let res = init_zkp_flow(user_info, socket, test_callback);    
    assert_eq!(res.unwrap(), ());
}