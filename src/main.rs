mod datetime_deserializer;
mod types;
mod utils;

use std::{env, error::Error};

// use chrono::{DateTime, Utc};
// use cmd_lib::{run_cmd, run_fun};
// use serde::Deserialize;
use tokio::fs::create_dir_all;
use types::repositories::Repositories;
use utils::{load_config, repos_base_path};

// #[derive(Deserialize, Debug)]
// struct Commit {
//     hash: String,
//     author_email: String,
//     author_name: String,
//     #[serde(with = "datetime_deserializer")]
//     author_when: DateTime<Utc>,
//     committer_email: String,
//     committer_name: String,
//     #[serde(with = "datetime_deserializer")]
//     committer_when: DateTime<Utc>,
//     message: String,
//     parents: i32,
// }

const HOME: &str = env!("HOME");
const REPOS_BASE_PATH: &str = "/.local/share/brag-server/repos";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config().await?;

    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = sqlx::postgres::PgPoolOptions::new()
    //     .max_connections(20)
    //     .connect(&db_url)
    //     .await
    //     .expect("failed to connect to DATABASE_URL");

    let repos_path = repos_base_path();
    create_dir_all(&repos_path).await?;
    let repositories = Repositories::from(&config.hosts).await?;
    // TODO
    dbg!(repositories);

    // for (url, full_name) in &urls {
    //     let sql = "select * from commits".to_string();
    //     let full_path = repos_path.join(full_name);
    //     if !full_path.is_dir() {
    //         run_cmd!(git clone $url $full_path).unwrap();
    //     }
    //     dbg!(&url);
    //     dbg!(&full_name);
    //     dbg!(&sql);
    //     let json_str = run_fun!(docker run -v $full_path:/repo mergestat/mergestat $sql --format json)
    //         .unwrap();
    //     let commits: Vec<Commit> = serde_json::from_str(&json_str).unwrap();
    //     let insert_qry = r"
    //     INSERT INTO commits (
    //         repo,
    //         hash,
    //         author_email,
    //         author_name,
    //         author_when,
    //         committer_email,
    //         committer_name,
    //         committer_when,
    //         message,
    //         parents
    //     )
    //     VALUES (
    //         $1,
    //         $2,
    //         $3,
    //         $4,
    //         $5,
    //         $6,
    //         $7,
    //         $8,
    //         $9,
    //         $10
    //     )
    //     ";
    //     for commit in &commits {
    //         sqlx::query(insert_qry)
    //             .bind(full_name)
    //             .bind(&commit.hash)
    //             .bind(&commit.author_email)
    //             .bind(&commit.author_name)
    //             .bind(commit.author_when)
    //             .bind(&commit.committer_email)
    //             .bind(&commit.committer_name)
    //             .bind(commit.committer_when)
    //             .bind(&commit.message)
    //             .bind(commit.parents)
    //             .execute(&pool)
    //             .await
    //             .expect("failed to save commit in db");
    //     }
    // }

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
    Ok(())
}
