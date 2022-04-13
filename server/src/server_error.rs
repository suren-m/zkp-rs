


#[derive(Debug)]
pub struct ServerError {
    pub msg: String,
}

impl ServerError {
    fn new(msg: &str) -> ServerError {
        ServerError {
            msg: msg.to_string(),
        }
    }
}
