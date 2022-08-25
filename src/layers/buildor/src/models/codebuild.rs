use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildInfo {
    pub uuid: String,
    pub build_number: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub current_phase: Option<String>,
    pub build_status: Option<String>,
}
