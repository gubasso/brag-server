use brag_server::{
    queries::insert_commits_to_db,
    types::repositories::Repositories,
    utils::{load_config, repos_base_path},
};
use sqlx::{Pool, Postgres};

use std::{env, error::Error, fmt::Display};
use std::{
    io::{stdout, BufWriter, Write},
    time::Duration,
};
use tokio::{fs::create_dir_all, time::interval};

const DAY_IN_SEC: u64 = 86400;

#[derive(Debug)]
struct UpdateRepoError {
    message: String,
}

impl Display for UpdateRepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UpdateRepoError: {}", self.message)
    }
}

impl Error for UpdateRepoError {}

impl From<Box<dyn Error>> for UpdateRepoError {
    fn from(value: Box<dyn Error>) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

async fn update_repositories(
    pool: &Pool<Postgres>,
    repositories: &mut Repositories,
) -> Result<(), Box<dyn Error>> {
    let mut interval = interval(Duration::from_secs(DAY_IN_SEC));
    loop {
        interval.tick().await;
        println!("# Recurring setting commits");
        repositories.set_all_commits()?;
        println!("# Recurring insert to DB");
        insert_commits_to_db(pool, repositories).await?;
        println!("# Recurring insert to DB finished");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stdout = stdout();
    let mut bw = BufWriter::new(stdout);
    writeln!(bw, "load_db main started")?;
    let repos_path = repos_base_path();
    create_dir_all(&repos_path).await?;
    let config = load_config()
        .await
        .expect("brag-server-config.toml needs to exist.");
    let mut repositories = Repositories::from(&config.hosts).await?;
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    println!("# Inserting commits to DB for the first time");
    insert_commits_to_db(&pool, &repositories).await?;
    let task = tokio::spawn(async move {
        update_repositories(&pool, &mut repositories)
            .await
            .map_err(UpdateRepoError::from)
    });
    task.await??;
    Ok(())
}
