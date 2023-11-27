use core::fmt;
use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
    str::FromStr,
};

use cmd_lib::{run_cmd, run_fun};
use reqwest::{header::USER_AGENT, Client};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use tokio::fs::create_dir_all;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Host {
    #[serde(deserialize_with = "deserialize_githost")]
    host: GitHosts,
    user: String,
}

#[derive(Deserialize, Debug)]
enum GitHosts {
    Github,
    Gitlab,
}

#[derive(Debug)]
struct GitHostsParseError {
    value: String,
}

impl fmt::Display for GitHostsParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid host: {}", self.value)
    }
}

impl FromStr for GitHosts {
    type Err = GitHostsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(GitHosts::Github),
            "gitlab" => Ok(GitHosts::Gitlab),
            _ => Err(GitHostsParseError {
                value: s.to_string(),
            }),
        }
    }
}

fn deserialize_githost<'de, D>(deserializer: D) -> Result<GitHosts, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    GitHosts::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Config {
    branches: Vec<String>,
    author_emails: Vec<String>,
    hosts: Vec<Host>,
}

async fn get_repos_url(host: &GitHosts, user: &str) -> Vec<(String, String)> {
    let req_url = match host {
        GitHosts::Github => format!("https://api.github.com/users/{}/repos", user),
        GitHosts::Gitlab => format!("https://gitlab.com/api/v4/users/{}/projects", user),
    };
    let version = env!("CARGO_PKG_VERSION");
    let client = Client::new();
    let json_str = client
        .get(req_url)
        .header(USER_AGENT, format!("brag-server/{}", version))
        .send()
        .await
        .expect("req fail")
        .text()
        .await
        .expect("parse body error");
    let json: Value = serde_json::from_str(&json_str).expect("fail parse json");
    let mut urls = vec![];
    match host {
        GitHosts::Github => {
            for repo in json.as_array().unwrap() {
                let tup = get_repo_url_n_full_name(repo, "full_name", "clone_url");
                urls.push(tup);
            }
        }
        GitHosts::Gitlab => {
            for repo in json.as_array().unwrap() {
                let tup = get_repo_url_n_full_name(repo, "path_with_namespace", "http_url_to_repo");
                urls.push(tup);
            }
        }
    };
    urls
}

fn get_repo_url_n_full_name(
    repo_obj: &Value,
    key_full_name: &str,
    key_url: &str,
) -> (String, String) {
    let repo_obj = repo_obj.as_object().unwrap();
    let full_name = repo_obj
        .get(key_full_name)
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    let url = repo_obj.get(key_url).unwrap().as_str().unwrap().to_owned();
    (url, full_name)
}

#[tokio::main]
async fn main() {
    let home = env::var("HOME").expect("HOME env var must be set");
    let repos_base = format!("{}/.local/share/brag-server/repos", home);
    let repos_path = Path::new(&repos_base);
    create_dir_all(repos_path)
        .await
        .expect("Unable to create repos base dir");
    let fpath = PathBuf::from("samples/brag-server.toml");
    let str = read_to_string(fpath).unwrap();
    let toml: Config = toml::from_str(&str).unwrap();
    let mut urls = vec![];
    for acc in &toml.hosts {
        let host_urls = get_repos_url(&acc.host, &acc.user).await;
        urls.extend(host_urls);
    }
    for (url, full_name) in &urls {
        let mut sql = "select count(*) from commits ".to_string();
        let full_path = repos_path.join(full_name);
        if !full_path.is_dir() {
            run_cmd!(git clone $url $full_path).unwrap();
        }
        if !toml.author_emails.is_empty() {
            let mut where_clause = "WHERE author_email IN (".to_string();
            for email in &toml.author_emails {
                where_clause.push_str(&format!("'{}',", email));
            }
            where_clause.pop();
            where_clause.push(')');
            sql.push_str(&where_clause);
        }
        dbg!(&sql);
        let j = run_fun!(docker run -v $full_path:/repo mergestat/mergestat $sql --format json)
            .unwrap();
        let js: Value = serde_json::from_str(&j).unwrap();
        dbg!(js);
    }

    // let app = Router::new()
    //     .route("/", get(handler))
    //     .nest("/error", api_error());
    // let mut listenfd = ListenFd::from_env();
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // let server = match listenfd.take_tcp_listener(0).unwrap() {
    //     Some(listener) => Server::from_tcp(listener).unwrap(),
    //     None => Server::bind(&addr),
    // };
    // println!("listening on {}", addr);
    // server.serve(app.into_make_service()).await.unwrap();
}
