use std::{fs::read_to_string, path::PathBuf};

use serde::Deserialize;

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

#[tokio::main]
async fn main() {
    let fpath = PathBuf::from("samples/brag-server.toml");
    let str = read_to_string(fpath).unwrap();
    let toml: Config = toml::from_str(&str).unwrap();
    dbg!(toml);

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
