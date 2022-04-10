use warp::Filter;
use zkp_server::error;
use zkp_server::handlers::*;

#[tokio::main]
async fn main() {
    let register_route = warp::path!("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(register_handler);

    let create_auth_challenge_route = warp::path!("create_authentication_challenge")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_auth_handler);

    let verify_auth_route = warp::path!("verify_authentication")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(verify_auth_handler);

    let routes = register_route
        .or(create_auth_challenge_route)
        .or(verify_auth_route)
        .recover(error::handle_rejection);

    println!("listening on port 8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
