use brag_server::{
    queries::insert_commits_to_db, types::repositories::Repositories, utils::load_config,
};

use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config().await?;
    let repositories = Repositories::from(&config.hosts).await?;
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    insert_commits_to_db(&pool, &repositories).await?;
    Ok(())
}
