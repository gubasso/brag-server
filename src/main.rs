mod datetime_deserializer;
mod queries;
mod types;
mod utils;

use std::{env, error::Error, net::SocketAddr};

use axum::{routing::get, Router, Server};
use queries::insert_commits_to_db;
use tokio::fs::create_dir_all;
use tower_http::cors::{Any, CorsLayer};
use types::repositories::Repositories;
use utils::{load_config, repos_base_path};

const HOME: &str = env!("HOME");
const REPOS_BASE_PATH: &str = "/.local/share/brag-server/repos";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

async fn root() -> &'static str {
    "Hello, World!!!!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config().await?;

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");

    let repos_path = repos_base_path();
    create_dir_all(&repos_path).await?;
    let repositories = Repositories::from(&config.hosts).await?;
    insert_commits_to_db(&pool, &repositories).await?;

    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .with_state(pool)
        .layer(cors);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
