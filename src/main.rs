use std::{env, error::Error, net::SocketAddr};

use axum::{routing::get, Router, Server};
use brag_server::handlers::get::{count, repos};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/count", get(count))
        .route("/", get(repos))
        .with_state(pool)
        .layer(cors);
    let addr_str = format!("{}:{}", env::var("NET_HOST")?, env::var("API_PORT")?);
    let addr: SocketAddr = addr_str.parse()?;
    println!("server is up");
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
