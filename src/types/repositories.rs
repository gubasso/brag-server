use std::{
    error::Error,
    path::{Path, PathBuf},
    slice::Iter,
};

use cmd_lib::{run_cmd, run_fun};
use reqwest::{header::USER_AGENT, Client};
use serde::Serialize;
use serde_json::Value;
use url::Url;

use crate::{global_vars::CARGO_PKG_VERSION, utils::repos_base_path};

use super::{
    commits::Commit,
    git_hosts::{GitHost, Host},
};

#[derive(Debug)]
#[allow(unused)]
pub struct Repo {
    pub url: Url,
    user: String,
    name: String,
    host: GitHost,
    path: PathBuf,
    pub commits: Vec<Commit>,
    pub user_repo_name: String,
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
        path.push(&user_repo_name);
        Ok(Self {
            host: host.host,
            user: host.user.clone(),
            name,
            commits: get_commits(&path, &url)?,
            url,
            path,
            user_repo_name,
        })
    }
}

fn get_commits(path: &Path, url: &Url) -> Result<Vec<Commit>, Box<dyn Error>> {
    if !path.is_dir() {
        let url = url.as_str();
        let path = path.to_str().unwrap();
        run_cmd!(git clone $url $path)?;
    }
    let path = path.to_str().expect("repo-commits-path");
    let sql = "select * from commits".to_string();
    let json_str = run_fun!(docker run -v $path:/repo mergestat/mergestat $sql --format json)?;
    let commits: Vec<Commit> = serde_json::from_str(&json_str)?;
    Ok(commits)
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
    pub fn iter(&self) -> Iter<'_, Repo> {
        self.0.iter()
    }
}

#[derive(Serialize)]
pub struct RepoResp {
    name: String,
    user: String,
    full_name: String,
}

impl RepoResp {
    pub fn from_full_name(full_name: &str) -> Self {
        let v: Vec<&str> = full_name.split('/').collect();
        let full_name = full_name.to_owned();
        let user = v.first().unwrap_or(&"").to_string();
        let name = v.get(1).unwrap_or(&"").to_string();
        Self {
            full_name,
            user,
            name,
        }
    }
}
