use aws_sdk_codebuild::{model::Build, output::StartBuildOutput};
use aws_sdk_dynamodb::model::AttributeValue;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use super::common::AsDynamoDBAttributeValue;

#[derive(Debug, PartialEq)]
pub enum BuildPhase {
    Submitted,       // "SUBMITTED",
    Provisioning,    // "PROVISIONING",
    DownloadSource,  // "DOWNLOAD_SOURCE",
    Install,         // "INSTALL",
    PreBuild,        // "PRE_BUILD",
    Build,           // "BUILD",
    PostBuild,       // "POST_BUILD",
    UploadArtifacts, // "UPLOAD_ARTIFACTS",
    Finalizing,      // "FINALIZING",
    Unknown,         // "UNKNOWN", (custom value used when parsing from/to string/enum)
}
impl FromStr for BuildPhase {
    type Err = ();

    fn from_str(input: &str) -> Result<BuildPhase, ()> {
        match String::from(input) {
            submitted if submitted == BuildPhase::Submitted.to_string() => {
                Ok(BuildPhase::Submitted)
            }
            provisioning if provisioning == BuildPhase::Provisioning.to_string() => {
                Ok(BuildPhase::Provisioning)
            }
            download_source if download_source == BuildPhase::DownloadSource.to_string() => {
                Ok(BuildPhase::DownloadSource)
            }
            install if install == BuildPhase::Install.to_string() => Ok(BuildPhase::Install),
            pre_build if pre_build == BuildPhase::PreBuild.to_string() => Ok(BuildPhase::PreBuild),
            build if build == BuildPhase::Build.to_string() => Ok(BuildPhase::Build),
            post_build if post_build == BuildPhase::PostBuild.to_string() => {
                Ok(BuildPhase::PostBuild)
            }
            upload_artifacts if upload_artifacts == BuildPhase::UploadArtifacts.to_string() => {
                Ok(BuildPhase::UploadArtifacts)
            }
            finalizing if finalizing == BuildPhase::Finalizing.to_string() => {
                Ok(BuildPhase::Finalizing)
            }
            _ => Ok(BuildPhase::Unknown),
        }
    }
}
impl fmt::Display for BuildPhase {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildPhase::Submitted => fmt.write_str("SUBMITTED"),
            BuildPhase::Provisioning => fmt.write_str("PROVISIONING"),
            BuildPhase::DownloadSource => fmt.write_str("DOWNLOAD_SOURCE"),
            BuildPhase::Install => fmt.write_str("INSTALL"),
            BuildPhase::PreBuild => fmt.write_str("PRE_BUILD"),
            BuildPhase::Build => fmt.write_str("BUILD"),
            BuildPhase::PostBuild => fmt.write_str("POST_BUILD"),
            BuildPhase::UploadArtifacts => fmt.write_str("UPLOAD_ARTIFACTS"),
            BuildPhase::Finalizing => fmt.write_str("FINALIZING"),
            _ => fmt.write_str("UNKNOWN"),
        }
    }
}

pub enum BuildPhaseStatus {
    TimedOut,    // "TIMED_OUT",
    Stopped,     // "STOPPED",
    Failed,      // "FAILED",
    Succeeded,   // "SUCCEEDED",
    Fault,       // "FAULT",
    ClientError, // "CLIENT_ERROR",
    Unknown,     // "UNKNOWN", (custom value used when parsing from/to string/enum)
}
impl FromStr for BuildPhaseStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<BuildPhaseStatus, ()> {
        match String::from(input) {
            timed_out if timed_out == BuildPhaseStatus::TimedOut.to_string() => {
                Ok(BuildPhaseStatus::TimedOut)
            }
            stopped if stopped == BuildPhaseStatus::Stopped.to_string() => {
                Ok(BuildPhaseStatus::Stopped)
            }
            failed if failed == BuildPhaseStatus::Failed.to_string() => {
                Ok(BuildPhaseStatus::Failed)
            }
            succeeded if succeeded == BuildPhaseStatus::Succeeded.to_string() => {
                Ok(BuildPhaseStatus::Succeeded)
            }
            fault if fault == BuildPhaseStatus::Fault.to_string() => Ok(BuildPhaseStatus::Fault),
            client_error if client_error == BuildPhaseStatus::ClientError.to_string() => {
                Ok(BuildPhaseStatus::ClientError)
            }
            _ => Ok(BuildPhaseStatus::Unknown),
        }
    }
}
impl fmt::Display for BuildPhaseStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildPhaseStatus::TimedOut => fmt.write_str("TIMED_OUT"),
            BuildPhaseStatus::Stopped => fmt.write_str("STOPPED"),
            BuildPhaseStatus::Failed => fmt.write_str("FAILED"),
            BuildPhaseStatus::Succeeded => fmt.write_str("SUCCEEDED"),
            BuildPhaseStatus::Fault => fmt.write_str("FAULT"),
            BuildPhaseStatus::ClientError => fmt.write_str("CLIENT_ERROR"),
            BuildPhaseStatus::Unknown => fmt.write_str("UNKNOWN"),
        }
    }
}

pub enum BuildObject {
    Build(Build),
    Builds(Option<Vec<Build>>),
    StartBuildOutput(StartBuildOutput),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildInfo {
    pub uuid: String,
    #[serde(rename(serialize = "buildNumber"))]
    pub build_number: Option<i64>,
    #[serde(rename(serialize = "startTime"))]
    pub start_time: Option<i64>,
    #[serde(rename(serialize = "endTime"))]
    pub end_time: Option<i64>,
    #[serde(rename(serialize = "currentPhase"))]
    pub current_phase: Option<String>,
    #[serde(rename(serialize = "buildStatus"))]
    pub build_status: Option<String>,
}

impl AsDynamoDBAttributeValue for BuildInfo {
    fn as_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        map.insert("uuid".to_string(), AttributeValue::S(self.uuid.to_owned()));
        map.insert(
            "build_number".to_string(),
            AttributeValue::N(format!("{}", self.build_number.unwrap_or(0))),
        );
        map.insert(
            "start_time".to_string(),
            AttributeValue::N(format!("{}", self.start_time.unwrap_or(0))),
        );
        map.insert(
            "end_time".to_string(),
            AttributeValue::N(format!("{}", self.end_time.unwrap_or(0))),
        );
        map.insert(
            "current_phase".to_string(),
            AttributeValue::S(self.current_phase.to_owned().unwrap_or("-".to_string())),
        );
        map.insert(
            "build_status".to_string(),
            AttributeValue::S(self.build_status.to_owned().unwrap_or("-".to_string())),
        );

        map
    }

    fn as_attr(&self) -> AttributeValue {
        AttributeValue::M(self.as_hashmap())
    }
}
