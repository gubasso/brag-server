use std::{error::Error, path::PathBuf};

use reqwest::{header::USER_AGENT, Client};
use serde_json::Value;
use url::Url;

use crate::{utils::repos_base_path, CARGO_PKG_VERSION};

use super::git_hosts::{GitHost, Host};

#[derive(Debug)]
#[allow(unused)]
pub struct Repo {
    url: Url,
    user: String,
    name: String,
    host: GitHost,
    path: PathBuf,
}

fn get_value(obj: &Value, key: &str) -> Result<String, Box<dyn Error>> {
    Ok(obj.get(key).ok_or(key)?.as_str().ok_or(key)?.to_owned())
}

impl Repo {
    pub fn from(host: &Host, obj: &Value) -> Result<Self, Box<dyn Error>> {
        let name = get_value(obj, &host.host.repo_name_key())?;
        let url_str = get_value(obj, &host.host.url_key())?;
        let url = Url::parse(&url_str)?;
        let user_repo_name = get_value(obj, &host.host.user_repo_name_key())?;
        let mut path = repos_base_path();
        path.push(user_repo_name);
        Ok(Self {
            host: host.host,
            user: host.user.clone(),
            name,
            url,
            path,
        })
    }
}

#[derive(Debug)]
pub struct Repositories(Vec<Repo>);

impl Repositories {
    pub async fn from(hosts: &Vec<Host>) -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let mut repos = vec![];
        for host in hosts {
            let api_repos_url = host.host.api_repos_url(&host.user);
            let json_str = client
                .get(api_repos_url)
                .header(USER_AGENT, format!("brag-server/{}", CARGO_PKG_VERSION))
                .send()
                .await?
                .text()
                .await?;
            let json_repos: Vec<Value> = serde_json::from_str(&json_str)?;
            for jrepo in &json_repos {
                repos.push(Repo::from(host, jrepo)?);
            }
        }
        Ok(Self(repos))
    }
}
