use aws_sdk_dynamodb::model::AttributeValue;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::AsDynamoDBAttributeValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands {
    #[serde(rename(serialize = "preBuild"))]
    pub pre_build: Vec<String>,
    pub build: Vec<String>,
}
impl Commands {
    pub fn new_defaults() -> Self {
        Commands::new(None, None)
    }

    pub fn new(pre_build: Option<Vec<String>>, build: Option<Vec<String>>) -> Self {
        let defaults = Commands::defaults();
        let _pre_build = match pre_build {
            Some(commands) => match commands.len() {
                0 => defaults.pre_build,
                _ => commands,
            },
            None => defaults.pre_build,
        };
        let _build = match build {
            Some(commands) => match commands.len() {
                0 => defaults.build,
                _ => commands,
            },
            None => defaults.build,
        };

        Self {
            pre_build: _pre_build,
            build: _build,
        }
    }

    pub fn defaults() -> Self {
        Self {
            pre_build: vec!["npm install".to_string()],
            build: vec!["npm run build".to_string()],
        }
    }
}
impl AsDynamoDBAttributeValue for Commands {
    fn as_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        map.insert(
            "pre_build".to_string(),
            AttributeValue::L(
                self.pre_build
                    .iter()
                    .map(|command| AttributeValue::S(command.to_owned()))
                    .collect(),
            ),
        );
        map.insert(
            "build".to_string(),
            AttributeValue::L(
                self.build
                    .iter()
                    .map(|command| AttributeValue::S(command.to_owned()))
                    .collect(),
            ),
        );

        map
    }

    fn as_attr(&self) -> AttributeValue {
        AttributeValue::M(self.as_hashmap())
    }
}
