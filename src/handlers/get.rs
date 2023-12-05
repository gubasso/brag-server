use std::str::FromStr;

use axum::extract::Query;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use validator::Validate;

use crate::types::git_hosts::{deserializer_optional_enum, EnumParseError};
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
    query: Option<QueryFilterCount>,
    date_agg: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
enum Interval {
    Day,
    Week,
    Month,
}

impl FromStr for Interval {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "day" => Ok(Interval::Day),
            "week" => Ok(Interval::Week),
            "month" => Ok(Interval::Month),
            _ => Err(EnumParseError {
                value: s.to_string(),
            }),
        }
    }
}

#[derive(Debug, Deserialize, Validate, Serialize, Clone)]
pub struct QueryFilterCount {
    #[serde(deserialize_with = "deserializer_optional_enum")]
    by: Option<Interval>,
    #[validate(email)]
    author_email: Option<String>,
    repo: Option<String>,
}

fn where_filter(filters: Vec<(&str, &Option<String>)>) -> Option<String> {
    if filters.iter().all(|(_, filter)| filter.is_none()) {
        return None;
    }
    let mut where_str = r"
        WHERE"
        .to_string();
    for (field, filter) in filters.iter() {
        if let Some(f) = filter {
            where_str.push_str(&format!(
                r"
                {} = '{}' AND",
                field, f
            ));
        }
    }
    where_str.truncate(where_str.len() - 3);
    dbg!(&where_str);
    Some(where_str)
}

fn count_qry_builder(qs_opt: &Option<QueryFilterCount>) -> Result<String, StatusCode> {
    let mut q = r"
        SELECT
            repo,
            COUNT(*)
    "
    .to_string();
    let mut group_by = r"
        GROUP BY
            repo
    "
    .to_string();
    let from_commits = r"
        FROM
            commits";
    if qs_opt.is_none() {
        q.push_str(from_commits);
        q.push_str(&group_by);
        return Ok(q);
    }
    let qs = qs_opt.as_ref().unwrap();
    qs.validate()
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;
    if let Some(interval) = &qs.by {
        let date_agg_str = format!(
            r",
            DATE_TRUNC('{:?}', author_when) AS date_agg",
            interval
        );
        q.push_str(&date_agg_str);
        group_by.push_str(
            r",
            date_agg",
        );
    }
    q.push_str(from_commits);
    let where_vec = vec![("repo", &qs.repo), ("author_email", &qs.author_email)];
    let where_opt = where_filter(where_vec);
    if let Some(where_qry) = where_opt {
        q.push_str(&where_qry);
    }
    q.push_str(&group_by);
    dbg!(&q);
    Ok(q)
}

pub async fn count(
    State(pool): State<PgPool>,
    qry_opt: Option<Query<QueryFilterCount>>,
) -> Result<Json<Vec<CountResp>>, StatusCode> {
    let qry_filter_opt = qry_opt.map(|q| q.0);
    let rows = sqlx::query(&count_qry_builder(&qry_filter_opt)?)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        rows.into_iter()
            .map(|r| {
                let date_agg = r.try_get("date_agg").ok();
                CountResp {
                    repo: RepoResp::from_full_name(r.get("repo")),
                    count: r.get("count"),
                    query: qry_filter_opt.clone(),
                    date_agg,
                }
            })
            .collect(),
    ))
}
