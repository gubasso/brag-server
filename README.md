# brag-server

<!-- toc -->

- [Prerequisites](#prerequisites)

<!-- tocstop -->

The value returned for each day is a sum of the:

- Number of commits in all registered repositories for the selected git contributors

## Prerequisites

- needs to be authenticated (e.g. ssh-agent) if is needed for git clone. This program will not handle this.


I have a binary that loads data to a db, using postgresql, rust, sqlx...

the table:

```sql
CREATE TABLE IF NOT EXISTS commits (
    repo            	 VARCHAR(64) NOT NULL,
    hash            	 VARCHAR(40) NOT NULL,
    author_email    	 VARCHAR(64) NOT NULL,
    author_name     	 VARCHAR(64) NOT NULL,
    author_when     	 TIMESTAMPTZ DEFAULT NULL,
    committer_email 	 VARCHAR(64) DEFAULT NULL,
    committer_name  	 VARCHAR(64) DEFAULT NULL,
    committer_when  	 TIMESTAMPTZ DEFAULT NULL,
    message         	 TEXT,
);
```

the main function:

```rs

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let repos_path = repos_base_path();
    create_dir_all(&repos_path).await?;
    let config = load_config()
        .await
        .expect("brag-server.toml needs to exist.");
    let mut repositories = Repositories::from(&config.hosts).await?;
    repositories.pull_all()?;
    repositories.set_all_commits()?;
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    println!("# Inserting commits to DB for the first time");
    insert_commits_to_db(&pool, &repositories).await?;
    # continues....
```

insert_commits_to_db function:

```rs

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
    println!("(insert_commits_to_db)");
    for repo in repositories.iter() {
        println!("repo: {:?}", repo);
        for commit in &repo.commits {
            println!("commit: {:?}", commit);
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

```

When I run this code, I'm getting the following error:

```
thread 'main' panicked at /home/gubasso/Projects/cwnt-gubasso.group/brag-server/src/queries/mod.rs:58:18:
failed to save commit in db: Database(PgDatabaseError { severity: Error, code: "42601", message: "syntax error at or near \")\"", detail: None, hint: None, position: Some(Original(170)), where: None, schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("scan.l"), line: Some(1241), routine: Some("scanner_yyerror") })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

what's going on?
