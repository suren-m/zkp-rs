use pretty_assertions::assert_eq;
use reqwest::Error;
use reqwest::StatusCode;
use zkp_common::request_dto::*;

// #[tokio::test]
// async fn register_should_return_200() -> Result<(), Error> {
//     let req = RegisterRequest {
//         user: User {
//             username: String::from("testuser"),
//         },
//         commits: Commits { r1: 1, r2: 2 },
//     };
//     let result = register(req).await?;
//     assert_eq!(result, StatusCode::OK);
//     Ok(())
// }
