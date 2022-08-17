use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct RequestError {
    pub code: String,
    pub message: String,
    pub details: String,
}
impl RequestError {
    pub fn new(code: String, message: String, details: String) -> Self {
        Self {
            code,
            message,
            details,
        }
    }

    pub fn internal() -> Self {
        Self {
            code: "ISE00".to_string(),
            message: "Internal Server Error".to_string(),
            details: "Something wrong happened, try again later".to_string(),
        }
    }
}
