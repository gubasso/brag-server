-- Add down migration script here
DROP INDEX IF EXISTS idx_commits_repo_authoremail;
DROP TABLE IF EXISTS commits;
