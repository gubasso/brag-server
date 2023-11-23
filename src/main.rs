use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use cmd_lib::run_cmd;
use serde::Deserialize;
use tokio::fs::create_dir_all;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Repository {
    url: String,
    user: String,
    branch: String,
    author_email: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Config {
    repositories: Vec<Repository>,
}

fn get_repo_name(url: &str) -> String {
    let strs = url.split('/');
    strs.last().unwrap().to_owned()
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
    dbg!(&toml);
    for repo in &toml.repositories {
        let url = &repo.url;
        let name = get_repo_name(url);
        let repo_path = repos_path.join(name);
        run_cmd!(git clone $url $repo_path).unwrap();
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
