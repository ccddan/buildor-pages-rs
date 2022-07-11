use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uuid: String,
    pub fname: String,
    pub lname: String,
}
