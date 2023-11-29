use std::error::Error;

use sqlx::{Pool, Postgres};

use crate::types::repositories::Repositories;

const INSERT_COMMIT_QRY: &str = r"
INSERT INTO commits (
    repo,
    hash,
    author_email,
    author_name,
    author_when,
    committer_email,
    committer_name,
    committer_when,
    message,
    parents
)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10
)";

pub async fn insert_commits_to_db(
    pool: &Pool<Postgres>,
    repositories: &Repositories,
) -> Result<(), Box<dyn Error>> {
    for repo in repositories.iter() {
        for commit in &repo.commits {
            sqlx::query(INSERT_COMMIT_QRY)
                .bind(&repo.user_repo_name)
                .bind(&commit.hash)
                .bind(&commit.author_email)
                .bind(&commit.author_name)
                .bind(commit.author_when)
                .bind(&commit.committer_email)
                .bind(&commit.committer_name)
                .bind(commit.committer_when)
                .bind(&commit.message)
                .bind(commit.parents)
                .execute(pool)
                .await
                .expect("failed to save commit in db");
        }
    }
    Ok(())
}
