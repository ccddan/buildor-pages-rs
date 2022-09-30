use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use error_stack::{Context, Report};
use std::fmt;

#[derive(Debug)]
pub struct HandlerError {
    pub msg: String,
}

impl HandlerError {
    pub fn new(message: &str) -> Self {
        Self {
            msg: String::from(message),
        }
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Handler create error: {}", self.msg).as_str())
    }
}

impl Context for HandlerError {}

pub trait HandlerInit {
    fn new(table_name: String, client: Client) -> Self;
}

#[async_trait]
pub trait HandlerCreate<T, PC, CE> {
    /// T = Main handler type (Project, User, etc.)
    /// PC = Payload Create. Payload to create objects of type T.
    /// CE = Create error
    async fn create(&self, payload: PC) -> Result<T, Report<CE>>;
}

#[async_trait]
pub trait HandlerGet<T, GE> {
    /// T = Main handler type (Project, User, etc.)
    /// GE = Update error
    async fn get(&self, uuid: String) -> Result<Option<T>, Report<GE>>;
}

#[async_trait]
pub trait HandlerList<T, LE> {
    /// T = Main handler type (Project, User, etc.)
    /// LE = List error
    async fn list(&self) -> Result<Vec<T>, Report<LE>>;
}

#[async_trait]
pub trait HandlerUpdate<T, PU, UE> {
    /// T = Main handler type (Project, User, etc.)
    /// PU = Payload update. Payload to update object.
    /// UE = Update error
    async fn update(&self, uuid: String, payload: PU) -> Result<(), Report<UE>>;
}

#[async_trait]
pub trait HandlerDelete<T, DE> {
    /// T = Main handler type (Project, User, etc.)
    /// DE = Delete error
    async fn delete(&self, uuid: String) -> Result<bool, Report<DE>>;
}
