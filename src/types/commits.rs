use crate::datetime_deserializer;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub author_email: String,
    pub author_name: String,
    #[serde(with = "datetime_deserializer")]
    pub author_when: DateTime<Utc>,
    pub committer_email: String,
    pub committer_name: String,
    #[serde(with = "datetime_deserializer")]
    pub committer_when: DateTime<Utc>,
    pub message: String,
    pub parents: i32,
}
