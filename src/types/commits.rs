use crate::datetime_deserializer;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Agent {
    pub email: String,
    pub name: String,
    #[serde(rename = "dateISO8601", with = "datetime_deserializer")]
    pub date: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Commit {
    #[serde(rename = "commitHash")]
    pub hash: String,
    pub author: Agent,
    pub committer: Agent,
    #[serde(rename = "subject")]
    pub message: String,
}
