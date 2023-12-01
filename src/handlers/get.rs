use axum::http::StatusCode;
use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::{PgPool, Row};

use crate::types::repositories::RepoResp;

pub async fn repos(State(pool): State<PgPool>) -> Result<Json<Vec<RepoResp>>, StatusCode> {
    let q = "SELECT DISTINCT repo FROM commits";
    let rows = sqlx::query(q)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        rows.into_iter()
            .map(|r| RepoResp::from_full_name(r.get("repo")))
            .collect(),
    ))
}

#[derive(Serialize)]
pub struct CountResp {
    repo: RepoResp,
    count: i64,
}

pub async fn count(State(pool): State<PgPool>) -> Result<Json<Vec<CountResp>>, StatusCode> {
    let q = r"
        SELECT repo, COUNT(*)
        FROM commits
        GROUP BY repo
    ";
    let rows = sqlx::query(q)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        rows.into_iter()
            .map(|r| CountResp {
                repo: RepoResp::from_full_name(r.get("repo")),
                count: r.get("count"),
            })
            .collect(),
    ))
}
