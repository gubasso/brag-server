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
    message
)
SELECT * FROM (
    SELECT
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8,
        $9
) AS tmp
WHERE NOT EXISTS (
    SELECT 1 FROM commits WHERE
        repo = $1 AND
        hash = $2
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
                .bind(&commit.author.email)
                .bind(&commit.author.name)
                .bind(commit.author.date)
                .bind(&commit.committer.email)
                .bind(&commit.committer.name)
                .bind(commit.committer.date)
                .bind(&commit.message)
                .execute(pool)
                .await
                .expect("failed to save commit in db");
        }
    }
    Ok(())
}
