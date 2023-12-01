use std::{env, error::Error, net::SocketAddr};

use axum::{routing::get, Router, Server};
use brag_server::handlers::get::{count, repos};
use brag_server::utils::repos_base_path;
use tokio::fs::create_dir_all;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    let repos_path = repos_base_path();
    create_dir_all(&repos_path).await?;
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(repos))
        .route("/count", get(count))
        .with_state(pool)
        .layer(cors);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("server is up");
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
