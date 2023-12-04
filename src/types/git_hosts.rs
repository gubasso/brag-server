use core::fmt;
use serde::{de::Error, Deserialize, Deserializer};
use std::{fmt::Display, str::FromStr};
use url::Url;

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum GitHost {
    Github,
    Gitlab,
}

impl GitHost {
    pub fn api_url(&self) -> Url {
        use GitHost::*;
        let url_str = match self {
            Github => "https://api.github.com/users/",
            Gitlab => "https://gitlab.com/api/v4/users/",
        };
        Url::parse(url_str).expect("Git host api url must be right.")
    }
    pub fn api_repos_url(&self, user: &str) -> Url {
        use GitHost::*;
        let endpoint_str = match self {
            Github => "repos",
            Gitlab => "projects",
        };
        let user_str = format!("{}/", user);
        let user_endpoint = self.api_url().join(&user_str).unwrap();
        user_endpoint.join(endpoint_str).unwrap()
    }
    pub fn repo_name_key(&self) -> String {
        "name".to_owned()
    }
    pub fn url_key(&self) -> String {
        use GitHost::*;
        match self {
            Github => "clone_url".to_owned(),
            Gitlab => "http_url_to_repo".to_owned(),
        }
    }
    pub fn user_repo_name_key(&self) -> String {
        use GitHost::*;
        match self {
            Github => "full_name".to_owned(),
            Gitlab => "path_with_namespace".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct EnumParseError {
    pub value: String,
}

impl fmt::Display for EnumParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid host: {}", self.value)
    }
}

impl FromStr for GitHost {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(GitHost::Github),
            "gitlab" => Ok(GitHost::Gitlab),
            _ => Err(EnumParseError {
                value: s.to_string(),
            }),
        }
    }
}

pub fn deserializer_enum<'de, D, E>(deserializer: D) -> Result<E, D::Error>
where
    D: Deserializer<'de>,
    E: FromStr + Deserialize<'de>,
    <E as FromStr>::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    E::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn deserializer_optional_enum<'de, D, E>(deserializer: D) -> Result<Option<E>, D::Error>
where
    D: Deserializer<'de>,
    E: FromStr + Deserialize<'de>,
    <E as FromStr>::Err: Display,
{
    let option = Option::<String>::deserialize(deserializer)?;
    option.map_or(Ok(None), |s| {
        E::from_str(&s).map(Some).map_err(Error::custom)
    })
}

#[derive(Deserialize, Debug)]
pub struct Host {
    #[serde(deserialize_with = "deserializer_enum")]
    pub host: GitHost,
    pub user: String,
}
